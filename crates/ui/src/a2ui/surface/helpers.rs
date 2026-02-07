use makepad_widgets::*;

use crate::a2ui::message::{ActionDefinition, UserAction};
use super::draw_types::A2uiSurfaceAction;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::theme::colors::*;

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

    pub A2uiColumn = {{A2uiColumn}} {
        width: Fill
        height: Fit
        flow: Down
        spacing: 8.0
    }

    pub A2uiRow = {{A2uiRow}} {
        width: Fill
        height: Fit
        flow: Right
        spacing: 8.0
        align: { y: 0.5 }
    }

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
