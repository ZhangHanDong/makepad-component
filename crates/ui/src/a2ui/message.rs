//! A2UI Protocol Message Types
//!
//! This module defines the Rust types for all A2UI protocol messages.
//! Messages are serialized/deserialized using serde_json.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::value::{BooleanValue, NumberValue, StringValue};

/// Top-level A2UI message enum.
///
/// Each variant corresponds to one of the A2UI protocol message types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum A2uiMessage {
    /// Initialize a new UI surface
    BeginRendering(BeginRendering),

    /// Add or update components in the tree
    SurfaceUpdate(SurfaceUpdate),

    /// Update the data model
    DataModelUpdate(DataModelUpdate),

    /// Delete a surface
    DeleteSurface(DeleteSurface),

    /// User action event (sent from client to server)
    UserAction(UserAction),
}

impl A2uiMessage {
    /// Get the surface ID this message applies to
    pub fn surface_id(&self) -> &str {
        match self {
            A2uiMessage::BeginRendering(m) => &m.surface_id,
            A2uiMessage::SurfaceUpdate(m) => &m.surface_id,
            A2uiMessage::DataModelUpdate(m) => &m.surface_id,
            A2uiMessage::DeleteSurface(m) => &m.surface_id,
            A2uiMessage::UserAction(m) => &m.surface_id,
        }
    }
}

/// Initialize a new UI surface.
///
/// # Example JSON
///
/// ```text
/// {
///   "beginRendering": {
///     "surfaceId": "main",
///     "root": "root-column",
///     "styles": {
///       "primaryColor": "#007BFF",
///       "font": "Roboto"
///     }
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeginRendering {
    /// Unique identifier for this surface
    pub surface_id: String,

    /// ID of the root component
    pub root: String,

    /// Optional style configuration
    #[serde(default)]
    pub styles: Option<SurfaceStyles>,
}

/// Style configuration for a surface
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceStyles {
    /// Primary color (hex format)
    #[serde(default)]
    pub primary_color: Option<String>,

    /// Font family name
    #[serde(default)]
    pub font: Option<String>,

    /// Additional custom styles
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Add or update components in the surface.
///
/// # Example JSON
///
/// ```text
/// {
///   "surfaceUpdate": {
///     "surfaceId": "main",
///     "components": [
///       {
///         "id": "root",
///         "component": {
///           "Column": {
///             "children": {"explicitList": ["header", "content"]}
///           }
///         }
///       }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceUpdate {
    /// Target surface ID
    pub surface_id: String,

    /// Components to add or update
    pub components: Vec<ComponentDefinition>,
}

/// A single component definition in the adjacency list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDefinition {
    /// Unique component ID
    pub id: String,

    /// Optional flex weight for Row/Column layouts
    #[serde(default)]
    pub weight: Option<f64>,

    /// The component type and properties
    pub component: ComponentType,
}

/// Component type enum - each variant is a different widget type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentType {
    // Layout components
    Column(ColumnComponent),
    Row(RowComponent),
    List(ListComponent),
    Card(CardComponent),

    // Display components
    Text(TextComponent),
    Image(ImageComponent),
    Icon(IconComponent),
    Divider(DividerComponent),

    // Interactive components
    Button(ButtonComponent),
    TextField(TextFieldComponent),
    CheckBox(CheckBoxComponent),
    Slider(SliderComponent),
    MultipleChoice(MultipleChoiceComponent),

    // Container components
    Modal(ModalComponent),
    Tabs(TabsComponent),

    // Visualization components
    Chart(ChartComponent),
    // Media components
    AudioPlayer(AudioPlayerComponent),
}

/// Children reference - either explicit list or template-based
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChildrenRef {
    /// Explicit list of child component IDs
    ExplicitList(Vec<String>),

    /// Template-based children (for dynamic lists)
    Template {
        /// Template component ID
        #[serde(rename = "componentId")]
        component_id: String,
        /// Data binding path for the list data
        #[serde(rename = "dataBinding")]
        data_binding: String,
    },
}

impl Default for ChildrenRef {
    fn default() -> Self {
        ChildrenRef::ExplicitList(vec![])
    }
}

// ============================================================================
// Layout Components
// ============================================================================

/// Vertical layout container
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnComponent {
    /// Child component references
    #[serde(default)]
    pub children: ChildrenRef,

    /// Cross-axis alignment
    #[serde(default)]
    pub alignment: Option<Alignment>,

    /// Main-axis distribution
    #[serde(default)]
    pub distribution: Option<Distribution>,
}

/// Horizontal layout container
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowComponent {
    /// Child component references
    #[serde(default)]
    pub children: ChildrenRef,

    /// Cross-axis alignment
    #[serde(default)]
    pub alignment: Option<Alignment>,

    /// Main-axis distribution
    #[serde(default)]
    pub distribution: Option<Distribution>,
}

/// Scrollable list container
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListComponent {
    /// Child component references (usually template-based)
    #[serde(default)]
    pub children: ChildrenRef,

    /// Scroll direction
    #[serde(default)]
    pub direction: Option<ListDirection>,
}

/// Card container with optional styling
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardComponent {
    /// Single child component ID
    pub child: String,

    /// Elevation level (shadow depth)
    #[serde(default)]
    pub elevation: Option<u8>,
}

// ============================================================================
// Display Components
// ============================================================================

/// Text display component
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextComponent {
    /// Text content (literal or path-bound)
    pub text: StringValue,

    /// Usage hint for styling (h1, h2, h3, body, caption, etc.)
    #[serde(default)]
    pub usage_hint: Option<TextUsageHint>,
}

/// Image display component
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageComponent {
    /// Image URL (literal or path-bound)
    pub url: StringValue,

    /// Fit mode
    #[serde(default)]
    pub fit: Option<ImageFit>,

    /// Usage hint for sizing
    #[serde(default)]
    pub usage_hint: Option<ImageUsageHint>,
}

/// Icon component
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconComponent {
    /// Icon name (e.g., "settings", "check", "close")
    pub name: StringValue,

    /// Icon size in logical pixels
    #[serde(default)]
    pub size: Option<f64>,
}

/// Visual divider/separator
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DividerComponent {
    /// Orientation
    #[serde(default)]
    pub orientation: Option<Orientation>,
}

// ============================================================================
// Interactive Components
// ============================================================================

/// Clickable button component
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonComponent {
    /// Child component ID (button content)
    pub child: String,

    /// Whether this is a primary action
    #[serde(default)]
    pub primary: Option<bool>,

    /// Action to trigger on click
    #[serde(default)]
    pub action: Option<ActionDefinition>,
}

/// Text input field
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextFieldComponent {
    /// Current value (path-bound for two-way binding)
    pub text: StringValue,

    /// Label text
    #[serde(default)]
    pub label: Option<StringValue>,

    /// Placeholder text
    #[serde(default)]
    pub placeholder: Option<StringValue>,

    /// Input type
    #[serde(default)]
    pub input_type: Option<TextInputType>,
}

/// Checkbox component
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckBoxComponent {
    /// Current checked state (path-bound)
    pub value: BooleanValue,

    /// Label text
    #[serde(default)]
    pub label: Option<StringValue>,
}

/// Slider component for numeric input
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SliderComponent {
    /// Current value (path-bound)
    pub value: NumberValue,

    /// Minimum value
    #[serde(default)]
    pub min: Option<f64>,

    /// Maximum value
    #[serde(default)]
    pub max: Option<f64>,

    /// Step size
    #[serde(default)]
    pub step: Option<f64>,
}

/// Multiple choice selection
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleChoiceComponent {
    /// Selected value(s) (path-bound)
    pub value: StringValue,

    /// Available options
    pub options: Vec<ChoiceOption>,

    /// Allow multiple selections
    #[serde(default)]
    pub multi_select: Option<bool>,
}

/// A single choice option
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceOption {
    /// Option value
    pub value: String,
    /// Display label
    pub label: StringValue,
}

// ============================================================================
// Container Components
// ============================================================================

/// Modal dialog overlay
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModalComponent {
    /// Visibility state (path-bound)
    pub visible: BooleanValue,

    /// Child component references
    #[serde(default)]
    pub children: ChildrenRef,
}

/// Tabbed interface
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabsComponent {
    /// Tab definitions
    pub tabs: Vec<TabDefinition>,

    /// Currently selected tab ID
    #[serde(default)]
    pub selected: Option<StringValue>,
}

/// A single tab definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabDefinition {
    /// Tab ID
    pub id: String,
    /// Tab label
    pub label: StringValue,
    /// Content component ID
    pub content: String,
}

// ============================================================================
// Visualization Components
// ============================================================================

/// Chart visualization component (bar, line, pie, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartComponent {
    /// Chart type
    pub chart_type: ChartType,

    /// Chart title (optional)
    #[serde(default)]
    pub title: Option<StringValue>,

    /// Category labels (X-axis for bar/line, slice labels for pie)
    #[serde(default)]
    pub labels: Vec<String>,

    /// Data series - each series has a name and values
    pub series: Vec<ChartSeries>,

    /// Chart width in logical pixels
    #[serde(default = "default_chart_width")]
    pub width: f64,

    /// Chart height in logical pixels
    #[serde(default = "default_chart_height")]
    pub height: f64,

    /// Color palette (hex strings, optional - defaults to built-in palette)
    #[serde(default)]
    pub colors: Vec<String>,

    /// Whether to show legend
    #[serde(default)]
    pub show_legend: Option<bool>,

    /// Max value (used by gauge chart as dial maximum, default 100)
    #[serde(default)]
    pub max_value: Option<f64>,

    /// Whether the chart is interactive (pan/zoom)
    #[serde(default)]
    pub interactive: Option<bool>,

    /// Colormap name (e.g., "viridis", "plasma", "inferno")
    #[serde(default)]
    pub colormap: Option<String>,

    /// Whether to stack series data
    #[serde(default)]
    pub stacked: Option<bool>,

    /// Whether bars/layout should be horizontal
    #[serde(default)]
    pub horizontal: Option<bool>,

    /// X-axis label
    #[serde(default)]
    pub x_label: Option<String>,

    /// Y-axis label
    #[serde(default)]
    pub y_label: Option<String>,
}

fn default_chart_width() -> f64 {
    400.0
}

fn default_chart_height() -> f64 {
    300.0
}

/// A data series for charts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartSeries {
    /// Series name (for legend)
    #[serde(default)]
    pub name: Option<String>,

    /// Data values (Y-axis or primary values)
    pub values: Vec<f64>,

    /// Explicit X-axis values (optional; defaults to 0, 1, 2, ... if absent)
    #[serde(default)]
    pub x_values: Option<Vec<f64>>,
}

/// Supported chart types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChartType {
    // Original types
    Bar,
    Line,
    Pie,
    Area,
    Scatter,
    Radar,
    Gauge,
    Bubble,
    Candlestick,
    Heatmap,
    Treemap,
    Chord,
    Sankey,
    // New types from makepad-plot
    Histogram,
    BoxPlot,
    Donut,
    Stem,
    Violin,
    Polar,
    Contour,
    Waterfall,
    Funnel,
    Step,
    Stackplot,
    Hexbin,
    Streamgraph,
    // 3D types
    Surface3d,
    Scatter3d,
    Line3d,
}

// Media Components
// ============================================================================

/// Audio player component for playing audio URLs
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioPlayerComponent {
    /// Audio URL (literal or path-bound)
    pub url: StringValue,

    /// Optional title for the audio
    #[serde(default)]
    pub title: Option<StringValue>,

    /// Optional artist name
    #[serde(default)]
    pub artist: Option<StringValue>,

    /// Whether to autoplay (default false)
    #[serde(default)]
    pub autoplay: Option<bool>,
}

// ============================================================================
// Enums
// ============================================================================

/// Alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}

/// Distribution options for Row/Column
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Distribution {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// List scroll direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ListDirection {
    #[default]
    Vertical,
    Horizontal,
}

/// Text usage hints for styling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextUsageHint {
    H1,
    H2,
    H3,
    H4,
    H5,
    Body,
    Caption,
    Code,
}

/// Image fit modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ImageFit {
    #[default]
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}

/// Image usage hints for sizing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ImageUsageHint {
    Icon,
    Avatar,
    SmallFeature,
    MediumFeature,
    LargeFeature,
    Header,
}

/// Orientation for dividers etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Text input types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextInputType {
    #[default]
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
}

// ============================================================================
// Action & Data Model
// ============================================================================

/// Action definition for interactive components
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDefinition {
    /// Action name (e.g., "addToCart", "submit")
    pub name: String,

    /// Context values to include with the action
    #[serde(default)]
    pub context: Vec<ActionContextItem>,
}

/// A single context item for an action
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionContextItem {
    /// Key name
    pub key: String,

    /// Value (literal or path-bound)
    pub value: ActionValue,
}

/// Value type for action context
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActionValue {
    String(StringValue),
    Number(NumberValue),
    Boolean(BooleanValue),
}

/// Update the data model.
///
/// # Example JSON
///
/// ```text
/// {
///   "dataModelUpdate": {
///     "surfaceId": "main",
///     "path": "/",
///     "contents": [
///       {"key": "products", "valueMap": [...]}
///     ]
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataModelUpdate {
    /// Target surface ID
    pub surface_id: String,

    /// Base path for updates (default "/")
    #[serde(default = "default_path")]
    pub path: String,

    /// Data updates
    pub contents: Vec<DataContent>,
}

fn default_path() -> String {
    "/".to_string()
}

/// A single data content item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataContent {
    /// Key name
    pub key: String,

    /// Value (one of the typed variants)
    #[serde(flatten)]
    pub value: DataValue,
}

/// Data value types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataValue {
    /// String value
    ValueString(String),
    /// Number value
    ValueNumber(f64),
    /// Boolean value
    ValueBoolean(bool),
    /// Nested map (object)
    ValueMap(Vec<DataContent>),
    /// Array of values
    ValueArray(Vec<DataValue>),
}

/// Delete a surface
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSurface {
    /// Surface ID to delete
    pub surface_id: String,
}

/// User action event (sent from client to server)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAction {
    /// Source surface ID
    pub surface_id: String,

    /// Action details
    pub action: UserActionPayload,

    /// Source component ID
    #[serde(default)]
    pub component_id: Option<String>,
}

/// User action payload
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserActionPayload {
    /// Action name
    pub name: String,

    /// Context values (resolved from data model)
    #[serde(default)]
    pub context: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_begin_rendering() {
        let json = r##"{"beginRendering": {"surfaceId": "main", "root": "root-column", "styles": {"primaryColor": "#007BFF"}}}"##;

        let msg: A2uiMessage = serde_json::from_str(json).unwrap();
        match msg {
            A2uiMessage::BeginRendering(br) => {
                assert_eq!(br.surface_id, "main");
                assert_eq!(br.root, "root-column");
                assert_eq!(
                    br.styles.as_ref().unwrap().primary_color,
                    Some("#007BFF".to_string())
                );
            }
            _ => panic!("Expected BeginRendering"),
        }
    }

    #[test]
    fn test_parse_surface_update() {
        let json = r##"{"surfaceUpdate": {"surfaceId": "main", "components": [{"id": "title", "component": {"Text": {"text": {"literalString": "Hello"}, "usageHint": "h1"}}}]}}"##;

        let msg: A2uiMessage = serde_json::from_str(json).unwrap();
        match msg {
            A2uiMessage::SurfaceUpdate(su) => {
                assert_eq!(su.surface_id, "main");
                assert_eq!(su.components.len(), 1);
                assert_eq!(su.components[0].id, "title");
            }
            _ => panic!("Expected SurfaceUpdate"),
        }
    }

    #[test]
    fn test_parse_data_model_update() {
        let json = r##"{"dataModelUpdate": {"surfaceId": "main", "path": "/", "contents": [{"key": "name", "valueString": "Alice"}, {"key": "count", "valueNumber": 42}]}}"##;

        let msg: A2uiMessage = serde_json::from_str(json).unwrap();
        match msg {
            A2uiMessage::DataModelUpdate(dm) => {
                assert_eq!(dm.surface_id, "main");
                assert_eq!(dm.contents.len(), 2);
            }
            _ => panic!("Expected DataModelUpdate"),
        }
    }

    #[test]
    fn test_parse_data_model_with_array() {
        let json = r##"{"dataModelUpdate": {"surfaceId": "main", "path": "/", "contents": [{"key": "products", "valueArray": [{"valueMap": [{"key": "name", "valueString": "Test"}]}]}]}}"##;

        let result: Result<A2uiMessage, _> = serde_json::from_str(json);
        println!("Parse result: {:?}", result);

        let msg = result.unwrap();
        match msg {
            A2uiMessage::DataModelUpdate(dm) => {
                assert_eq!(dm.surface_id, "main");
                assert_eq!(dm.contents.len(), 1);
            }
            _ => panic!("Expected DataModelUpdate"),
        }
    }

    #[test]
    fn test_parse_full_demo_json() {
        // Test parsing the complete demo JSON as Vec<A2uiMessage>
        let json = r##"[
            {"beginRendering": {"surfaceId": "main", "root": "root-column"}},
            {"surfaceUpdate": {"surfaceId": "main", "components": []}},
            {"dataModelUpdate": {"surfaceId": "main", "path": "/", "contents": [{"key": "products", "valueArray": [{"valueMap": [{"key": "name", "valueString": "Test"}]}]}]}}
        ]"##;

        let result: Result<Vec<A2uiMessage>, _> = serde_json::from_str(json);
        println!("Full demo parse result: {:?}", result);

        let messages = result.unwrap();
        assert_eq!(messages.len(), 3);
    }

    #[test]
    fn test_parse_audio_player() {
        // Test parsing AudioPlayer component
        let json = r##"{"surfaceUpdate": {"surfaceId": "main", "components": [
            {
                "id": "player1",
                "component": {
                    "AudioPlayer": {
                        "url": {"literalString": "https://example.com/song.mp3"},
                        "title": {"literalString": "My Song"},
                        "artist": {"literalString": "Artist Name"}
                    }
                }
            }
        ]}}"##;

        let msg: A2uiMessage = serde_json::from_str(json).unwrap();
        match msg {
            A2uiMessage::SurfaceUpdate(su) => {
                assert_eq!(su.components.len(), 1);
                assert_eq!(su.components[0].id, "player1");
                match &su.components[0].component {
                    ComponentType::AudioPlayer(player) => {
                        assert_eq!(player.url.as_literal(), Some("https://example.com/song.mp3"));
                        assert_eq!(player.title.as_ref().and_then(|t| t.as_literal()), Some("My Song"));
                        assert_eq!(player.artist.as_ref().and_then(|a| a.as_literal()), Some("Artist Name"));
                    }
                    _ => panic!("Expected AudioPlayer"),
                }
            }
            _ => panic!("Expected SurfaceUpdate"),
        }
    }

    #[test]
    fn test_parse_audio_player_with_path() {
        // Test parsing AudioPlayer with data binding
        let json = r##"{"surfaceUpdate": {"surfaceId": "main", "components": [
            {
                "id": "player2",
                "component": {
                    "AudioPlayer": {
                        "url": {"path": "/currentSong/audioUrl"},
                        "title": {"path": "/currentSong/title"}
                    }
                }
            }
        ]}}"##;

        let msg: A2uiMessage = serde_json::from_str(json).unwrap();
        match msg {
            A2uiMessage::SurfaceUpdate(su) => {
                assert_eq!(su.components.len(), 1);
                match &su.components[0].component {
                    ComponentType::AudioPlayer(player) => {
                        assert_eq!(player.url.as_path(), Some("/currentSong/audioUrl"));
                        assert_eq!(player.title.as_ref().and_then(|t| t.as_path()), Some("/currentSong/title"));
                    }
                    _ => panic!("Expected AudioPlayer"),
                }
            }
            _ => panic!("Expected SurfaceUpdate"),
        }
    }
}
