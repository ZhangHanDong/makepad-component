use makepad_widgets::*;
use makepad_component::widgets::*;
use serde::{Deserialize, Serialize};

// ============================================================
// JsonRenderDemo - JSON-based Dynamic UI Generation
// ============================================================

/// JSON Widget types for A2UI protocol
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum JsonWidget {
    View {
        #[serde(default)]
        props: JsonViewProps,
        #[serde(default)]
        children: Vec<JsonWidget>,
    },
    HStack {
        #[serde(default)]
        props: JsonStackProps,
        #[serde(default)]
        children: Vec<JsonWidget>,
    },
    VStack {
        #[serde(default)]
        props: JsonStackProps,
        #[serde(default)]
        children: Vec<JsonWidget>,
    },
    Label {
        #[serde(default)]
        props: JsonLabelProps,
    },
    Button {
        #[serde(default)]
        props: JsonButtonProps,
    },
    Card {
        #[serde(default)]
        props: JsonCardProps,
    },
    Progress {
        #[serde(default)]
        props: JsonProgressProps,
    },
    Switch {
        #[serde(default)]
        props: JsonSwitchProps,
    },
    TextInput {
        #[serde(default)]
        props: JsonInputProps,
    },
    Image {
        #[serde(default)]
        props: JsonImageProps,
    },
    Divider,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JsonViewProps {
    #[serde(default)]
    pub padding: Option<f64>,
    #[serde(default)]
    pub spacing: Option<f64>,
    #[serde(default)]
    pub background: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JsonStackProps {
    #[serde(default)]
    pub spacing: Option<f64>,
    #[serde(default)]
    pub align: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JsonLabelProps {
    #[serde(default)]
    pub text: String,
    #[serde(rename = "fontSize", default)]
    pub font_size: Option<f64>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub bold: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JsonButtonProps {
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub disabled: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JsonCardProps {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JsonProgressProps {
    #[serde(default)]
    pub value: f64,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JsonSwitchProps {
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub checked: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JsonInputProps {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub placeholder: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct JsonImageProps {
    #[serde(default)]
    pub src: Option<String>,
    #[serde(default)]
    pub width: Option<f64>,
    #[serde(default)]
    pub height: Option<f64>,
}

/// Flattened widget for PortalList rendering
#[derive(Clone, Debug)]
pub enum FlatWidget {
    Label { text: String },
    Button { text: String },
    Card { title: String, description: String },
    Progress { value: f64, label: String },
    Switch { label: String },
    Input { label: String, placeholder: String },
    Image,
    Divider,
}

#[derive(Live, Widget)]
pub struct JsonRenderDemo {
    #[deref] view: View,
    #[rust] flat_widgets: Vec<FlatWidget>,
}

impl LiveHook for JsonRenderDemo {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.flat_widgets = Vec::new();
    }
}

impl JsonRenderDemo {
    /// Parse JSON string into JsonWidget tree
    fn parse_json(&self, json: &str) -> Result<JsonWidget, String> {
        // Try to extract JSON from markdown code block
        let json_str = if json.contains("```json") {
            json.split("```json")
                .nth(1)
                .and_then(|s| s.split("```").next())
                .map(|s| s.trim())
                .unwrap_or(json.trim())
        } else if json.contains("```") {
            json.split("```")
                .nth(1)
                .map(|s| s.trim())
                .unwrap_or(json.trim())
        } else {
            json.trim()
        };

        serde_json::from_str(json_str)
            .map_err(|e| format!("JSON Parse Error: {}", e))
    }

    /// Flatten widget tree for PortalList rendering
    fn flatten_widgets(widget: &JsonWidget, result: &mut Vec<FlatWidget>) {
        match widget {
            JsonWidget::View { children, .. } |
            JsonWidget::VStack { children, .. } |
            JsonWidget::HStack { children, .. } => {
                for child in children {
                    Self::flatten_widgets(child, result);
                }
            }
            JsonWidget::Label { props } => {
                result.push(FlatWidget::Label {
                    text: props.text.clone(),
                });
            }
            JsonWidget::Button { props } => {
                result.push(FlatWidget::Button {
                    text: props.text.clone(),
                });
            }
            JsonWidget::Card { props } => {
                result.push(FlatWidget::Card {
                    title: props.title.clone(),
                    description: props.description.clone().unwrap_or_default(),
                });
            }
            JsonWidget::Progress { props } => {
                result.push(FlatWidget::Progress {
                    value: props.value,
                    label: props.label.clone().unwrap_or_else(|| format!("{}%", props.value as i32)),
                });
            }
            JsonWidget::Switch { props } => {
                result.push(FlatWidget::Switch {
                    label: props.label.clone(),
                });
            }
            JsonWidget::TextInput { props } => {
                result.push(FlatWidget::Input {
                    label: props.label.clone().unwrap_or_default(),
                    placeholder: props.placeholder.clone().unwrap_or_else(|| "Enter text...".to_string()),
                });
            }
            JsonWidget::Image { .. } => {
                result.push(FlatWidget::Image);
            }
            JsonWidget::Divider => {
                result.push(FlatWidget::Divider);
            }
        }
    }

    /// Get example JSON for demonstration
    fn get_example_json() -> &'static str {
        r#"{
  "type": "VStack",
  "props": { "spacing": 16 },
  "children": [
    {
      "type": "Card",
      "props": {
        "title": "User Profile",
        "description": "Dynamically generated card"
      }
    },
    {
      "type": "Label",
      "props": { "text": "Welcome to JSON Render!", "bold": true }
    },
    {
      "type": "HStack",
      "props": { "spacing": 12 },
      "children": [
        { "type": "Button", "props": { "text": "Submit" } },
        { "type": "Button", "props": { "text": "Cancel" } }
      ]
    },
    {
      "type": "Progress",
      "props": { "value": 75, "label": "Loading: 75%" }
    },
    {
      "type": "Switch",
      "props": { "label": "Dark Mode" }
    },
    {
      "type": "TextInput",
      "props": { "label": "Email", "placeholder": "Enter your email" }
    },
    { "type": "Divider" },
    {
      "type": "Label",
      "props": { "text": "Generated via A2UI Protocol" }
    }
  ]
}"#
    }
}

impl Widget for JsonRenderDemo {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Render button clicked
        if self.view.mp_button(ids!(render_btn)).clicked(&actions) {
            let json_text = self.view.text_input(ids!(json_input)).text();

            match self.parse_json(&json_text) {
                Ok(widget_tree) => {
                    self.flat_widgets.clear();
                    Self::flatten_widgets(&widget_tree, &mut self.flat_widgets);

                    self.view.label(ids!(render_status))
                        .set_text(cx, &format!("{} widgets rendered", self.flat_widgets.len()));
                }
                Err(e) => {
                    self.view.label(ids!(render_status))
                        .set_text(cx, &format!("Error: {}", e));
                }
            }

            self.redraw(cx);
        }

        // Clear button clicked
        if self.view.mp_button(ids!(clear_render_btn)).clicked(&actions) {
            self.flat_widgets.clear();
            self.view.text_input(ids!(json_input)).set_text(cx, "");
            self.view.label(ids!(render_status)).set_text(cx, "Ready");
            self.redraw(cx);
        }

        // Load example button clicked
        if self.view.mp_button(ids!(load_example_btn)).clicked(&actions) {
            self.view.text_input(ids!(json_input))
                .set_text(cx, Self::get_example_json());
            self.view.label(ids!(render_status)).set_text(cx, "Example loaded");
            self.redraw(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, self.flat_widgets.len());

                while let Some(item_id) = list.next_visible_item(cx) {
                    if let Some(widget_def) = self.flat_widgets.get(item_id) {
                        match widget_def {
                            FlatWidget::Label { text } => {
                                let item_widget = list.item(cx, item_id, live_id!(JsonLabel));
                                item_widget.label(ids!(json_label_text)).set_text(cx, text);
                                item_widget.draw_all(cx, &mut Scope::empty());
                            }
                            FlatWidget::Button { text } => {
                                let item_widget = list.item(cx, item_id, live_id!(JsonButton));
                                item_widget.mp_button(ids!(json_button)).set_text(text);
                                item_widget.draw_all(cx, &mut Scope::empty());
                            }
                            FlatWidget::Card { title, description } => {
                                let item_widget = list.item(cx, item_id, live_id!(JsonCard));
                                item_widget.label(ids!(json_card_title)).set_text(cx, title);
                                item_widget.label(ids!(json_card_desc)).set_text(cx, description);
                                item_widget.draw_all(cx, &mut Scope::empty());
                            }
                            FlatWidget::Progress { value, label } => {
                                let item_widget = list.item(cx, item_id, live_id!(JsonProgress));
                                item_widget.label(ids!(json_progress_label)).set_text(cx, label);
                                item_widget.mp_progress(ids!(json_progress)).set_value(cx, *value);
                                item_widget.draw_all(cx, &mut Scope::empty());
                            }
                            FlatWidget::Switch { label } => {
                                let item_widget = list.item(cx, item_id, live_id!(JsonSwitch));
                                item_widget.label(ids!(json_switch_label)).set_text(cx, label);
                                item_widget.draw_all(cx, &mut Scope::empty());
                            }
                            FlatWidget::Input { label, placeholder } => {
                                let item_widget = list.item(cx, item_id, live_id!(JsonInput));
                                item_widget.label(ids!(json_input_label)).set_text(cx, label);
                                item_widget.text_input(ids!(json_text_input)).set_text(cx, "");
                                item_widget.draw_all(cx, &mut Scope::empty());
                            }
                            FlatWidget::Image => {
                                let item_widget = list.item(cx, item_id, live_id!(JsonImage));
                                item_widget.draw_all(cx, &mut Scope::empty());
                            }
                            FlatWidget::Divider => {
                                let item_widget = list.item(cx, item_id, live_id!(JsonDivider));
                                item_widget.draw_all(cx, &mut Scope::empty());
                            }
                        }
                    }
                }
            }
        }
        DrawStep::done()
    }
}

