use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // Horizontal Divider
    pub MpDivider = <View> {
        width: Fill,
        height: 1,
        show_bg: true,
        draw_bg: {
            color: (BORDER)
        }
    }

    // Vertical Divider
    pub MpDividerVertical = <View> {
        width: 1,
        height: Fill,
        show_bg: true,
        draw_bg: {
            color: (BORDER)
        }
    }

    // Divider with margin
    pub MpDividerWithMargin = <View> {
        width: Fill,
        height: Fit,
        margin: { top: 16, bottom: 16 }

        <View> {
            width: Fill,
            height: 1,
            show_bg: true,
            draw_bg: {
                color: (BORDER)
            }
        }
    }

    // Divider with label in center
    pub MpDividerWithLabel = {{MpDividerWithLabel}} {
        width: Fill,
        height: Fit,
        flow: Right,
        align: { y: 0.5 }

        left_line = <View> {
            width: Fill,
            height: 1,
            show_bg: true,
            draw_bg: { color: (BORDER) }
        }

        label = <Label> {
            width: Fit,
            margin: { left: 12, right: 12 }
            draw_text: {
                text_style: <THEME_FONT_REGULAR>{ font_size: 12.0 }
                color: (MUTED_FOREGROUND)
            }
            text: ""
        }

        right_line = <View> {
            width: Fill,
            height: 1,
            show_bg: true,
            draw_bg: { color: (BORDER) }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MpDividerWithLabel {
    #[deref]
    view: View,

    #[live]
    text: ArcStringMut,
}

impl Widget for MpDividerWithLabel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.text.as_ref().is_empty() {
            self.view.label(ids!(label)).set_text(cx, self.text.as_ref());
        }
        self.view.draw_walk(cx, scope, walk)
    }
}
