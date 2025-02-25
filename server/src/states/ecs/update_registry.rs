use crate::{
    handlers::{update_iidm, RegisterRequest, UpdateError},
    states::AppState,
};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use iidm::{JsonSchema, Updatable};
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
        tracing::info!("Registered update handler for {}", type_name);
    }

    pub fn get_handler(&self, component_type: &str) -> Option<&UpdateHandlerFn> {
        self.handlers.get(component_type)
    }
}

// Dispatcher function
pub async fn update_iidm_by_component(
    Path(component_type): Path<String>,
    state: State<Arc<AppState>>,
    payload: Json<RegisterRequest>,
) -> Result<Response, UpdateError> {
    let ecs = state.ecs.read().await;
    let update_registry = ecs.update_registry.read().await;

    // Get the appropriate handler
    let handler = update_registry
        .get_handler(&component_type)
        .ok_or_else(|| {
            UpdateError::NotFoundError(format!(
                "No handler registered for component type: {}",
                component_type
            ))
        })?;

    // Call the handler with the original state and payload
    handler(state.clone(), payload).await
}
