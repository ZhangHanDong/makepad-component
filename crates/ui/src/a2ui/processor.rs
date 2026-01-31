//! A2UI Message Processor
//!
//! Processes incoming A2UI messages and updates the component tree and data model.

use std::collections::HashMap;

use super::{
    data_model::{DataModel, SurfaceDataModels},
    message::*,
    registry::ComponentRegistry,
    value::{BooleanValue, NumberValue, StringValue},
};

/// Represents a UI surface with its component tree and configuration.
#[derive(Debug, Clone)]
pub struct Surface {
    /// Surface ID
    pub id: String,

    /// Root component ID
    pub root: String,

    /// Style configuration
    pub styles: Option<SurfaceStyles>,

    /// Component definitions by ID
    pub components: HashMap<String, ComponentDefinition>,

    /// Whether the surface needs to be redrawn
    pub needs_redraw: bool,
}

impl Surface {
    /// Create a new surface
    pub fn new(id: String, root: String, styles: Option<SurfaceStyles>) -> Self {
        Surface {
            id,
            root,
            styles,
            components: HashMap::new(),
            needs_redraw: true,
        }
    }

    /// Get a component by ID
    pub fn get_component(&self, id: &str) -> Option<&ComponentDefinition> {
        self.components.get(id)
    }

    /// Get all component IDs
    pub fn component_ids(&self) -> impl Iterator<Item = &String> {
        self.components.keys()
    }

    /// Mark the surface as needing redraw
    pub fn mark_dirty(&mut self) {
        self.needs_redraw = true;
    }

    /// Clear the dirty flag
    pub fn clear_dirty(&mut self) {
        self.needs_redraw = false;
    }
}

/// Event emitted when a surface is created
#[derive(Debug, Clone)]
pub struct SurfaceCreatedEvent {
    pub surface_id: String,
}

/// Event emitted when a surface is updated
#[derive(Debug, Clone)]
pub struct SurfaceUpdatedEvent {
    pub surface_id: String,
    pub updated_components: Vec<String>,
}

/// Event emitted when a surface is deleted
#[derive(Debug, Clone)]
pub struct SurfaceDeletedEvent {
    pub surface_id: String,
}

/// Event emitted when data model is updated
#[derive(Debug, Clone)]
pub struct DataModelUpdatedEvent {
    pub surface_id: String,
    pub updated_paths: Vec<String>,
}

/// Events that can be emitted by the processor
#[derive(Debug, Clone)]
pub enum ProcessorEvent {
    SurfaceCreated(SurfaceCreatedEvent),
    SurfaceUpdated(SurfaceUpdatedEvent),
    SurfaceDeleted(SurfaceDeletedEvent),
    DataModelUpdated(DataModelUpdatedEvent),
}

/// The A2UI message processor.
///
/// Manages surfaces, component trees, and data models.
/// Processes incoming A2UI messages and emits events for UI updates.
///
/// # Example
///
/// ```rust,ignore
/// let registry = ComponentRegistry::with_standard_catalog();
/// let mut processor = A2uiMessageProcessor::new(registry);
///
/// // Process a message
/// let json = r#"{"beginRendering": {"surfaceId": "main", "root": "root"}}"#;
/// let message: A2uiMessage = serde_json::from_str(json)?;
/// let events = processor.process_message(message);
///
/// // Handle events
/// for event in events {
///     match event {
///         ProcessorEvent::SurfaceCreated(e) => {
///             println!("Surface created: {}", e.surface_id);
///         }
///         // ...
///     }
/// }
/// ```
#[derive(Debug)]
pub struct A2uiMessageProcessor {
    /// Component registry
    registry: ComponentRegistry,

    /// Active surfaces by ID
    surfaces: HashMap<String, Surface>,

    /// Data models for each surface
    data_models: SurfaceDataModels,

    /// Pending user actions to send
    pending_actions: Vec<UserAction>,
}

impl A2uiMessageProcessor {
    /// Create a new processor with the given component registry
    pub fn new(registry: ComponentRegistry) -> Self {
        A2uiMessageProcessor {
            registry,
            surfaces: HashMap::new(),
            data_models: SurfaceDataModels::new(),
            pending_actions: Vec::new(),
        }
    }

    /// Create a new processor with the standard component catalog
    pub fn with_standard_catalog() -> Self {
        Self::new(ComponentRegistry::with_standard_catalog())
    }

    /// Get the component registry
    pub fn registry(&self) -> &ComponentRegistry {
        &self.registry
    }

    /// Get a surface by ID
    pub fn get_surface(&self, surface_id: &str) -> Option<&Surface> {
        self.surfaces.get(surface_id)
    }

    /// Get a mutable surface by ID
    pub fn get_surface_mut(&mut self, surface_id: &str) -> Option<&mut Surface> {
        self.surfaces.get_mut(surface_id)
    }

    /// Get all surface IDs
    pub fn surface_ids(&self) -> impl Iterator<Item = &String> {
        self.surfaces.keys()
    }

    /// Get the data model for a surface
    pub fn get_data_model(&self, surface_id: &str) -> Option<&DataModel> {
        self.data_models.get(surface_id)
    }

    /// Get a mutable data model for a surface
    pub fn get_data_model_mut(&mut self, surface_id: &str) -> Option<&mut DataModel> {
        self.data_models.get_mut(surface_id)
    }

    /// Process a single A2UI message
    ///
    /// Returns a list of events that occurred as a result of processing.
    pub fn process_message(&mut self, message: A2uiMessage) -> Vec<ProcessorEvent> {
        match message {
            A2uiMessage::BeginRendering(msg) => self.process_begin_rendering(msg),
            A2uiMessage::SurfaceUpdate(msg) => self.process_surface_update(msg),
            A2uiMessage::DataModelUpdate(msg) => self.process_data_model_update(msg),
            A2uiMessage::DeleteSurface(msg) => self.process_delete_surface(msg),
            A2uiMessage::UserAction(msg) => {
                // UserAction is typically sent TO the server, not processed here
                // But we store it for the host to retrieve
                self.pending_actions.push(msg);
                vec![]
            }
        }
    }

    /// Process multiple A2UI messages (e.g., from a JSON array)
    pub fn process_messages(&mut self, messages: Vec<A2uiMessage>) -> Vec<ProcessorEvent> {
        let mut events = Vec::new();
        for message in messages {
            events.extend(self.process_message(message));
        }
        events
    }

    /// Parse and process a JSON string containing A2UI messages
    pub fn process_json(&mut self, json: &str) -> Result<Vec<ProcessorEvent>, serde_json::Error> {
        // Try to parse as array first
        if let Ok(messages) = serde_json::from_str::<Vec<A2uiMessage>>(json) {
            return Ok(self.process_messages(messages));
        }

        // Try to parse as single message
        let message: A2uiMessage = serde_json::from_str(json)?;
        Ok(self.process_message(message))
    }

    /// Take pending user actions (clears the queue)
    pub fn take_pending_actions(&mut self) -> Vec<UserAction> {
        std::mem::take(&mut self.pending_actions)
    }

    /// Queue a user action to be sent
    pub fn queue_user_action(&mut self, action: UserAction) {
        self.pending_actions.push(action);
    }

    /// Create a user action from a button click
    pub fn create_action(
        &self,
        surface_id: &str,
        component_id: &str,
        action_def: &ActionDefinition,
    ) -> UserAction {
        let mut context = HashMap::new();

        // Resolve context values from data model
        if let Some(data_model) = self.get_data_model(surface_id) {
            for item in &action_def.context {
                let value = match &item.value {
                    ActionValue::String(sv) => match sv {
                        StringValue::Literal { literal_string } => {
                            serde_json::Value::String(literal_string.clone())
                        }
                        StringValue::Path { path } => data_model
                            .get(path)
                            .cloned()
                            .unwrap_or(serde_json::Value::Null),
                    },
                    ActionValue::Number(nv) => match nv {
                        NumberValue::Literal { literal_number } => {
                            serde_json::json!(*literal_number)
                        }
                        NumberValue::Path { path } => data_model
                            .get(path)
                            .cloned()
                            .unwrap_or(serde_json::Value::Null),
                    },
                    ActionValue::Boolean(bv) => match bv {
                        BooleanValue::Literal { literal_boolean } => {
                            serde_json::Value::Bool(*literal_boolean)
                        }
                        BooleanValue::Path { path } => data_model
                            .get(path)
                            .cloned()
                            .unwrap_or(serde_json::Value::Null),
                    },
                };
                context.insert(item.key.clone(), value);
            }
        }

        UserAction {
            surface_id: surface_id.to_string(),
            action: UserActionPayload {
                name: action_def.name.clone(),
                context,
            },
            component_id: Some(component_id.to_string()),
        }
    }

    // ========================================================================
    // Private processing methods
    // ========================================================================

    fn process_begin_rendering(&mut self, msg: BeginRendering) -> Vec<ProcessorEvent> {
        let surface = Surface::new(msg.surface_id.clone(), msg.root, msg.styles);

        // Create data model for this surface
        self.data_models.get_or_create(&msg.surface_id);

        // Store surface
        self.surfaces.insert(msg.surface_id.clone(), surface);

        vec![ProcessorEvent::SurfaceCreated(SurfaceCreatedEvent {
            surface_id: msg.surface_id,
        })]
    }

    fn process_surface_update(&mut self, msg: SurfaceUpdate) -> Vec<ProcessorEvent> {
        let surface = match self.surfaces.get_mut(&msg.surface_id) {
            Some(s) => s,
            None => {
                // Create surface implicitly if it doesn't exist
                let surface = Surface::new(msg.surface_id.clone(), String::new(), None);
                self.surfaces.insert(msg.surface_id.clone(), surface);
                self.data_models.get_or_create(&msg.surface_id);
                self.surfaces.get_mut(&msg.surface_id).unwrap()
            }
        };

        let mut updated_ids = Vec::new();

        for component in msg.components {
            updated_ids.push(component.id.clone());
            surface.components.insert(component.id.clone(), component);
        }

        surface.mark_dirty();

        vec![ProcessorEvent::SurfaceUpdated(SurfaceUpdatedEvent {
            surface_id: msg.surface_id,
            updated_components: updated_ids,
        })]
    }

    fn process_data_model_update(&mut self, msg: DataModelUpdate) -> Vec<ProcessorEvent> {
        let data_model = self.data_models.get_or_create(&msg.surface_id);

        let mut updated_paths = Vec::new();

        for content in &msg.contents {
            let full_path = if msg.path == "/" {
                format!("/{}", content.key)
            } else {
                format!("{}/{}", msg.path.trim_end_matches('/'), content.key)
            };
            updated_paths.push(full_path);
        }

        data_model.apply_updates(&msg.path, &msg.contents);

        // Mark surface as needing redraw
        if let Some(surface) = self.surfaces.get_mut(&msg.surface_id) {
            surface.mark_dirty();
        }

        vec![ProcessorEvent::DataModelUpdated(DataModelUpdatedEvent {
            surface_id: msg.surface_id,
            updated_paths,
        })]
    }

    fn process_delete_surface(&mut self, msg: DeleteSurface) -> Vec<ProcessorEvent> {
        self.surfaces.remove(&msg.surface_id);
        self.data_models.remove(&msg.surface_id);

        vec![ProcessorEvent::SurfaceDeleted(SurfaceDeletedEvent {
            surface_id: msg.surface_id,
        })]
    }
}

/// Resolve a StringValue to an actual string using the data model
pub fn resolve_string_value(value: &StringValue, data_model: &DataModel) -> String {
    match value {
        StringValue::Literal { literal_string } => literal_string.clone(),
        StringValue::Path { path } => data_model
            .get_string(path)
            .map(|s| s.to_string())
            .unwrap_or_default(),
    }
}

/// Resolve a NumberValue to an actual number using the data model
pub fn resolve_number_value(value: &NumberValue, data_model: &DataModel) -> f64 {
    match value {
        NumberValue::Literal { literal_number } => *literal_number,
        NumberValue::Path { path } => data_model.get_number(path).unwrap_or(0.0),
    }
}

/// Resolve a BooleanValue to an actual boolean using the data model
pub fn resolve_boolean_value(value: &BooleanValue, data_model: &DataModel) -> bool {
    match value {
        BooleanValue::Literal { literal_boolean } => *literal_boolean,
        BooleanValue::Path { path } => data_model.get_bool(path).unwrap_or(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_begin_rendering() {
        let mut processor = A2uiMessageProcessor::with_standard_catalog();

        let msg = A2uiMessage::BeginRendering(BeginRendering {
            surface_id: "main".to_string(),
            root: "root".to_string(),
            styles: None,
        });

        let events = processor.process_message(msg);

        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            ProcessorEvent::SurfaceCreated(e) if e.surface_id == "main"
        ));

        assert!(processor.get_surface("main").is_some());
        assert!(processor.get_data_model("main").is_some());
    }

    #[test]
    fn test_process_surface_update() {
        let mut processor = A2uiMessageProcessor::with_standard_catalog();

        // First create the surface
        processor.process_message(A2uiMessage::BeginRendering(BeginRendering {
            surface_id: "main".to_string(),
            root: "root".to_string(),
            styles: None,
        }));

        // Then update it
        let msg = A2uiMessage::SurfaceUpdate(SurfaceUpdate {
            surface_id: "main".to_string(),
            components: vec![ComponentDefinition {
                id: "title".to_string(),
                weight: None,
                component: ComponentType::Text(TextComponent {
                    text: StringValue::literal("Hello"),
                    usage_hint: Some(TextUsageHint::H1),
                }),
            }],
        });

        let events = processor.process_message(msg);

        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            ProcessorEvent::SurfaceUpdated(e) if e.surface_id == "main"
        ));

        let surface = processor.get_surface("main").unwrap();
        assert!(surface.get_component("title").is_some());
    }

    #[test]
    fn test_process_data_model_update() {
        let mut processor = A2uiMessageProcessor::with_standard_catalog();

        // Create surface
        processor.process_message(A2uiMessage::BeginRendering(BeginRendering {
            surface_id: "main".to_string(),
            root: "root".to_string(),
            styles: None,
        }));

        // Update data model
        let msg = A2uiMessage::DataModelUpdate(DataModelUpdate {
            surface_id: "main".to_string(),
            path: "/".to_string(),
            contents: vec![DataContent {
                key: "name".to_string(),
                value: DataValue::ValueString("Alice".to_string()),
            }],
        });

        let events = processor.process_message(msg);

        assert_eq!(events.len(), 1);

        let data_model = processor.get_data_model("main").unwrap();
        assert_eq!(data_model.get_string("/name"), Some("Alice"));
    }

    #[test]
    fn test_resolve_string_value() {
        let mut data_model = DataModel::new();
        data_model.set_string("/user/name", "Bob");

        // Test literal
        let literal = StringValue::literal("Hello");
        assert_eq!(resolve_string_value(&literal, &data_model), "Hello");

        // Test path
        let path = StringValue::path("/user/name");
        assert_eq!(resolve_string_value(&path, &data_model), "Bob");
    }
}
