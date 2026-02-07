impl Widget for A2uiSurface {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Forward events to 3D chart widgets FIRST for interactive rotation/zoom
        self.plot_surface3d.handle_event(cx, event, scope);
        self.plot_scatter3d.handle_event(cx, event, scope);
        self.plot_line3d.handle_event(cx, event, scope);

        let mut needs_redraw = false;
        let surface_id = self.get_surface_id();

        // Handle text input events for focused text field
        if let Some(focused_idx) = self.focused_text_field_idx {
            if let Event::TextInput(te) = event {
                // Insert text at cursor position
                self.text_input_buffer.insert_str(self.cursor_pos, &te.input);
                self.cursor_pos += te.input.len();
                needs_redraw = true;

                // Emit data model change
                if let Some((_, binding_path, _)) = self.text_field_data.get(focused_idx) {
                    if let Some(path) = binding_path {
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            A2uiSurfaceAction::DataModelChanged {
                                surface_id: surface_id.clone(),
                                path: path.clone(),
                                value: serde_json::Value::String(self.text_input_buffer.clone()),
                            },
                        );
                    }
                }
            }

            if let Event::KeyDown(ke) = event {
                match ke.key_code {
                    KeyCode::Backspace => {
                        if self.cursor_pos > 0 {
                            self.cursor_pos -= 1;
                            self.text_input_buffer.remove(self.cursor_pos);
                            needs_redraw = true;

                            // Emit data model change
                            if let Some((_, binding_path, _)) = self.text_field_data.get(focused_idx) {
                                if let Some(path) = binding_path {
                                    cx.widget_action(
                                        self.widget_uid(),
                                        &scope.path,
                                        A2uiSurfaceAction::DataModelChanged {
                                            surface_id: surface_id.clone(),
                                            path: path.clone(),
                                            value: serde_json::Value::String(self.text_input_buffer.clone()),
                                        },
                                    );
                                }
                            }
                        }
                    }
                    KeyCode::Delete => {
                        if self.cursor_pos < self.text_input_buffer.len() {
                            self.text_input_buffer.remove(self.cursor_pos);
                            needs_redraw = true;

                            if let Some((_, binding_path, _)) = self.text_field_data.get(focused_idx) {
                                if let Some(path) = binding_path {
                                    cx.widget_action(
                                        self.widget_uid(),
                                        &scope.path,
                                        A2uiSurfaceAction::DataModelChanged {
                                            surface_id: surface_id.clone(),
                                            path: path.clone(),
                                            value: serde_json::Value::String(self.text_input_buffer.clone()),
                                        },
                                    );
                                }
                            }
                        }
                    }
                    KeyCode::ArrowLeft => {
                        if self.cursor_pos > 0 {
                            self.cursor_pos -= 1;
                            needs_redraw = true;
                        }
                    }
                    KeyCode::ArrowRight => {
                        if self.cursor_pos < self.text_input_buffer.len() {
                            self.cursor_pos += 1;
                            needs_redraw = true;
                        }
                    }
                    KeyCode::Escape => {
                        self.focused_text_field_idx = None;
                        needs_redraw = true;
                    }
                    _ => {}
                }
            }
        }

        // Handle button events
        for (idx, area) in self.button_areas.iter().enumerate() {
            match event.hits(cx, *area) {
                Hit::FingerHoverIn(_) => {
                    if self.hovered_button_idx != Some(idx) {
                        self.hovered_button_idx = Some(idx);
                        cx.set_cursor(MouseCursor::Hand);
                        needs_redraw = true;
                    }
                }
                Hit::FingerHoverOut(_) => {
                    if self.hovered_button_idx == Some(idx) {
                        self.hovered_button_idx = None;
                        cx.set_cursor(MouseCursor::Default);
                        needs_redraw = true;
                    }
                }
                Hit::FingerDown(_) => {
                    self.pressed_button_idx = Some(idx);
                    self.hovered_button_idx = Some(idx);
                    needs_redraw = true;
                }
                Hit::FingerUp(fe) => {
                    if self.pressed_button_idx == Some(idx) {
                        self.pressed_button_idx = None;
                        needs_redraw = true;

                        // Check if released over this button (click confirmed)
                        if fe.is_over {
                            if let Some((component_id, action_def, btn_scope)) =
                                self.button_data.get(idx)
                            {
                                if let Some(action_def) = action_def {
                                    // Create resolved UserAction with data model values
                                    if let Some(processor) = &self.processor {
                                        let user_action = processor.create_action(
                                            &surface_id,
                                            component_id,
                                            action_def,
                                            btn_scope.as_deref(),
                                        );
                                        // Emit widget action for app layer to handle
                                        cx.widget_action(
                                            self.widget_uid(),
                                            &scope.path,
                                            A2uiSurfaceAction::UserAction(user_action),
                                        );
                                    }
                                }
                            }
                            self.hovered_button_idx = Some(idx);
                        } else {
                            self.hovered_button_idx = None;
                            cx.set_cursor(MouseCursor::Default);
                        }
                    }
                }
                _ => {}
            }
        }

        // Handle text field events
        for (idx, area) in self.text_field_areas.iter().enumerate() {
            match event.hits(cx, *area) {
                Hit::FingerDown(_) => {
                    // Focus this text field
                    self.focused_text_field_idx = Some(idx);
                    if let Some((_, _, current_value)) = self.text_field_data.get(idx) {
                        self.text_input_buffer = current_value.clone();
                        self.cursor_pos = self.text_input_buffer.len();
                    }
                    cx.set_key_focus(self.area);
                    needs_redraw = true;
                }
                _ => {}
            }
        }

        // Handle checkbox events
        for (idx, area) in self.checkbox_areas.iter().enumerate() {
            match event.hits(cx, *area) {
                Hit::FingerHoverIn(_) => {
                    if self.hovered_checkbox_idx != Some(idx) {
                        self.hovered_checkbox_idx = Some(idx);
                        cx.set_cursor(MouseCursor::Hand);
                        needs_redraw = true;
                    }
                }
                Hit::FingerHoverOut(_) => {
                    if self.hovered_checkbox_idx == Some(idx) {
                        self.hovered_checkbox_idx = None;
                        cx.set_cursor(MouseCursor::Default);
                        needs_redraw = true;
                    }
                }
                Hit::FingerDown(_) => {
                    // Must handle FingerDown to receive FingerUp
                    self.hovered_checkbox_idx = Some(idx);
                    needs_redraw = true;
                }
                Hit::FingerUp(fe) => {
                    if fe.is_over {
                        // Toggle checkbox value
                        if let Some((_, binding_path, current_value)) =
                            self.checkbox_data.get(idx).cloned()
                        {
                            let new_value = !current_value;
                            if let Some(path) = binding_path {
                                cx.widget_action(
                                    self.widget_uid(),
                                    &scope.path,
                                    A2uiSurfaceAction::DataModelChanged {
                                        surface_id: surface_id.clone(),
                                        path,
                                        value: serde_json::Value::Bool(new_value),
                                    },
                                );
                            }
                        }
                        needs_redraw = true;
                    }
                }
                _ => {}
            }
        }

        // Handle audio player events
        for (idx, area) in self.audio_player_areas.iter().enumerate() {
            match event.hits(cx, *area) {
                Hit::FingerHoverIn(_) => {
                    if self.hovered_audio_player_idx != Some(idx) {
                        self.hovered_audio_player_idx = Some(idx);
                        cx.set_cursor(MouseCursor::Hand);
                        needs_redraw = true;
                    }
                }
                Hit::FingerHoverOut(_) => {
                    if self.hovered_audio_player_idx == Some(idx) {
                        self.hovered_audio_player_idx = None;
                        cx.set_cursor(MouseCursor::Default);
                        needs_redraw = true;
                    }
                }
                Hit::FingerDown(_) => {
                    log!("[AudioPlayer] Click idx={}", idx);
                    self.hovered_audio_player_idx = Some(idx);
                    // Trigger play immediately on FingerDown
                    if let Some((component_id, url, title)) = self.audio_player_data.get(idx).cloned() {
                        log!("[AudioPlayer] Emitting PlayAudio: {} - {}", title, url);
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            A2uiSurfaceAction::PlayAudio {
                                component_id,
                                url,
                                title,
                            },
                        );
                    }
                    needs_redraw = true;
                }
                _ => {}
            }
        }

        // Handle slider events
        for (idx, area) in self.slider_areas.iter().enumerate() {
            match event.hits(cx, *area) {
                Hit::FingerHoverIn(_) => {
                    if self.hovered_slider_idx != Some(idx) {
                        self.hovered_slider_idx = Some(idx);
                        cx.set_cursor(MouseCursor::Hand);
                        needs_redraw = true;
                    }
                }
                Hit::FingerHoverOut(_) => {
                    if self.hovered_slider_idx == Some(idx) && self.dragging_slider_idx != Some(idx)
                    {
                        self.hovered_slider_idx = None;
                        cx.set_cursor(MouseCursor::Default);
                        needs_redraw = true;
                    }
                }
                Hit::FingerDown(fe) => {
                    self.dragging_slider_idx = Some(idx);
                    self.hovered_slider_idx = Some(idx);

                    // Calculate value from position
                    if let Some((_, binding_path, min, max, _)) = self.slider_data.get(idx).cloned()
                    {
                        let rect = area.rect(cx);
                        let rel_x = (fe.abs.x - rect.pos.x) / rect.size.x;
                        let new_value = min + (max - min) * rel_x.clamp(0.0, 1.0);

                        if let Some(path) = binding_path {
                            cx.widget_action(
                                self.widget_uid(),
                                &scope.path,
                                A2uiSurfaceAction::DataModelChanged {
                                    surface_id: surface_id.clone(),
                                    path,
                                    value: serde_json::json!(new_value),
                                },
                            );
                        }
                    }
                    needs_redraw = true;
                }
                Hit::FingerMove(fe) => {
                    if self.dragging_slider_idx == Some(idx) {
                        if let Some((_, binding_path, min, max, _)) =
                            self.slider_data.get(idx).cloned()
                        {
                            let rect = area.rect(cx);
                            let rel_x = (fe.abs.x - rect.pos.x) / rect.size.x;
                            let new_value = min + (max - min) * rel_x.clamp(0.0, 1.0);

                            if let Some(path) = binding_path {
                                cx.widget_action(
                                    self.widget_uid(),
                                    &scope.path,
                                    A2uiSurfaceAction::DataModelChanged {
                                        surface_id: surface_id.clone(),
                                        path,
                                        value: serde_json::json!(new_value),
                                    },
                                );
                            }
                        }
                        needs_redraw = true;
                    }
                }
                Hit::FingerUp(_) => {
                    if self.dragging_slider_idx == Some(idx) {
                        self.dragging_slider_idx = None;
                        needs_redraw = true;
                    }
                }
                _ => {}
            }
        }

        if needs_redraw {
            self.redraw(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Load image textures if not loaded yet
        self.load_image_textures(cx);

        // Clear component data from previous frame
        // Keep areas - they will be updated in render_* to maintain event tracking
        self.button_data.clear();
        self.text_field_data.clear();
        self.checkbox_data.clear();
        self.slider_data.clear();
        self.audio_player_data.clear();

        self.draw_bg.begin(cx, walk, self.layout);

        // Get surface and data model - clone to avoid borrow issues
        let surface_id = self.get_surface_id();
        let render_data = if let Some(processor) = &self.processor {
            let surface_opt = processor.get_surface(&surface_id);
            let data_model_opt = processor.get_data_model(&surface_id);

            // Debug: Log what we found
            if surface_opt.is_none() {
                log!("[draw_walk] No surface found for id: {}", surface_id);
            }
            if data_model_opt.is_none() {
                log!("[draw_walk] No data model found for id: {}", surface_id);
            }

            if let (Some(surface), Some(data_model)) = (surface_opt, data_model_opt) {
                log!("[draw_walk] Found surface with root: {}, {} components", surface.root, surface.components.len());
                Some((surface.clone(), data_model.clone()))
            } else {
                None
            }
        } else {
            log!("[draw_walk] No processor!");
            None
        };

        // Render the component tree
        if let Some((surface, data_model)) = render_data {
            let root_id = surface.root.clone();
            if !root_id.is_empty() {
                self.render_component(cx, scope, &surface, &data_model, &root_id);
            }
        }

        // Trim areas if we have fewer components this frame
        let current_button_count = self.button_data.len();
        if current_button_count < self.button_areas.len() {
            self.button_areas.truncate(current_button_count);
        }

        let current_text_field_count = self.text_field_data.len();
        if current_text_field_count < self.text_field_areas.len() {
            self.text_field_areas.truncate(current_text_field_count);
        }

        let current_checkbox_count = self.checkbox_data.len();
        if current_checkbox_count < self.checkbox_areas.len() {
            self.checkbox_areas.truncate(current_checkbox_count);
        }

        let current_slider_count = self.slider_data.len();
        if current_slider_count < self.slider_areas.len() {
            self.slider_areas.truncate(current_slider_count);
        }

        let current_audio_player_count = self.audio_player_data.len();
        if current_audio_player_count < self.audio_player_areas.len() {
            self.audio_player_areas.truncate(current_audio_player_count);
        }

        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();

        DrawStep::done()
    }
}
