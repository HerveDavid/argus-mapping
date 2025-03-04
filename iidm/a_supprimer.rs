use enum_dispatch::enum_dispatch;

// Définir un trait pour les opérations communes sur les composants
#[enum_dispatch]
pub trait ComponentOps {
    fn id(&self) -> &str;
    fn update_from_json(&mut self, json: &str) -> Result<(), UpdateError>;
    fn to_json(&self) -> Result<String, serde_json::Error>;
}

// Implémenter ce trait pour tous vos types de composants
impl ComponentOps for Network {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn update_from_json(&mut self, json: &str) -> Result<(), UpdateError> {
        self.update_from_json(json).map_err(|e| UpdateError::ValidationError(e.to_string()))
    }
    
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

// Implémentations similaires pour tous les autres composants...

// Définir l'enum qui dispatche au trait
#[enum_dispatch(ComponentOps)]
pub enum Component {
    Network,
    Line,
    Substation,
    VoltageLevel,
    Generator,
    Load,
    Bus,
    BusbarSection,
    TwoWindingsTransformer,
    ThreeWindingsTransformer,
    Switch,
    ShuntCompensator,
    StaticVarCompensator,
    DanglingLine,
    TieLine,
    HvdcLine,
    HvdcConverterStation,
    TerminalRef,
    // ... autres composants
}

// Fonction de conversion de string vers ComponentType
impl Component {
    pub fn from_type_and_json(type_name: &str, json: &str) -> Result<Self, UpdateError> {
        match type_name {
            "Network" => {
                let component: Network = serde_json::from_str(json)?;
                Ok(Component::Network(component))
            },
            "Line" => {
                let component: Line = serde_json::from_str(json)?;
                Ok(Component::Line(component))
            },
            // ... cas pour tous les autres types
            _ => Err(UpdateError::ValidationError(format!("Unknown component type: {}", type_name)))
        }
    }
    
    pub fn type_name(&self) -> &'static str {
        match self {
            Component::Network(_) => "Network",
            Component::Line(_) => "Line",
            Component::Substation(_) => "Substation",
            // ... cas pour tous les autres types
        }
    }
}

// Handler Axum pour les mises à jour
async fn update_component_handler(
    State(state): State<Arc<AppState>>,
    Path(component_type): Path<String>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Response, UpdateError> {
    // Extraire les données nécessaires
    let ecs = state.ecs.read().await;
    let mut world = ecs.world.write().await;
    let mut schedule = ecs.schedule.write().await;
    let id = payload.id.clone();
    
    // Trouver l'entité
    let asset_registry = world.resource::<AssetRegistry>();
    let entity = asset_registry.find(&id)
        .ok_or_else(|| UpdateError::NotFoundError(format!("Entity with ID '{}' not found", id)))?;
    
    // Créer le composant à partir du JSON
    let json_str = serde_json::to_string(&payload.component)?;
    let mut component = Component::from_type_and_json(&component_type, &json_str)?;
    
    // Mettre à jour le composant existant
    component.update_from_json(&json_str)?;
    
    // Mettre à jour l'entité dans le monde
    commands.entity(entity).insert(component);
    
    // Exécuter le schedule pour traiter la mise à jour
    schedule.run(&mut world);
    
    // Publier les mises à jour SSE
    let sse_registry = ecs.sse_registry.read().await;
    if let Some(entity) = asset_registry.find(&id) {
        if let Some(component) = world.entity(entity).get::<Component>() {
            if let Ok(component_json) = component.to_json() {
                sse_registry.publish_update(component.type_name(), &id, &component_json);
            }
        }
    }
    
    Ok((
        StatusCode::OK,
        Json(RegisterResponse {
            status: "Component updated successfully".to_string(),
        })
    ).into_response())
}
