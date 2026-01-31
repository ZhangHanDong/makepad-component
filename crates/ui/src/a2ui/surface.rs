//! A2UI Surface Widget
//!
//! The A2uiSurface widget is the root container for rendering A2UI component trees.
//! It manages the A2uiMessageProcessor and dynamically renders components.

use makepad_widgets::*;

use super::{
    data_model::DataModel,
    message::*,
    processor::{resolve_string_value, A2uiMessageProcessor, ProcessorEvent},
};

// ============================================================================
// A2UI Surface Actions
// ============================================================================

/// Actions emitted by A2uiSurface widget
#[derive(Clone, Debug, DefaultNone)]
pub enum A2uiSurfaceAction {
    None,
    /// User triggered an action (e.g., button click)
    UserAction(UserAction),
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // A2UI Surface - Root container for A2UI component rendering
    pub A2uiSurface = {{A2uiSurface}} {
        width: Fill
        height: Fill
        flow: Down

        draw_bg: {
            instance bg_color: #1a1a2e

            fn pixel(self) -> vec4 {
                return self.bg_color;
            }
        }

        // Text rendering settings (for text outside cards)
        draw_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
                line_spacing: 1.4
            }
            color: #FFFFFF
        }

        // Text rendering for content inside cards (separate draw item)
        draw_card_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
                line_spacing: 1.4
            }
            color: #FFFFFF
        }

        // Card background
        draw_card: {
            color: #2a3a5a
            instance border_color: #5588bb
            instance border_radius: 8.0
            instance border_width: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    max(1.0, self.border_radius)
                );
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, self.border_width);
                return sdf.result;
            }
        }

        // Button background with rounded corners
        draw_button: {
            instance border_radius: 6.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, self.border_radius);
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        // Text rendering for button labels (drawn after button background)
        draw_button_text: {
            text_style: <THEME_FONT_BOLD> {
                font_size: 14.0
                line_spacing: 1.4
            }
            color: #FFFFFF
        }
    }

    // A2UI Text component
    pub A2uiText = {{A2uiText}} {
        width: Fit
        height: Fit

        draw_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
                line_spacing: 1.4
            }
            color: (FOREGROUND)
        }
    }

    // A2UI Column layout
    pub A2uiColumn = {{A2uiColumn}} {
        width: Fill
        height: Fit
        flow: Down
        spacing: 8.0
    }

    // A2UI Row layout
    pub A2uiRow = {{A2uiRow}} {
        width: Fill
        height: Fit
        flow: Right
        spacing: 8.0
        align: { y: 0.5 }
    }

    // A2UI Card container
    pub A2uiCard = {{A2uiCard}} {
        width: Fill
        height: Fit
        flow: Down
        padding: 16.0
        margin: { top: 4.0, bottom: 4.0 }

        show_bg: true
        draw_bg: {
            instance radius: 8.0
            instance border_width: 1.0
            instance border_color: (BORDER)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    max(1.0, self.radius)
                );

                sdf.fill_keep(#FFFFFF);
                sdf.stroke(self.border_color, self.border_width);

                return sdf.result;
            }
        }
    }

    // A2UI Button
    pub A2uiButton = {{A2uiButton}} {
        width: Fit
        height: Fit
        align: { x: 0.5, y: 0.5 }
        padding: { left: 16, right: 16, top: 8, bottom: 8 }

        draw_bg: {
            instance radius: 6.0
            instance hover: 0.0
            instance pressed: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, self.radius);

                let base_color = vec4(0.231, 0.51, 0.965, 1.0);
                let hover_color = vec4(0.145, 0.388, 0.922, 1.0);
                let pressed_color = vec4(0.114, 0.306, 0.847, 1.0);

                let color = mix(base_color, hover_color, self.hover);
                let final_color = mix(color, pressed_color, self.pressed);

                sdf.fill(final_color);
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
            color: #FFFFFF
        }

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_bg: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_bg: { hover: 1.0 } }
                }
            }
            pressed = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { pressed: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { pressed: 1.0 } }
                }
            }
        }
    }
}

// ============================================================================
// A2UI Surface Widget
// ============================================================================

/// The root container for rendering A2UI component trees.
#[derive(Live, LiveHook, Widget)]
pub struct A2uiSurface {
    #[redraw]
    #[live]
    draw_bg: DrawQuad,

    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    /// Draw text for rendering text components (outside cards)
    #[live]
    draw_text: DrawText,

    /// Draw text for content inside cards (separate draw item for correct z-order)
    #[live]
    draw_card_text: DrawText,

    /// Draw card background
    #[redraw]
    #[live]
    draw_card: DrawColor,

    /// Draw button background (with rounded corners shader)
    #[redraw]
    #[live]
    draw_button: DrawColor,

    /// Draw text for button labels (drawn after button background)
    #[live]
    draw_button_text: DrawText,

    /// Surface ID
    #[live]
    surface_id: LiveValue,

    /// The message processor (manages surfaces and data models)
    #[rust]
    processor: Option<A2uiMessageProcessor>,

    #[rust]
    area: Area,

    /// Flag to track if we're inside a card context (for correct text draw ordering)
    #[rust]
    inside_card: bool,

    /// Flag to track if we're inside a button context
    #[rust]
    inside_button: bool,

    /// Button areas for event.hits() detection - each button has independent Area
    #[rust]
    button_areas: Vec<Area>,

    /// Button metadata: (component_id, Option<ActionDefinition>)
    #[rust]
    button_data: Vec<(String, Option<ActionDefinition>)>,

    /// Currently hovered button index (only one at a time)
    #[rust]
    hovered_button_idx: Option<usize>,

    /// Currently pressed button index (only one at a time)
    #[rust]
    pressed_button_idx: Option<usize>,
}

impl A2uiSurface {
    /// Initialize the surface with a processor
    pub fn init_processor(&mut self) {
        if self.processor.is_none() {
            self.processor = Some(A2uiMessageProcessor::with_standard_catalog());
        }
    }

    /// Get the processor
    pub fn processor(&self) -> Option<&A2uiMessageProcessor> {
        self.processor.as_ref()
    }

    /// Get mutable processor
    pub fn processor_mut(&mut self) -> Option<&mut A2uiMessageProcessor> {
        self.processor.as_mut()
    }

    /// Process A2UI JSON messages
    pub fn process_json(&mut self, json: &str) -> Result<Vec<ProcessorEvent>, serde_json::Error> {
        self.init_processor();
        if let Some(processor) = self.processor.as_mut() {
            processor.process_json(json)
        } else {
            Ok(vec![])
        }
    }

    /// Process a single A2UI message
    pub fn process_message(&mut self, message: A2uiMessage) -> Vec<ProcessorEvent> {
        self.init_processor();
        if let Some(processor) = self.processor.as_mut() {
            processor.process_message(message)
        } else {
            vec![]
        }
    }

    /// Get the current surface ID
    fn get_surface_id(&self) -> String {
        // For now, use "main" as default
        "main".to_string()
    }
}

impl Widget for A2uiSurface {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Skip if no buttons registered
        if self.button_areas.is_empty() {
            return;
        }

        let mut needs_redraw = false;

        // Use Makepad's standard event.hits() pattern for each button's Area
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
                            if let Some((component_id, action_def)) = self.button_data.get(idx) {
                                if let Some(action_def) = action_def {
                                    // Create resolved UserAction with data model values
                                    let surface_id = self.get_surface_id();
                                    if let Some(processor) = &self.processor {
                                        let user_action = processor.create_action(
                                            &surface_id,
                                            component_id,
                                            action_def,
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

        if needs_redraw {
            self.redraw(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Clear button data from previous frame
        // Keep button_areas - they will be updated in render_button to maintain event tracking
        self.button_data.clear();

        self.draw_bg.begin(cx, walk, self.layout);

        // Get surface and data model - clone to avoid borrow issues
        let surface_id = self.get_surface_id();
        let render_data = if let Some(processor) = &self.processor {
            if let Some(surface) = processor.get_surface(&surface_id) {
                if let Some(data_model) = processor.get_data_model(&surface_id) {
                    Some((surface.clone(), data_model.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        // Render the component tree
        if let Some((surface, data_model)) = render_data {
            let root_id = surface.root.clone();
            if !root_id.is_empty() {
                self.render_component(cx, scope, &surface, &data_model, &root_id);
            }
        }

        // Trim button_areas if we have fewer buttons this frame
        let current_button_count = self.button_data.len();
        if current_button_count < self.button_areas.len() {
            self.button_areas.truncate(current_button_count);
        }

        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();

        DrawStep::done()
    }
}

impl A2uiSurface {
    /// Render a component and its children recursively
    fn render_component(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &super::processor::Surface,
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
            _ => {
                // Unsupported component - skip for now
            }
        }
    }

    fn render_column(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &super::processor::Surface,
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
        surface: &super::processor::Surface,
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
        surface: &super::processor::Surface,
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
        surface: &super::processor::Surface,
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
        surface: &super::processor::Surface,
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
        surface: &super::processor::Surface,
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
        surface: &super::processor::Surface,
        data_model: &DataModel,
        component_id: &str,
        _item_path: &str,
    ) {
        // TODO: Set up scoped data model for template items
        self.render_component(cx, scope, surface, data_model, component_id);
    }

    fn render_text(&mut self, cx: &mut Cx2d, text: &TextComponent, data_model: &DataModel) {
        let text_value = resolve_string_value(&text.text, data_model);


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

    fn render_card(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &super::processor::Surface,
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
        surface: &super::processor::Surface,
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


        // Store button metadata
        self.button_data.push((
            component_id.to_string(),
            btn.action.clone(),
        ));
    }
}

impl A2uiSurfaceRef {
    /// Process A2UI JSON messages
    pub fn process_json(&self, json: &str) -> Result<Vec<ProcessorEvent>, serde_json::Error> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.process_json(json)
        } else {
            Ok(vec![])
        }
    }

    /// Process a single A2UI message
    pub fn process_message(&self, message: A2uiMessage) -> Vec<ProcessorEvent> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.process_message(message)
        } else {
            vec![]
        }
    }

    /// Check if any user action was triggered
    /// Returns the UserAction if one was triggered
    pub fn user_action(&self, actions: &Actions) -> Option<UserAction> {
        if let Some(inner) = self.borrow() {
            if let Some(action) = actions.find_widget_action(inner.widget_uid()) {
                if let A2uiSurfaceAction::UserAction(user_action) =
                    action.cast::<A2uiSurfaceAction>()
                {
                    return Some(user_action);
                }
            }
        }
        None
    }

    /// Check if a specific action was triggered by name
    /// Returns the context HashMap if the action matches
    pub fn action_by_name(
        &self,
        actions: &Actions,
        action_name: &str,
    ) -> Option<std::collections::HashMap<String, serde_json::Value>> {
        if let Some(user_action) = self.user_action(actions) {
            if user_action.action.name == action_name {
                return Some(user_action.action.context);
            }
        }
        None
    }
}

// ============================================================================
// A2UI Text Widget
// ============================================================================

/// A2UI Text component
#[derive(Live, LiveHook, Widget)]
pub struct A2uiText {
    #[redraw]
    #[live]
    draw_text: DrawText,

    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    #[live]
    text: ArcStringMut,

    #[rust]
    area: Area,
}

impl Widget for A2uiText {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        self.draw_text
            .draw_walk(cx, Walk::fit(), Align::default(), self.text.as_ref());
        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

impl A2uiText {
    pub fn set_text(&mut self, text: &str) {
        self.text.as_mut_empty().push_str(text);
    }
}

// ============================================================================
// A2UI Column Widget
// ============================================================================

/// A2UI Column layout component
#[derive(Live, LiveHook, Widget)]
pub struct A2uiColumn {
    #[deref]
    view: View,
}

impl Widget for A2uiColumn {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// ============================================================================
// A2UI Row Widget
// ============================================================================

/// A2UI Row layout component
#[derive(Live, LiveHook, Widget)]
pub struct A2uiRow {
    #[deref]
    view: View,
}

impl Widget for A2uiRow {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// ============================================================================
// A2UI Card Widget
// ============================================================================

/// A2UI Card container component
#[derive(Live, LiveHook, Widget)]
pub struct A2uiCard {
    #[deref]
    view: View,

    #[live]
    draw_bg: DrawQuad,
}

impl Widget for A2uiCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// ============================================================================
// A2UI Button Widget
// ============================================================================

/// A2UI Button component with action support
#[derive(Live, LiveHook, Widget)]
pub struct A2uiButton {
    #[redraw]
    #[live]
    draw_bg: DrawQuad,

    #[live]
    draw_text: DrawText,

    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    #[live]
    text: ArcStringMut,

    #[animator]
    animator: Animator,

    /// The action definition from A2UI
    #[rust]
    action_def: Option<ActionDefinition>,

    #[rust]
    area: Area,
}

/// Actions emitted by A2uiButton
#[derive(Clone, Debug, DefaultNone)]
pub enum A2uiButtonAction {
    Clicked {
        action_name: String,
        component_id: String,
    },
    None,
}

impl Widget for A2uiButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Default);
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerDown(_) => {
                self.animator_play(cx, ids!(pressed.on));
            }
            Hit::FingerUp(fe) => {
                self.animator_play(cx, ids!(pressed.off));
                if fe.is_over {
                    // Emit action
                    if let Some(action_def) = &self.action_def {
                        cx.widget_action(
                            uid,
                            &scope.path,
                            A2uiButtonAction::Clicked {
                                action_name: action_def.name.clone(),
                                component_id: String::new(), // TODO: get from context
                            },
                        );
                    }
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text
            .draw_walk(cx, Walk::fit(), Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();
        DrawStep::done()
    }
}

impl A2uiButton {
    pub fn set_text(&mut self, text: &str) {
        self.text.as_mut_empty().push_str(text);
    }

    pub fn set_action(&mut self, action_def: ActionDefinition) {
        self.action_def = Some(action_def);
    }

    pub fn clicked(&self, actions: &Actions) -> Option<(String, String)> {
        if let Some(action) = actions.find_widget_action(self.widget_uid()) {
            if let A2uiButtonAction::Clicked {
                action_name,
                component_id,
            } = action.cast::<A2uiButtonAction>()
            {
                return Some((action_name, component_id));
            }
        }
        None
    }
}

impl A2uiButtonRef {
    pub fn set_text(&self, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_text(text);
        }
    }

    pub fn clicked(&self, actions: &Actions) -> Option<(String, String)> {
        if let Some(inner) = self.borrow() {
            inner.clicked(actions)
        } else {
            None
        }
    }
}
