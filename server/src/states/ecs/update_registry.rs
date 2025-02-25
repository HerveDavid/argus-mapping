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
use bevy_ecs::event::Events;
use iidm::{EntityNotFoundEvent, ErrorType, JsonSchema, Updatable, UpdateEvent};
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
        C: Updatable<Updater = U, Err = E> + 'static,
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
    C: Updatable<Updater = U, Err = E> + 'static,
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
    C: Updatable<Updater = U, Err = E> + 'static,
    U: JsonSchema + Send + Sync + 'static,
    U::Err: Display,
{
    let ecs = state.ecs.read().await;

    let mut world = ecs.world.write().await;
    let mut schedule = ecs.schedule.write().await;
    let id = payload.id.clone();

    // Critical verifications
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

    if !world.contains_resource::<Events<EntityNotFoundEvent>>() {
        tracing::error!("Events<EntityNotFoundEvent> not initialized");
        return Err(UpdateError::InternalError(
            "Error errors system not initialized".to_string(),
        ));
    }

    // Convert the component JSON to a string
    let json_str = serde_json::to_string(&payload.component)?;

    // Validate the JSON against the component schema
    let update =
        U::validate_json(&json_str).map_err(|e| UpdateError::ValidationError(e.to_string()))?;

    // Send the update event
    match world.get_resource_mut::<Events<UpdateEvent<C>>>() {
        Some(mut event_writer) => {
            event_writer.send(UpdateEvent {
                id: id.clone(),
                update,
            });

            // Run the schedule to process the event
            schedule.run(&mut world);

            // Check if any error events were generated
            let error_events = world.resource_mut::<Events<EntityNotFoundEvent>>();
            let mut error_reader = error_events.get_cursor();

            for error in error_reader.read(&error_events) {
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

            tracing::debug!("Successfully updated component: {}", id);
            Ok(())
        }
        None => {
            tracing::error!("Events resource not found");
            Err(UpdateError::InternalError(
                "Event system not initialized".to_string(),
            ))
        }
    }
}
