use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    pub MpLink = {{MpLink}} {
        width: Fit,
        height: Fit,

        draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
            color: (PRIMARY)

            fn get_color(self) -> vec4 {
                return mix(
                    mix(self.color, self.color * 0.8, self.hover),
                    self.color * 0.6,
                    self.pressed
                );
            }
        }

        cursor: Hand

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_text: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_text: { hover: 1.0 } }
                }
            }
            pressed = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_text: { pressed: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_text: { pressed: 1.0 } }
                }
            }
        }

        text: ""
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MpLink {
    #[redraw]
    #[live]
    draw_text: DrawText,

    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    #[live]
    text: ArcStringMut,

    #[live]
    href: ArcStringMut,

    #[live(false)]
    disabled: bool,

    #[animator]
    animator: Animator,

    #[rust]
    area: Area,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum MpLinkAction {
    Clicked,
    None,
}

impl Widget for MpLink {
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
            }
            Hit::FingerUp(fe) => {
                self.animator_play(cx, ids!(pressed.off));
                if fe.is_over {
                    let href = self.href.as_ref();
                    if !href.is_empty() {
                        cx.open_url(href, OpenUrlInPlace::No);
                    }
                    cx.widget_action(uid, &scope.path, MpLinkAction::Clicked);
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), self.text.as_ref());
        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

impl MpLink {
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(action) = actions.find_widget_action(self.widget_uid()) {
            matches!(action.cast::<MpLinkAction>(), MpLinkAction::Clicked)
        } else {
            false
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text.as_mut_empty().push_str(text);
    }

    pub fn set_href(&mut self, href: &str) {
        self.href.as_mut_empty().push_str(href);
    }
}

impl MpLinkRef {
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

    pub fn set_href(&self, href: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_href(href);
        }
    }
}
