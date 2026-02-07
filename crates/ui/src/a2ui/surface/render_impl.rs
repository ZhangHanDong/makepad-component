impl A2uiSurface {
    /// Render a component and its children recursively
    fn render_component(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &crate::a2ui::processor::Surface,
        data_model: &DataModel,
        component_id: &str,
    ) {
        let Some(component_def) = surface.get_component(component_id) else {
            return;
        };

        // Clone component data to avoid borrow issues
        let component = component_def.component.clone();

        match &component {
            ComponentType::Column(col) => {
                self.render_column(cx, scope, surface, data_model, col);
            }
            ComponentType::Row(row) => {
                self.render_row(cx, scope, surface, data_model, row);
            }
            ComponentType::Text(text) => {
                self.render_text(cx, text, data_model);
            }
            ComponentType::Card(card) => {
                self.render_card(cx, scope, surface, data_model, card);
            }
            ComponentType::Button(btn) => {
                self.render_button(cx, scope, surface, data_model, btn, component_id);
            }
            ComponentType::Image(img) => {
                self.render_image(cx, img, data_model);
            }
            ComponentType::TextField(text_field) => {
                self.render_text_field(cx, text_field, data_model, component_id);
            }
            ComponentType::CheckBox(checkbox) => {
                self.render_checkbox(cx, checkbox, data_model, component_id);
            }
            ComponentType::Slider(slider) => {
                self.render_slider(cx, slider, data_model, component_id);
            }
            ComponentType::List(list) => {
                self.render_list(cx, scope, surface, data_model, list);
            }
            ComponentType::Chart(chart) => {
                self.render_chart(cx, scope, chart, data_model, component_id);
            }
            ComponentType::AudioPlayer(audio_player) => {
                self.render_audio_player(cx, audio_player, data_model, component_id);
            }
            _ => {
                // Unsupported component - skip for now
            }
        }
    }

    fn render_column(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &crate::a2ui::processor::Surface,
        data_model: &DataModel,
        col: &ColumnComponent,
    ) {
        // Start a vertical layout
        let walk = Walk::fill_fit();
        let layout = Layout {
            flow: Flow::Down,
            spacing: 8.0,
            ..Layout::default()
        };

        cx.begin_turtle(walk, layout);

        // Render children
        let children = col.children.clone();
        self.render_children(cx, scope, surface, data_model, &children);

        cx.end_turtle();
    }

    fn render_row(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &crate::a2ui::processor::Surface,
        data_model: &DataModel,
        row: &RowComponent,
    ) {
        // Start a horizontal layout - Fill width to allow spacer pattern
        let walk = Walk::fill_fit();
        let layout = Layout {
            flow: Flow::right(),
            spacing: 16.0,
            align: Align { x: 0.0, y: 0.5 },
            ..Layout::default()
        };

        cx.begin_turtle(walk, layout);

        // Render children with special handling for Row context
        let children = row.children.clone();
        self.render_row_children(cx, scope, surface, data_model, &children);

        cx.end_turtle();
    }

    /// Render children specifically for Row context (horizontal layout)
    /// If last child is a Button, it's placed in a Fill-width container with right alignment
    fn render_row_children(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &crate::a2ui::processor::Surface,
        data_model: &DataModel,
        children: &ChildrenRef,
    ) {
        match children {
            ChildrenRef::ExplicitList(ids) => {
                let len = ids.len();

                // Check if last child is a Button for right-alignment
                let last_is_button = if len > 0 {
                    if let Some(comp) = surface.get_component(&ids[len - 1]) {
                        matches!(comp.component, ComponentType::Button(_))
                    } else {
                        false
                    }
                } else {
                    false
                };

                if last_is_button && len > 1 {
                    // Render non-button children with fixed min-width for alignment
                    // 280px is enough for longest product name
                    for child_id in ids.iter().take(len - 1) {
                        self.render_row_child_with_min_width(cx, scope, surface, data_model, child_id, 280.0);
                    }

                    // Render button
                    self.render_row_child(cx, scope, surface, data_model, &ids[len - 1]);
                } else {
                    // Render all children normally
                    for child_id in ids.iter() {
                        self.render_row_child(cx, scope, surface, data_model, child_id);
                    }
                }
            }
            ChildrenRef::Template { .. } => {
                // For templates in Row, use regular rendering
                self.render_children(cx, scope, surface, data_model, children);
            }
        }
    }

    /// Render a single child in Row context
    fn render_row_child(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &crate::a2ui::processor::Surface,
        data_model: &DataModel,
        component_id: &str,
    ) {
        self.render_row_child_with_min_width(cx, scope, surface, data_model, component_id, 0.0);
    }

    /// Render a single child in Row context with minimum width for Column alignment
    fn render_row_child_with_min_width(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &crate::a2ui::processor::Surface,
        data_model: &DataModel,
        component_id: &str,
        min_width: f64,
    ) {
        let Some(component_def) = surface.get_component(component_id) else {
            return;
        };

        let component = component_def.component.clone();

        match &component {
            ComponentType::Column(col) => {
                // Column with fixed width ensures buttons align
                // Height is Fit to adapt to content
                let walk = if min_width > 0.0 {
                    // Fixed width, Fit height using Walk::new()
                    Walk::new(Size::Fixed(min_width), Size::fit())
                } else {
                    Walk::fit()
                };
                let layout = Layout {
                    flow: Flow::Down,
                    spacing: 4.0,
                    ..Layout::default()
                };

                cx.begin_turtle(walk, layout);

                // Render Column children
                if let ChildrenRef::ExplicitList(ids) = &col.children {
                    for child_id in ids {
                        self.render_component(cx, scope, surface, data_model, child_id);
                    }
                }

                cx.end_turtle();
            }
            _ => {
                // Other components render normally
                self.render_component(cx, scope, surface, data_model, component_id);
            }
        }
    }

    fn render_children(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &crate::a2ui::processor::Surface,
        data_model: &DataModel,
        children: &ChildrenRef,
    ) {
        match children {
            ChildrenRef::ExplicitList(ids) => {
                let ids_clone = ids.clone();
                for child_id in ids_clone {
                    self.render_component(cx, scope, surface, data_model, &child_id);
                }
            }
            ChildrenRef::Template {
                component_id,
                data_binding,
            } => {
                // Get array data from data model
                if let Some(array) = data_model.get_array(data_binding) {
                    let component_id = component_id.clone();
                    let data_binding = data_binding.clone();
                    for (index, _item) in array.iter().enumerate() {
                        // For template rendering, we need to set up item context
                        // For now, just render the template component
                        let item_path = format!("{}/{}", data_binding, index);
                        self.render_template_item(
                            cx,
                            scope,
                            surface,
                            data_model,
                            &component_id,
                            &item_path,
                        );
                    }
                }
            }
        }
    }

    fn render_template_item(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &crate::a2ui::processor::Surface,
        data_model: &DataModel,
        component_id: &str,
        item_path: &str,
    ) {
        // Set up scoped data model for template items
        // Save previous scope and set new one
        let previous_scope = self.current_scope.take();
        self.current_scope = Some(item_path.to_string());

        // Render the component with scoped path resolution
        self.render_component(cx, scope, surface, data_model, component_id);

        // Restore previous scope
        self.current_scope = previous_scope;
    }

    fn render_text(&mut self, cx: &mut Cx2d, text: &TextComponent, data_model: &DataModel) {
        // Use scoped resolution for template rendering
        let text_value = resolve_string_value_scoped(
            &text.text,
            data_model,
            self.current_scope.as_deref(),
        );



        // Determine font size based on usage hint
        let font_size = match text.usage_hint {
            Some(TextUsageHint::H1) => 28.0,
            Some(TextUsageHint::H2) => 22.0,
            Some(TextUsageHint::H3) => 18.0,
            Some(TextUsageHint::H4) => 16.0,
            Some(TextUsageHint::H5) => 14.0,
            Some(TextUsageHint::Caption) => 12.0,
            Some(TextUsageHint::Code) => 13.0,
            _ => 14.0, // Body default
        };

        // Use different DrawText based on context for correct z-ordering:
        // - Text inside button uses draw_button_text (drawn after draw_button)
        // - Text inside card uses draw_card_text (drawn after draw_card)
        // - Text outside both uses draw_text
        if self.inside_button {
            self.draw_button_text.text_style.font_size = font_size;
            self.draw_button_text.draw_walk(cx, Walk::fit(), Align::default(), &text_value);
        } else if self.inside_card {
            self.draw_card_text.text_style.font_size = font_size;
            self.draw_card_text.draw_walk(cx, Walk::fit(), Align::default(), &text_value);
        } else {
            self.draw_text.text_style.font_size = font_size;
            self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), &text_value);
        }
    }

    fn render_image(&mut self, cx: &mut Cx2d, img: &ImageComponent, data_model: &DataModel) {
        // Use scoped resolution for template rendering
        let url = resolve_string_value_scoped(
            &img.url,
            data_model,
            self.current_scope.as_deref(),
        );

        // Determine size based on usage hint
        let (width, height) = match img.usage_hint {
            Some(ImageUsageHint::Icon) => (24.0, 24.0),
            Some(ImageUsageHint::Avatar) => (48.0, 48.0),
            Some(ImageUsageHint::SmallFeature) => (64.0, 64.0),
            Some(ImageUsageHint::MediumFeature) => (120.0, 80.0),
            Some(ImageUsageHint::LargeFeature) => (200.0, 150.0),
            Some(ImageUsageHint::Header) => (300.0, 100.0),
            _ => (80.0, 80.0), // Default size
        };

        let walk = Walk::new(Size::Fixed(width), Size::Fixed(height));

        // Get texture index (avoid borrow conflict)
        let texture_idx = self.get_texture_index_for_url(&url);

        // Try to render actual image if texture is available
        if let Some(idx) = texture_idx {
            // Get texture reference by index
            let texture = match idx {
                0 => self.texture_headphones.as_ref(),
                1 => self.texture_mouse.as_ref(),
                2 => self.texture_keyboard.as_ref(),
                3 => self.texture_alipay.as_ref(),
                4 => self.texture_wechat.as_ref(),
                _ => None,
            };

            if let Some(tex) = texture {
                // Draw actual image with texture
                self.draw_image.draw_vars.set_texture(0, tex);
                self.draw_image.draw_walk(cx, walk);
                return;
            }
        }

        // Fallback to placeholder
        let layout = Layout {
            padding: Padding {
                left: 4.0,
                right: 4.0,
                top: 4.0,
                bottom: 4.0,
            },
            align: Align { x: 0.5, y: 0.5 },
            ..Layout::default()
        };

        self.draw_image_placeholder.begin(cx, walk, layout);
        self.draw_image_text.draw_walk(cx, Walk::fit(), Align::default(), "IMG");
        self.draw_image_placeholder.end(cx);
    }

    fn render_card(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &crate::a2ui::processor::Surface,
        data_model: &DataModel,
        card: &CardComponent,
    ) {
        // Use the standard Makepad pattern: begin/end with draw_bg
        // The key is that begin() adds background instance, then children are drawn, then end() finalizes
        let walk = Walk {
            margin: Margin { left: 0.0, right: 0.0, top: 8.0, bottom: 8.0 },
            ..Walk::fill_fit()
        };
        let layout = Layout {
            flow: Flow::Down,
            padding: Padding {
                left: 16.0,
                right: 16.0,
                top: 12.0,
                bottom: 12.0,
            },
            ..Layout::default()
        };


        // Begin card - this adds background instance and starts turtle
        self.draw_card.begin(cx, walk, layout);

        // Set flag to use card text (which will be drawn AFTER the card background)
        self.inside_card = true;

        // Render child content
        let child = card.child.clone();
        self.render_component(cx, scope, surface, data_model, &child);

        // Reset flag
        self.inside_card = false;

        // End card
        self.draw_card.end(cx);

    }

    fn render_button(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &crate::a2ui::processor::Surface,
        data_model: &DataModel,
        btn: &ButtonComponent,
        component_id: &str,
    ) {
        // Get button index (this is the button we're about to render)
        let button_idx = self.button_data.len();

        // Get button state (hover/pressed) for this specific button
        let is_hover = self.hovered_button_idx == Some(button_idx);
        let is_pressed = self.pressed_button_idx == Some(button_idx);

        // Set button color based on state
        let base_color = vec4(0.231, 0.51, 0.965, 1.0);     // #3B82F6 - blue
        let hover_color = vec4(0.145, 0.388, 0.922, 1.0);   // #2563EB - darker blue
        let pressed_color = vec4(0.114, 0.306, 0.847, 1.0); // #1D4ED8 - even darker

        let color = if is_pressed {
            pressed_color
        } else if is_hover {
            hover_color
        } else {
            base_color
        };

        // Button layout with padding - this ensures text has proper spacing
        let layout = Layout {
            padding: Padding {
                left: 16.0,
                right: 16.0,
                top: 8.0,
                bottom: 8.0,
            },
            align: Align { x: 0.5, y: 0.5 },
            ..Layout::default()
        };

        // Record starting position before drawing
        let start_pos = cx.turtle().pos();

        // Draw button background with proper padding
        self.draw_button.color = color;
        self.draw_button.begin(cx, Walk::fit(), layout);

        // Set flag to use button text (drawn after button background)
        self.inside_button = true;

        // Render button child (usually Text)
        let child = btn.child.clone();
        self.render_component(cx, scope, surface, data_model, &child);

        // Reset flag
        self.inside_button = false;

        // End button background
        self.draw_button.end(cx);

        // Calculate button rect from start position and current turtle position
        let end_pos = cx.turtle().pos();
        // For Flow::Right, the width is the difference in x, height needs to be calculated
        // Use the used rect from turtle
        let used_rect = cx.turtle().used();
        let button_rect = Rect {
            pos: start_pos,
            size: dvec2(end_pos.x - start_pos.x, used_rect.y),
        };

        // Update or create Area for this button using add_rect_area
        // Reuse existing Area if available to maintain event tracking across frames
        if button_idx < self.button_areas.len() {
            // Update existing area
            cx.add_rect_area(&mut self.button_areas[button_idx], button_rect);
        } else {
            // Create new area
            let mut button_area = Area::Empty;
            cx.add_rect_area(&mut button_area, button_rect);
            self.button_areas.push(button_area);
        }


        // Store button metadata including template scope for action context resolution
        self.button_data.push((
            component_id.to_string(),
            btn.action.clone(),
            self.current_scope.clone(),
        ));
    }

    // ============================================================================
    // TextField Rendering
    // ============================================================================

    fn render_text_field(
        &mut self,
        cx: &mut Cx2d,
        text_field: &TextFieldComponent,
        data_model: &DataModel,
        component_id: &str,
    ) {
        let text_field_idx = self.text_field_data.len();
        let is_focused = self.focused_text_field_idx == Some(text_field_idx);

        // Get current value - use input buffer if focused, otherwise from data model
        let current_value = if is_focused {
            self.text_input_buffer.clone()
        } else {
            resolve_string_value_scoped(&text_field.text, data_model, self.current_scope.as_deref())
        };

        // Get placeholder text
        let placeholder = text_field
            .placeholder
            .as_ref()
            .map(|p| resolve_string_value_scoped(p, data_model, self.current_scope.as_deref()))
            .unwrap_or_default();

        // Get binding path for two-way binding
        let binding_path = text_field.text.as_path().map(|p| {
            if let Some(scope) = &self.current_scope {
                format!("{}/{}", scope, p.trim_start_matches('/'))
            } else {
                p.to_string()
            }
        });

        // Layout
        let walk = Walk {
            width: Size::Fixed(200.0),
            height: Size::Fixed(36.0),
            ..Walk::default()
        };
        let layout = Layout {
            padding: Padding {
                left: 12.0,
                right: 12.0,
                top: 8.0,
                bottom: 8.0,
            },
            align: Align { x: 0.0, y: 0.5 },
            ..Layout::default()
        };

        // Record start position
        let start_pos = cx.turtle().pos();

        // Set focus state
        self.draw_text_field.focus = if is_focused { 1.0 } else { 0.0 };

        // Draw background
        self.draw_text_field.begin(cx, walk, layout);

        // Draw text or placeholder
        if current_value.is_empty() && !is_focused {
            self.draw_text_field_placeholder
                .draw_walk(cx, Walk::fit(), Align::default(), &placeholder);
        } else {
            // Draw text with cursor if focused
            if is_focused {
                // Draw text before cursor
                let (before, after) = current_value.split_at(self.cursor_pos.min(current_value.len()));
                self.draw_text_field_text
                    .draw_walk(cx, Walk::fit(), Align::default(), before);
                // Draw cursor (simple vertical line approximation using |)
                self.draw_text_field_text
                    .draw_walk(cx, Walk::fit(), Align::default(), "|");
                self.draw_text_field_text
                    .draw_walk(cx, Walk::fit(), Align::default(), after);
            } else {
                self.draw_text_field_text
                    .draw_walk(cx, Walk::fit(), Align::default(), &current_value);
            }
        }

        self.draw_text_field.end(cx);

        // Calculate rect for hit testing (using fixed size)
        let rect = Rect {
            pos: start_pos,
            size: dvec2(200.0, 36.0),
        };

        // Update or create area
        if text_field_idx < self.text_field_areas.len() {
            cx.add_rect_area(&mut self.text_field_areas[text_field_idx], rect);
        } else {
            let mut area = Area::Empty;
            cx.add_rect_area(&mut area, rect);
            self.text_field_areas.push(area);
        }

        // Store metadata
        self.text_field_data.push((
            component_id.to_string(),
            binding_path,
            current_value,
        ));
    }

    // ============================================================================
    // CheckBox Rendering
    // ============================================================================

    fn render_checkbox(
        &mut self,
        cx: &mut Cx2d,
        checkbox: &CheckBoxComponent,
        data_model: &DataModel,
        component_id: &str,
    ) {
        let checkbox_idx = self.checkbox_data.len();
        let is_hovered = self.hovered_checkbox_idx == Some(checkbox_idx);

        // Get current checked state
        let is_checked =
            resolve_boolean_value_scoped(&checkbox.value, data_model, self.current_scope.as_deref());

        // Get label text
        let label = checkbox
            .label
            .as_ref()
            .map(|l| resolve_string_value_scoped(l, data_model, self.current_scope.as_deref()))
            .unwrap_or_default();

        // Get binding path
        let binding_path = checkbox.value.as_path().map(|p| {
            if let Some(scope) = &self.current_scope {
                format!("{}/{}", scope, p.trim_start_matches('/'))
            } else {
                p.to_string()
            }
        });

        // Record start position
        let start_pos = cx.turtle().pos();

        // Draw checkbox row
        let row_walk = Walk::fit();
        let row_layout = Layout {
            flow: Flow::right(),
            spacing: 8.0,
            align: Align { x: 0.0, y: 0.5 },
            ..Layout::default()
        };

        cx.begin_turtle(row_walk, row_layout);

        // Draw checkbox box
        let checkbox_walk = Walk {
            width: Size::Fixed(20.0),
            height: Size::Fixed(20.0),
            ..Walk::default()
        };

        self.draw_checkbox.checked = if is_checked { 1.0 } else { 0.0 };
        self.draw_checkbox.hover = if is_hovered { 1.0 } else { 0.0 };
        self.draw_checkbox.draw_walk(cx, checkbox_walk);

        // Draw label
        if !label.is_empty() {
            if self.inside_card {
                self.draw_card_text
                    .draw_walk(cx, Walk::fit(), Align::default(), &label);
            } else {
                self.draw_checkbox_label
                    .draw_walk(cx, Walk::fit(), Align::default(), &label);
            }
        }

        // Get the used rect before ending turtle
        let used = cx.turtle().used();
        cx.end_turtle();

        // Calculate rect for hit testing using the actual used space
        // Ensure minimum clickable area: 200px wide, 28px high
        let rect = Rect {
            pos: start_pos,
            size: dvec2(used.x.max(200.0), used.y.max(28.0)),
        };

        // Update or create area
        if checkbox_idx < self.checkbox_areas.len() {
            cx.add_rect_area(&mut self.checkbox_areas[checkbox_idx], rect);
        } else {
            let mut area = Area::Empty;
            cx.add_rect_area(&mut area, rect);
            self.checkbox_areas.push(area);
        }

        // Store metadata
        self.checkbox_data
            .push((component_id.to_string(), binding_path, is_checked));
    }

    // ============================================================================
    // Slider Rendering
    // ============================================================================

    fn render_slider(
        &mut self,
        cx: &mut Cx2d,
        slider: &SliderComponent,
        data_model: &DataModel,
        component_id: &str,
    ) {
        let slider_idx = self.slider_data.len();
        let _is_hovered = self.hovered_slider_idx == Some(slider_idx);
        let _is_dragging = self.dragging_slider_idx == Some(slider_idx);

        // Get values
        let current_value =
            resolve_number_value_scoped(&slider.value, data_model, self.current_scope.as_deref());
        let min = slider.min.unwrap_or(0.0);
        let max = slider.max.unwrap_or(100.0);

        // Calculate progress (0.0 to 1.0)
        let progress = if max > min {
            ((current_value - min) / (max - min)).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // Get binding path
        let binding_path = slider.value.as_path().map(|p| {
            if let Some(scope) = &self.current_scope {
                format!("{}/{}", scope, p.trim_start_matches('/'))
            } else {
                p.to_string()
            }
        });

        // Record start position
        let start_pos = cx.turtle().pos();

        // Slider dimensions
        let slider_width = 200.0;
        let track_height = 6.0;
        let thumb_size = 18.0;

        // Draw slider container
        let container_walk = Walk {
            width: Size::Fixed(slider_width),
            height: Size::Fixed(thumb_size),
            ..Walk::default()
        };
        let container_layout = Layout {
            align: Align { x: 0.0, y: 0.5 },
            ..Layout::default()
        };

        cx.begin_turtle(container_walk, container_layout);

        // Draw track
        let track_walk = Walk {
            width: Size::Fixed(slider_width),
            height: Size::Fixed(track_height),
            margin: Margin {
                top: (thumb_size - track_height) / 2.0,
                ..Margin::default()
            },
            ..Walk::default()
        };

        self.draw_slider_track.progress = progress as f32;
        self.draw_slider_track.draw_walk(cx, track_walk);

        cx.end_turtle();

        // Draw thumb (overlay at correct position)
        // Note: For proper overlay we'd need absolute positioning
        // For now, we'll use a simpler approach

        // Calculate rect for hit testing (the entire slider area)
        let rect = Rect {
            pos: start_pos,
            size: dvec2(slider_width, thumb_size),
        };

        // Update or create area
        if slider_idx < self.slider_areas.len() {
            cx.add_rect_area(&mut self.slider_areas[slider_idx], rect);
        } else {
            let mut area = Area::Empty;
            cx.add_rect_area(&mut area, rect);
            self.slider_areas.push(area);
        }

        // Store metadata
        self.slider_data.push((
            component_id.to_string(),
            binding_path,
            min,
            max,
            current_value,
        ));
    }

    // ============================================================================
    // List Rendering
    // ============================================================================

    fn render_list(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &crate::a2ui::processor::Surface,
        data_model: &DataModel,
        list: &ListComponent,
    ) {
        // For now, render List similar to Column
        // TODO: Implement PortalList for virtualized scrolling
        let walk = Walk::fill_fit();
        let layout = Layout {
            flow: Flow::Down,
            spacing: 8.0,
            ..Layout::default()
        };

        cx.begin_turtle(walk, layout);

        // Render children (supports template binding)
        let children = list.children.clone();
        self.render_children(cx, scope, surface, data_model, &children);

        cx.end_turtle();
    }

    // ============================================================================
    // Chart Rendering
    // ============================================================================

    /// Default color palette for charts
    fn chart_palette(index: usize) -> Vec4 {
        const COLORS: &[(f32, f32, f32)] = &[
            (0.231, 0.510, 0.965),  // #3B82F6 blue
            (0.161, 0.714, 0.467),  // #28B677 green
            (0.937, 0.333, 0.314),  // #EF5550 red
            (0.969, 0.643, 0.176),  // #F7A42D orange
            (0.545, 0.361, 0.886),  // #8B5CE2 purple
            (0.071, 0.741, 0.812),  // #12BDD0 teal
            (0.957, 0.486, 0.667),  // #F47CAA pink
            (0.400, 0.553, 0.200),  // #668D33 olive
        ];
        let (r, g, b) = COLORS[index % COLORS.len()];
        Vec4 { x: r, y: g, z: b, w: 1.0 }
    }

    /// Parse a hex color string to Vec4
    fn parse_hex_color(hex: &str) -> Option<Vec4> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 { return None; }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
        Some(Vec4 { x: r, y: g, z: b, w: 1.0 })
    }

    fn get_chart_color(&self, chart: &ChartComponent, index: usize) -> Vec4 {
        if index < chart.colors.len() {
            if let Some(color) = Self::parse_hex_color(&chart.colors[index]) {
                return color;
            }
        }
        Self::chart_palette(index)
    }

    /// Estimate text width in pixels for chart layout
    fn estimate_text_width(text: &str, font_size: f64) -> f64 {
        let avg_char_width = font_size * 0.55;
        text.len() as f64 * avg_char_width
    }
}
