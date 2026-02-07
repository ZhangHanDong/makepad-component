use makepad_widgets::*;
use makepad_component::widgets::*;

// ============================================================
// SplashDemo - Natural Language UI Generation
// ============================================================

// Widget type enum for dynamic generation
#[derive(Clone, Debug)]
pub enum GeneratedWidget {
    Button { text: String },
    Label { text: String },
    Card { title: String },
    Progress { value: f64 },
    Switch { label: String },
    Input { placeholder: String },
}

#[derive(Live, Widget)]
pub struct SplashDemo {
    #[deref] view: View,
    #[rust] widgets: Vec<GeneratedWidget>,
}

impl LiveHook for SplashDemo {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.widgets = Vec::new();
    }
}

impl SplashDemo {
    // Parse natural language command and return widget type
    fn parse_command(&self, input: &str) -> Option<GeneratedWidget> {
        let input = input.trim().to_lowercase();

        // Parse "add <type> <content>" pattern
        if let Some(rest) = input.strip_prefix("add ") {
            let rest = rest.trim();

            // Button: "add button Submit"
            if let Some(text) = rest.strip_prefix("button ") {
                return Some(GeneratedWidget::Button {
                    text: text.trim().to_string()
                });
            }
            if rest == "button" {
                return Some(GeneratedWidget::Button {
                    text: "Button".to_string()
                });
            }

            // Label: "add label Hello World"
            if let Some(text) = rest.strip_prefix("label ") {
                return Some(GeneratedWidget::Label {
                    text: text.trim().to_string()
                });
            }
            if rest == "label" {
                return Some(GeneratedWidget::Label {
                    text: "Label".to_string()
                });
            }

            // Card: "add card User Profile"
            if let Some(title) = rest.strip_prefix("card ") {
                return Some(GeneratedWidget::Card {
                    title: title.trim().to_string()
                });
            }
            if rest == "card" {
                return Some(GeneratedWidget::Card {
                    title: "Card".to_string()
                });
            }

            // Progress: "add progress 75"
            if let Some(val) = rest.strip_prefix("progress ") {
                if let Ok(v) = val.trim().parse::<f64>() {
                    return Some(GeneratedWidget::Progress {
                        value: (v / 100.0).clamp(0.0, 1.0)
                    });
                }
            }
            if rest == "progress" {
                return Some(GeneratedWidget::Progress { value: 0.5 });
            }

            // Switch: "add switch Dark Mode"
            if let Some(label) = rest.strip_prefix("switch ") {
                return Some(GeneratedWidget::Switch {
                    label: label.trim().to_string()
                });
            }
            if rest == "switch" {
                return Some(GeneratedWidget::Switch {
                    label: "Toggle".to_string()
                });
            }

            // Input: "add input Email address"
            if let Some(placeholder) = rest.strip_prefix("input ") {
                return Some(GeneratedWidget::Input {
                    placeholder: placeholder.trim().to_string()
                });
            }
            if rest == "input" {
                return Some(GeneratedWidget::Input {
                    placeholder: "Enter text...".to_string()
                });
            }
        }

        None
    }
}

impl Widget for SplashDemo {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Handle button actions
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        // Generate button clicked
        if self.view.mp_button(ids!(generate_btn)).clicked(&actions) {
            let input_text = self.view.text_input(ids!(command_input)).text();

            if input_text.trim().to_lowercase() == "clear" {
                self.widgets.clear();
            } else if let Some(widget) = self.parse_command(&input_text) {
                self.widgets.push(widget);
            }

            // Update count label
            self.view.label(ids!(widget_count_label))
                .set_text(cx, &format!("{} widgets", self.widgets.len()));

            // Clear input
            self.view.text_input(ids!(command_input)).set_text(cx, "");
            self.redraw(cx);
        }

        // Clear button clicked
        if self.view.mp_button(ids!(clear_btn)).clicked(&actions) {
            self.widgets.clear();
            self.view.label(ids!(widget_count_label))
                .set_text(cx, "0 widgets");
            self.redraw(cx);
        }

        // Handle Enter key in text input
        if let Event::KeyDown(ke) = event {
            if ke.key_code == KeyCode::ReturnKey {
                let input_text = self.view.text_input(ids!(command_input)).text();

                if input_text.trim().to_lowercase() == "clear" {
                    self.widgets.clear();
                } else if let Some(widget) = self.parse_command(&input_text) {
                    self.widgets.push(widget);
                }

                self.view.label(ids!(widget_count_label))
                    .set_text(cx, &format!("{} widgets", self.widgets.len()));
                self.view.text_input(ids!(command_input)).set_text(cx, "");
                self.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, self.widgets.len());

                while let Some(item_id) = list.next_visible_item(cx) {
                    if let Some(widget_def) = self.widgets.get(item_id) {
                        match widget_def {
                            GeneratedWidget::Button { text } => {
                                let item_widget = list.item(cx, item_id, live_id!(GenButton));
                                item_widget.mp_button(ids!(gen_button)).set_text(text);
                                item_widget.draw_all(cx, &mut Scope::empty());
                            }
                            GeneratedWidget::Label { text } => {
                                let item_widget = list.item(cx, item_id, live_id!(GenLabel));
                                item_widget.label(ids!(gen_label)).set_text(cx, text);
                                item_widget.draw_all(cx, &mut Scope::empty());
                            }
                            GeneratedWidget::Card { title } => {
                                let item_widget = list.item(cx, item_id, live_id!(GenCard));
                                item_widget.label(ids!(card_title)).set_text(cx, title);
                                item_widget.draw_all(cx, &mut Scope::empty());
                            }
                            GeneratedWidget::Progress { value } => {
                                let item_widget = list.item(cx, item_id, live_id!(GenProgress));
                                let percent = (*value * 100.0) as u32;
                                item_widget.label(ids!(progress_label))
                                    .set_text(cx, &format!("Progress: {}%", percent));
                                item_widget.mp_progress(ids!(gen_progress)).set_value(cx, percent as f64);
                                item_widget.draw_all(cx, &mut Scope::empty());
                            }
                            GeneratedWidget::Switch { label } => {
                                let item_widget = list.item(cx, item_id, live_id!(GenSwitch));
                                item_widget.label(ids!(switch_label)).set_text(cx, label);
                                item_widget.draw_all(cx, &mut Scope::empty());
                            }
                            GeneratedWidget::Input { placeholder } => {
                                let item_widget = list.item(cx, item_id, live_id!(GenInput));
                                item_widget.label(ids!(input_label)).set_text(cx, placeholder);
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

