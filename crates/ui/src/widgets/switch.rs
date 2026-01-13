use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Switch toggle component
    pub MpSwitch = {{MpSwitch}} {
        width: 44,
        height: 24,
        flow: Overlay,

        // Track background (capsule shape)
        track = <View> {
            width: Fill,
            height: Fill,
            show_bg: true,
            draw_bg: {
                instance on: 0.0
                instance hover: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let sz = self.rect_size;
                    let r = sz.y * 0.5;

                    // Draw capsule: left circle + rectangle + right circle
                    sdf.circle(r, r, r);
                    sdf.rect(r, 0.0, sz.x - sz.y, sz.y);
                    sdf.circle(sz.x - r, r, r);

                    let bg_off = #cbd5e1;
                    let bg_on = #3b82f6;
                    let color = mix(bg_off, bg_on, self.on);
                    let color = mix(color, #ffffff, self.hover * 0.1);

                    sdf.fill(color);
                    return sdf.result;
                }
            }
        }

        // Thumb container (for positioning)
        thumb_wrap = <View> {
            width: Fill,
            height: Fill,
            align: { x: 0.0, y: 0.5 }
            padding: { left: 3, right: 3 }

            thumb = <View> {
                width: 18,
                height: 18,
                show_bg: true,
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let radius = self.rect_size.y * 0.5;

                        sdf.circle(radius, radius, radius);
                        sdf.fill(#ffffff);

                        return sdf.result;
                    }
                }
            }
        }

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { track = { draw_bg: { hover: 0.0 } } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { track = { draw_bg: { hover: 1.0 } } }
                }
            }
            on = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.2 } }
                    apply: {
                        track = { draw_bg: { on: 0.0 } }
                        thumb_wrap = { align: { x: 0.0 } }
                    }
                }
                on = {
                    from: { all: Forward { duration: 0.2 } }
                    apply: {
                        track = { draw_bg: { on: 1.0 } }
                        thumb_wrap = { align: { x: 1.0 } }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MpSwitch {
    #[deref]
    view: View,

    #[live]
    on: bool,
    #[live]
    disabled: bool,

    #[animator]
    animator: Animator,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MpSwitchAction {
    Changed(bool),
    None,
}

impl Widget for MpSwitch {
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
                    self.on = !self.on;

                    if self.on {
                        self.animator_play(cx, ids!(on.on));
                    } else {
                        self.animator_play(cx, ids!(on.off));
                    }

                    cx.widget_action(uid, &scope.path, MpSwitchAction::Changed(self.on));
                    self.redraw(cx);
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Sync initial state
        if self.on {
            self.view.view(ids!(track)).apply_over(cx, live! {
                draw_bg: { on: 1.0 }
            });
            self.view.view(ids!(thumb_wrap)).apply_over(cx, live! {
                align: { x: 1.0 }
            });
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpSwitch {
    pub fn is_on(&self) -> bool {
        self.on
    }

    pub fn set_on(&mut self, cx: &mut Cx, on: bool) {
        self.on = on;
        if on {
            self.animator_play(cx, ids!(on.on));
        } else {
            self.animator_play(cx, ids!(on.off));
        }
        self.redraw(cx);
    }

    pub fn changed(&self, actions: &Actions) -> Option<bool> {
        if let Some(action) = actions.find_widget_action(self.widget_uid()) {
            if let MpSwitchAction::Changed(on) = action.cast() {
                return Some(on);
            }
        }
        None
    }
}

impl MpSwitchRef {
    pub fn is_on(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.on
        } else {
            false
        }
    }

    pub fn set_on(&self, cx: &mut Cx, on: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_on(cx, on);
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
