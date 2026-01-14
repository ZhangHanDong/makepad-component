use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // Base button component
    pub MpButton = {{MpButton}} {
        width: Fit,
        height: Fit,
        align: { x: 0.5, y: 0.5 }
        padding: { left: 16, right: 16, top: 8, bottom: 8 }

        draw_bg: {
            instance radius: 6.0
            instance border_width: 1.0
            instance border_color: #0000
            instance hover: 0.0
            instance pressed: 0.0
            instance color: (PRIMARY)
            instance color_hover: (PRIMARY_HOVER)
            instance color_pressed: (PRIMARY_ACTIVE)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    max(1.0, self.radius)
                );

                let bg_color = mix(self.color, self.color_hover, self.hover);
                let final_color = mix(bg_color, self.color_pressed, self.pressed);

                sdf.fill_keep(final_color);

                if self.border_width > 0.0 {
                    sdf.stroke(self.border_color, self.border_width);
                }

                return sdf.result;
            }
        }

        draw_text: {
            text_style: <THEME_FONT_BOLD>{ font_size: 14.0 }
            color: (PRIMARY_FOREGROUND)
        }

        text: ""

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

    // Variant: Primary Button (default)
    pub MpButtonPrimary = <MpButton> {
        draw_bg: {
            color: (PRIMARY)
            color_hover: (PRIMARY_HOVER)
            color_pressed: (PRIMARY_ACTIVE)
        }
        draw_text: {
            color: (PRIMARY_FOREGROUND)
        }
    }

    // Variant: Secondary Button
    pub MpButtonSecondary = <MpButton> {
        draw_bg: {
            color: (SECONDARY)
            color_hover: (SECONDARY_HOVER)
            color_pressed: (SECONDARY_ACTIVE)
            border_color: (BORDER)
        }
        draw_text: {
            color: (SECONDARY_FOREGROUND)
        }
    }

    // Variant: Danger Button
    pub MpButtonDanger = <MpButton> {
        draw_bg: {
            color: (DANGER)
            color_hover: (DANGER_HOVER)
            color_pressed: (DANGER_ACTIVE)
        }
        draw_text: {
            color: (DANGER_FOREGROUND)
        }
    }

    // Variant: Ghost Button
    pub MpButtonGhost = <MpButton> {
        draw_bg: {
            color: (TRANSPARENT)
            color_hover: (SECONDARY)
            color_pressed: (SECONDARY_ACTIVE)
        }
        draw_text: {
            color: (FOREGROUND)
        }
    }

    // Size: Small
    pub MpButtonSmall = <MpButton> {
        padding: { left: 12, right: 12, top: 4, bottom: 4 }
        draw_text: {
            text_style: <THEME_FONT_BOLD>{ font_size: 12.0 }
        }
    }

    // Size: Large
    pub MpButtonLarge = <MpButton> {
        padding: { left: 24, right: 24, top: 12, bottom: 12 }
        draw_text: {
            text_style: <THEME_FONT_BOLD>{ font_size: 16.0 }
        }
    }
}

// Rust implementation
#[derive(Live, LiveHook, Widget)]
pub struct MpButton {
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
    #[live]
    disabled: bool,

    #[animator]
    animator: Animator,

    #[rust]
    area: Area,
}

// Custom Actions
#[derive(Clone, Debug, DefaultNone)]
pub enum MpButtonAction {
    Clicked,
    Pressed,
    Released,
    None,
}

impl Widget for MpButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        if self.disabled {
            return;
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
                cx.widget_action(uid, &scope.path, MpButtonAction::Pressed);
            }
            Hit::FingerUp(fe) => {
                self.animator_play(cx, ids!(pressed.off));
                if fe.is_over {
                    cx.widget_action(uid, &scope.path, MpButtonAction::Clicked);
                }
                cx.widget_action(uid, &scope.path, MpButtonAction::Released);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();
        DrawStep::done()
    }
}

impl MpButton {
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(action) = actions.find_widget_action(self.widget_uid()) {
            matches!(action.cast::<MpButtonAction>(), MpButtonAction::Clicked)
        } else {
            false
        }
    }

    pub fn pressed(&self, actions: &Actions) -> bool {
        if let Some(action) = actions.find_widget_action(self.widget_uid()) {
            matches!(action.cast::<MpButtonAction>(), MpButtonAction::Pressed)
        } else {
            false
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text.as_mut_empty().push_str(text);
    }
}

impl MpButtonRef {
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            inner.clicked(actions)
        } else {
            false
        }
    }

    pub fn set_text(&self, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_text(text);
        }
    }
}
