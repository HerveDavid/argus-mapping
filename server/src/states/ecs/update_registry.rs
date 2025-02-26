use crate::{
    handlers::{RegisterRequest, RegisterResponse, UpdateError},
    states::AppState,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bevy_ecs::{component::Component, event::Events};
use iidm::{AssetRegistry, EntityNotFoundEvent, ErrorType, JsonSchema, Updatable, UpdateEvent};
use std::fmt::Display;
use std::future::Future;
use std::sync::Arc;
use std::{collections::HashMap, pin::Pin};

// Type-erased function to dispatch to update_iidm with correct types
type UpdateHandlerFn = Box<
    dyn Fn(
            State<Arc<AppState>>,
            Json<RegisterRequest>,
        ) -> Pin<Box<dyn Future<Output = Result<Response, UpdateError>> + Send>>
        + Send
        + Sync,
>;

// Registry to store handlers by component name
#[derive(Default)]
pub struct UpdateRegistry {
    handlers: HashMap<String, UpdateHandlerFn>,
}

impl UpdateRegistry {
    pub fn register<C, U, E>(&mut self, type_name: &str)
    where
        C: Updatable<Updater = U> + Component + 'static,
        U: JsonSchema + Send + Sync + 'static,
        U::Err: Display,
    {
        let handler = Box::new(
            move |state: State<Arc<AppState>>, payload: Json<RegisterRequest>| {
                Box::pin(async move {
                    // Call update_iidm and convert the result to Response
                    match update_iidm::<C, U, E>(state, payload).await {
                        Ok(response) => Ok(response.into_response()),
                        Err(err) => Err(err),
                    }
                })
                    as Pin<Box<dyn Future<Output = Result<Response, UpdateError>> + Send>>
            },
        );

        self.handlers.insert(type_name.to_string(), handler);
        tracing::debug!("Registered update handler for {}", type_name);
    }

    pub fn get_handler(&self, component_type: &str) -> Option<&UpdateHandlerFn> {
        self.handlers.get(component_type)
    }
}

async fn update_iidm<C, U, E>(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, UpdateError>
where
    C: Updatable<Updater = U> + Component + 'static,
    U: JsonSchema + Send + Sync + 'static,
    U::Err: Display,
{
    tracing::debug!("Received update request for component ID: {}", payload.id);

    update_component::<C, U, E>(&state, &payload).await?;

    Ok((
        StatusCode::OK,
        Json(RegisterResponse {
            status: "Component updated successfully".to_string(),
        }),
    ))
}

async fn update_component<C, U, E>(
    state: &Arc<AppState>,
    payload: &RegisterRequest,
) -> Result<(), UpdateError>
where
    C: Updatable<Updater = U> + Component + 'static,
    U: JsonSchema + Send + Sync + 'static,
    U::Err: Display,
{
    // Acquire locks and prepare state
    let ecs = state.ecs.read().await;
    let mut world = ecs.world.write().await;
    let mut schedule = ecs.schedule.write().await;
    let id = payload.id.clone();

    // Verify required resources exist
    verify_resources::<C>(&world)?;

    // Parse and validate the JSON
    let json_str = serde_json::to_string(&payload.component)?;
    let update = parse_and_validate_json::<U>(&json_str)?;

    // Process the update
    process_update::<C, U, E>(&mut world, &mut schedule, &id, update)?;

    // TODO: Update with SSE events, very dirty code
    let sse_registry = ecs.sse_registry.read().await;
    let asset_registry = world.resource::<AssetRegistry>();
    let component_type = std::any::type_name::<C>(); // Ou extraire le nom du type d'une autre façon

    {
        if let Some(entity) = asset_registry.find(&id) {
            // Si l'entité existe, récupérer le composant
            if let Some(component) = world.entity(entity).get::<C>() {
                // Sérialiser le composant complet
                if let Ok(component_json) = serde_json::to_string(component) {
                    // Envoyer le composant complet via SSE
                    sse_registry.publish_update(component_type, &id, &component_json);
                }
            }
        }
    }

    tracing::debug!("Successfully updated component: {}", id);
    Ok(())
}

// Helper function to verify resources
fn verify_resources<C>(world: &bevy_ecs::world::World) -> Result<(), UpdateError>
where
    C: Updatable + 'static,
{
    // Check for UpdateEvent resource
    if !world.contains_resource::<Events<UpdateEvent<C>>>() {
        tracing::error!(
            "Events<UpdateEvent<{}>> not initialized",
            std::any::type_name::<C>()
        );
        return Err(UpdateError::InternalError(format!(
            "Event system for {} not initialized",
            std::any::type_name::<C>()
        )));
    }

    // Check for EntityNotFoundEvent resource
    if !world.contains_resource::<Events<EntityNotFoundEvent>>() {
        tracing::error!("Events<EntityNotFoundEvent> not initialized");
        return Err(UpdateError::InternalError(
            "Error errors system not initialized".to_string(),
        ));
    }

    Ok(())
}

// Helper function to parse and validate JSON
fn parse_and_validate_json<U>(json_str: &str) -> Result<U, UpdateError>
where
    U: JsonSchema + Send + Sync,
    U::Err: Display,
{
    U::validate_json(json_str).map_err(|e| UpdateError::ValidationError(e.to_string()))
}

// Helper function to process the update
fn process_update<C, U, E>(
    world: &mut bevy_ecs::world::World,
    schedule: &mut bevy_ecs::schedule::Schedule,
    id: &str,
    update: U,
) -> Result<(), UpdateError>
where
    C: Updatable<Updater = U> + 'static,
    U: Send + Sync + 'static,
{
    // Get event writer
    let mut event_writer = world
        .get_resource_mut::<Events<UpdateEvent<C>>>()
        .ok_or_else(|| UpdateError::InternalError("Event system not initialized".to_string()))?;

    // Send update event
    event_writer.send(UpdateEvent {
        id: id.to_string(),
        updater: update,
    });

    // Run the schedule to process the event
    schedule.run(world);

    // Check for errors
    check_for_errors(world, id)
}

// Helper function to check for errors after update
fn check_for_errors(world: &bevy_ecs::world::World, id: &str) -> Result<(), UpdateError> {
    let error_events = world.resource::<Events<EntityNotFoundEvent>>();
    let mut error_reader = error_events.get_cursor();

    for error in error_reader.read(error_events) {
        if error.id == id {
            match error.error_type {
                ErrorType::EntityNotFound => {
                    return Err(UpdateError::NotFoundError(format!(
                        "Entity with ID '{}' not found",
                        id
                    )));
                }
                ErrorType::ComponentNotFound => {
                    return Err(UpdateError::NotFoundError(format!(
                        "Component of type '{}' not found on entity with ID '{}'",
                        error.component_type, id
                    )));
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::world::World;
    use serde::{Deserialize, Serialize};

    // Mock types for testing
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct MockComponent;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct MockUpdater;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct MockError;

    impl Display for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Mock error")
        }
    }

    impl Updatable for MockComponent {
        type Updater = MockUpdater;

        fn update(&mut self, _updates: Self::Updater) {
            // Mock implementation
        }
    }

    impl JsonSchema for MockUpdater {
        type Err = String;

        fn validate_json(json: &str) -> Result<Self, Self::Err> {
            if json.contains("valid") {
                Ok(MockUpdater)
            } else {
                Err("Invalid JSON".to_string())
            }
        }

        fn fields_json() -> Vec<String> {
            vec![]
        }
    }

    #[test]
    fn test_verify_resources() {
        let mut world = World::new();

        // Test with missing resources
        let result = verify_resources::<MockComponent>(&world);
        assert!(result.is_err());

        // Add required resources
        world.insert_resource(Events::<UpdateEvent<MockComponent>>::default());
        world.insert_resource(Events::<EntityNotFoundEvent>::default());

        // Test with all resources present
        let result = verify_resources::<MockComponent>(&world);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_for_errors() {
        let mut world = World::new();
        world.insert_resource(Events::<EntityNotFoundEvent>::default());

        // Test with no errors
        let result = check_for_errors(&world, "test_id");
        assert!(result.is_ok());

        // Add an error event
        let mut error_events = world.resource_mut::<Events<EntityNotFoundEvent>>();
        error_events.send(EntityNotFoundEvent {
            id: "test_id".to_string(),
            error_type: ErrorType::EntityNotFound,
            component_type: "MockComponent".to_string(),
        });

        // Test with an error
        let result = check_for_errors(&world, "test_id");
        assert!(result.is_err());
    }
}
