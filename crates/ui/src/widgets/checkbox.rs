use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // Checkbox component
    pub MpCheckbox = {{MpCheckbox}} {
        width: Fit,
        height: Fit,
        flow: Right,
        spacing: 8,
        align: { y: 0.5 }

        // Checkbox box with checkmark
        checkbox_box = <View> {
            width: 18,
            height: 18,

            show_bg: true,
            draw_bg: {
                instance checked: 0.0
                instance hover: 0.0
                instance radius: 4.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let sz = self.rect_size;

                    // Background box
                    sdf.box(1.0, 1.0, sz.x - 2.0, sz.y - 2.0, self.radius);

                    // Colors
                    let bg_unchecked = #ffffff;
                    let bg_checked = (PRIMARY);
                    let border_unchecked = mix((BORDER), (PRIMARY), self.hover * 0.5);
                    let border_checked = (PRIMARY);

                    // Interpolate based on checked state
                    let bg = mix(bg_unchecked, bg_checked, self.checked);
                    let border = mix(border_unchecked, border_checked, self.checked);

                    sdf.fill_keep(bg);
                    sdf.stroke(border, 1.5);

                    // Draw checkmark when checked
                    if self.checked > 0.5 {
                        let check_color = #ffffff;
                        let cx = sz.x * 0.5;
                        let cy = sz.y * 0.5;

                        // Checkmark path (two lines)
                        sdf.move_to(cx - 4.0, cy);
                        sdf.line_to(cx - 1.0, cy + 3.0);
                        sdf.line_to(cx + 4.0, cy - 3.0);
                        sdf.stroke(check_color, 2.0);
                    }

                    return sdf.result;
                }
            }
        }

        // Label
        label = <Label> {
            width: Fit,
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{ font_size: 14.0 }
                color: (FOREGROUND)
            }
            text: ""
        }

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { checkbox_box = { draw_bg: { hover: 0.0 } } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { checkbox_box = { draw_bg: { hover: 1.0 } } }
                }
            }
            checked = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { checkbox_box = { draw_bg: { checked: 0.0 } } }
                }
                on = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { checkbox_box = { draw_bg: { checked: 1.0 } } }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MpCheckbox {
    #[deref]
    view: View,

    #[live]
    text: ArcStringMut,
    #[live]
    checked: bool,
    #[live]
    disabled: bool,

    #[animator]
    animator: Animator,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MpCheckboxAction {
    Changed(bool),
    None,
}

impl Widget for MpCheckbox {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        self.view.handle_event(cx, event, scope);

        if self.disabled {
            return;
        }

        match event.hits(cx, self.view.area()) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Default);
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    self.checked = !self.checked;

                    if self.checked {
                        self.animator_play(cx, ids!(checked.on));
                    } else {
                        self.animator_play(cx, ids!(checked.off));
                    }

                    cx.widget_action(uid, &scope.path, MpCheckboxAction::Changed(self.checked));
                    self.redraw(cx);
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Update label text
        if !self.text.as_ref().is_empty() {
            self.view.label(ids!(label)).set_text(cx, self.text.as_ref());
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpCheckbox {
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_checked(&mut self, cx: &mut Cx, checked: bool) {
        self.checked = checked;
        if checked {
            self.animator_play(cx, ids!(checked.on));
        } else {
            self.animator_play(cx, ids!(checked.off));
        }
        self.redraw(cx);
    }

    pub fn changed(&self, actions: &Actions) -> Option<bool> {
        if let Some(action) = actions.find_widget_action(self.widget_uid()) {
            if let MpCheckboxAction::Changed(checked) = action.cast() {
                return Some(checked);
            }
        }
        None
    }
}

impl MpCheckboxRef {
    pub fn is_checked(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.checked
        } else {
            false
        }
    }

    pub fn set_checked(&self, cx: &mut Cx, checked: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_checked(cx, checked);
        }
    }

    pub fn changed(&self, actions: &Actions) -> Option<bool> {
        if let Some(inner) = self.borrow() {
            inner.changed(actions)
        } else {
            None
        }
    }
}
