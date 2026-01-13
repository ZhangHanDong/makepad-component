use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // Radio button component
    pub MpRadio = {{MpRadio}} {
        width: Fit,
        height: Fit,
        flow: Right,
        spacing: 8,
        align: { y: 0.5 }

        // Radio circle
        radio_circle = <View> {
            width: 18,
            height: 18,

            show_bg: true,
            draw_bg: {
                instance checked: 0.0
                instance hover: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let sz = self.rect_size;
                    let center = sz * 0.5;
                    let radius = center.x - 1.0;

                    // Outer circle
                    sdf.circle(center.x, center.y, radius);

                    // Colors
                    let bg_unchecked = #ffffff;
                    let bg_checked = #ffffff;
                    let border_unchecked = mix(#d2d8f0, #3b82f6, self.hover * 0.5);
                    let border_checked = #3b82f6;

                    let bg = mix(bg_unchecked, bg_checked, self.checked);
                    let border = mix(border_unchecked, border_checked, self.checked);

                    sdf.fill_keep(bg);
                    sdf.stroke(border, 1.5);

                    // Inner dot when checked
                    if self.checked > 0.5 {
                        let dot_radius = radius * 0.5;
                        sdf.circle(center.x, center.y, dot_radius);
                        sdf.fill(#3b82f6);
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
                color: #0f172a
            }
            text: ""
        }

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { radio_circle = { draw_bg: { hover: 0.0 } } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { radio_circle = { draw_bg: { hover: 1.0 } } }
                }
            }
            checked = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { radio_circle = { draw_bg: { checked: 0.0 } } }
                }
                on = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { radio_circle = { draw_bg: { checked: 1.0 } } }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MpRadio {
    #[deref]
    view: View,

    #[live]
    text: ArcStringMut,
    #[live]
    checked: bool,
    #[live]
    disabled: bool,
    #[live]
    value: ArcStringMut,

    #[animator]
    animator: Animator,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MpRadioAction {
    Changed(bool),
    None,
}

impl Widget for MpRadio {
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
                if fe.is_over && !self.checked {
                    // Radio can only be checked, not unchecked by clicking
                    self.checked = true;
                    self.animator_play(cx, ids!(checked.on));
                    cx.widget_action(uid, &scope.path, MpRadioAction::Changed(true));
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

        // Sync initial checked state
        if self.checked {
            self.view.view(ids!(radio_circle)).apply_over(cx, live! {
                draw_bg: { checked: 1.0 }
            });
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpRadio {
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn value(&self) -> &str {
        self.value.as_ref()
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
            if let MpRadioAction::Changed(checked) = action.cast() {
                return Some(checked);
            }
        }
        None
    }
}

impl MpRadioRef {
    pub fn is_checked(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.checked
        } else {
            false
        }
    }

    pub fn value(&self) -> String {
        if let Some(inner) = self.borrow() {
            inner.value.as_ref().to_string()
        } else {
            String::new()
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
