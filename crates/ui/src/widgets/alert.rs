use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // Alert constants
    ALERT_PADDING_X = 16.0
    ALERT_PADDING_Y = 10.0
    ALERT_GAP = 12.0
    ALERT_RADIUS = 6.0
    ALERT_BORDER_WIDTH = 1.0

    // Icon size
    ALERT_ICON_SIZE = 20.0

    // Base Alert component
    MpAlertBase = {{MpAlert}} {
        width: Fill,
        height: Fit,
        flow: Right,
        spacing: (ALERT_GAP),
        padding: { left: (ALERT_PADDING_X), right: (ALERT_PADDING_X), top: (ALERT_PADDING_Y), bottom: (ALERT_PADDING_Y) }

        show_bg: true,
        draw_bg: {
            instance bg_color: #F3F4F6
            instance border_color: #E5E7EB
            instance border_width: (ALERT_BORDER_WIDTH)
            instance radius: (ALERT_RADIUS)

            fn get_color(self) -> vec4 {
                return self.bg_color
            }

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - 2.0 * self.border_width,
                    self.rect_size.y - 2.0 * self.border_width,
                    max(1.0, self.radius)
                );
                sdf.fill_keep(self.bg_color);
                sdf.stroke(self.border_color, self.border_width);
                return sdf.result;
            }
        }

        // Icon container
        icon_container = <View> {
            width: Fit,
            height: Fit,
            align: { x: 0.0, y: 0.0 }

            icon = <Icon> {
                width: (ALERT_ICON_SIZE),
                height: (ALERT_ICON_SIZE),
                draw_icon: {
                    svg_file: dep("crate://self/resources/icons/info.svg"),
                    fn get_color(self) -> vec4 {
                        return #3B82F6;
                    }
                }
            }
        }

        // Content container
        content_container = <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 4.0

            // Title (optional)
            title = <Label> {
                width: Fill,
                height: Fit,
                visible: false,
                draw_text: {
                    text_style: <THEME_FONT_BOLD> {
                        font_size: 14.0
                    }
                    color: #1F2937
                }
                text: ""
            }

            // Message
            message = <Label> {
                width: Fill,
                height: Fit,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> {
                        font_size: 14.0
                    }
                    color: #374151
                }
                text: ""
            }
        }

        // Close button (optional)
        close_button = <View> {
            width: Fit,
            height: Fit,
            visible: false,
            padding: 4.0
            cursor: Hand,

            show_bg: true,
            draw_bg: {
                instance bg_color: #00000000
                instance hover_color: #00000010
                instance pressed_color: #00000020
                instance radius: 4.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        0.0,
                        0.0,
                        self.rect_size.x,
                        self.rect_size.y,
                        self.radius
                    );
                    sdf.fill(self.bg_color);
                    return sdf.result;
                }
            }

            close_icon = <Icon> {
                width: 16.0,
                height: 16.0,
                draw_icon: {
                    svg_file: dep("crate://self/resources/icons/close.svg"),
                    fn get_color(self) -> vec4 {
                        return #6B7280;
                    }
                }
            }
        }
    }

    // Info Alert (blue)
    pub MpAlert = <MpAlertBase> {
        draw_bg: {
            bg_color: #EFF6FF
            border_color: #3B82F6
        }
        icon_container = {
            icon = {
                draw_icon: {
                    fn get_color(self) -> vec4 {
                        return #3B82F6;
                    }
                }
            }
        }
    }

    // Success Alert (green)
    pub MpAlertSuccess = <MpAlertBase> {
        draw_bg: {
            bg_color: #F0FDF4
            border_color: #22C55E
        }
        icon_container = {
            icon = {
                draw_icon: {
                    svg_file: dep("crate://self/resources/icons/check-circle.svg"),
                    fn get_color(self) -> vec4 {
                        return #22C55E;
                    }
                }
            }
        }
    }

    // Warning Alert (yellow/orange)
    pub MpAlertWarning = <MpAlertBase> {
        draw_bg: {
            bg_color: #FFFBEB
            border_color: #F59E0B
        }
        icon_container = {
            icon = {
                draw_icon: {
                    svg_file: dep("crate://self/resources/icons/alert-triangle.svg"),
                    fn get_color(self) -> vec4 {
                        return #F59E0B;
                    }
                }
            }
        }
    }

    // Error Alert (red)
    pub MpAlertError = <MpAlertBase> {
        draw_bg: {
            bg_color: #FEF2F2
            border_color: #EF4444
        }
        icon_container = {
            icon = {
                draw_icon: {
                    svg_file: dep("crate://self/resources/icons/x-circle.svg"),
                    fn get_color(self) -> vec4 {
                        return #EF4444;
                    }
                }
            }
        }
    }

    // Secondary Alert (gray)
    pub MpAlertSecondary = <MpAlertBase> {
        draw_bg: {
            bg_color: #F9FAFB
            border_color: #D1D5DB
        }
        icon_container = {
            icon = {
                draw_icon: {
                    fn get_color(self) -> vec4 {
                        return #6B7280;
                    }
                }
            }
        }
    }
}

/// Alert widget for displaying messages to users
#[derive(Live, Widget)]
pub struct MpAlert {
    #[deref]
    view: View,

    #[live]
    message_text: String,

    #[live]
    title_text: String,

    #[live(true)]
    visible: bool,

    #[live(false)]
    closable: bool,

    #[animator]
    animator: Animator,
}

impl LiveHook for MpAlert {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.sync_display(cx);
    }
}

impl Widget for MpAlert {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Handle close button click
        if self.closable {
            if let Event::Actions(actions) = event {
                if self.view.button(ids!(close_button)).clicked(actions) {
                    self.visible = false;
                    self.sync_display(cx);
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        MpAlertAction::Closed,
                    );
                }
            }
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpAlert {
    /// Sync display state with properties
    fn sync_display(&mut self, cx: &mut Cx) {
        // Update message
        if !self.message_text.is_empty() {
            self.view
                .label(ids!(content_container.message))
                .set_text(cx, &self.message_text);
        }

        // Update title visibility and text
        let has_title = !self.title_text.is_empty();
        self.view
            .view(ids!(content_container.title))
            .set_visible(cx, has_title);
        if has_title {
            self.view
                .label(ids!(content_container.title))
                .set_text(cx, &self.title_text);
        }

        // Update close button visibility
        self.view
            .view(ids!(close_button))
            .set_visible(cx, self.closable);

        // Update overall visibility
        self.view.set_visible(cx, self.visible);
    }

    /// Set the message text
    pub fn set_message(&mut self, cx: &mut Cx, message: &str) {
        self.message_text = message.to_string();
        self.sync_display(cx);
        self.redraw(cx);
    }

    /// Set the title text
    pub fn set_title(&mut self, cx: &mut Cx, title: &str) {
        self.title_text = title.to_string();
        self.sync_display(cx);
        self.redraw(cx);
    }

    /// Set visibility
    pub fn set_visible(&mut self, cx: &mut Cx, visible: bool) {
        self.visible = visible;
        self.sync_display(cx);
        self.redraw(cx);
    }

    /// Set closable
    pub fn set_closable(&mut self, cx: &mut Cx, closable: bool) {
        self.closable = closable;
        self.sync_display(cx);
        self.redraw(cx);
    }
}

impl MpAlertRef {
    /// Set the message text
    pub fn set_message(&self, cx: &mut Cx, message: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_message(cx, message);
        }
    }

    /// Set the title text
    pub fn set_title(&self, cx: &mut Cx, title: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_title(cx, title);
        }
    }

    /// Set visibility
    pub fn set_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_visible(cx, visible);
        }
    }

    /// Set closable
    pub fn set_closable(&self, cx: &mut Cx, closable: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_closable(cx, closable);
        }
    }
}

/// Actions emitted by MpAlert
#[derive(Clone, Debug, DefaultNone)]
pub enum MpAlertAction {
    None,
    Closed,
}
