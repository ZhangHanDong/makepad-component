use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================================
    // MpNotification - Toast/notification component
    // ============================================================

    // Base notification
    MpNotificationBase = <View> {
        width: 320
        height: Fit
        padding: 16
        flow: Right
        spacing: 12
        align: { y: 0.0 }

        show_bg: true
        draw_bg: {
            instance bg_color: (CARD)
            instance border_radius: 8.0
            instance border_color: (BORDER)
            instance shadow_color: #0000001A
            instance shadow_offset_y: 4.0
            instance shadow_blur: 12.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Shadow
                sdf.box(
                    0.0,
                    self.shadow_offset_y,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.border_radius
                );
                sdf.blur = self.shadow_blur;
                sdf.fill(self.shadow_color);
                sdf.blur = 0.0;

                // Main card
                sdf.box(
                    0.5,
                    0.5,
                    self.rect_size.x - 1.0,
                    self.rect_size.y - 1.0,
                    self.border_radius
                );
                sdf.fill_keep(self.bg_color);
                sdf.stroke(self.border_color, 1.0);

                return sdf.result;
            }
        }
    }

    // ============================================================
    // Default Notification
    // ============================================================

    pub MpNotification = <MpNotificationBase> {
        icon = <View> {
            width: 20
            height: 20
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance icon_color: (MUTED_FOREGROUND)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let r = min(c.x, c.y) - 1.0;

                    // Info icon (circle with i)
                    sdf.circle(c.x, c.y, r);
                    sdf.stroke(self.icon_color, 1.5);

                    // Letter i
                    sdf.circle(c.x, c.y - 3.0, 1.5);
                    sdf.fill(self.icon_color);
                    sdf.rect(c.x - 1.0, c.y, 2.0, 5.0);
                    sdf.fill(self.icon_color);

                    return sdf.result;
                }
            }
        }

        content = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 4

            title = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: (FOREGROUND)
                }
                text: "Notification"
            }

            message = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: ""
            }
        }

        close = <View> {
            width: 20
            height: 20
            cursor: Hand
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance icon_color: #94a3b8
                instance hover: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let size = 5.0;

                    let final_color = mix(self.icon_color, #64748b, self.hover);

                    // X mark
                    sdf.move_to(c.x - size, c.y - size);
                    sdf.line_to(c.x + size, c.y + size);
                    sdf.stroke(final_color, 1.5);

                    sdf.move_to(c.x + size, c.y - size);
                    sdf.line_to(c.x - size, c.y + size);
                    sdf.stroke(final_color, 1.5);

                    return sdf.result;
                }
            }

            animator: {
                hover = {
                    default: off
                    off = {
                        from: { all: Forward { duration: 0.15 } }
                        apply: { draw_bg: { hover: 0.0 } }
                    }
                    on = {
                        from: { all: Forward { duration: 0.1 } }
                        apply: { draw_bg: { hover: 1.0 } }
                    }
                }
            }
        }
    }

    // ============================================================
    // Success Notification
    // ============================================================

    pub MpNotificationSuccess = <MpNotificationBase> {
        draw_bg: {
            border_color: #bbf7d0
        }

        icon = <View> {
            width: 20
            height: 20
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance icon_color: (SUCCESS)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;

                    // Checkmark
                    sdf.move_to(c.x - 5.0, c.y);
                    sdf.line_to(c.x - 1.0, c.y + 4.0);
                    sdf.line_to(c.x + 6.0, c.y - 4.0);
                    sdf.stroke(self.icon_color, 2.0);

                    return sdf.result;
                }
            }
        }

        content = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 4

            title = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: (SUCCESS)
                }
                text: "Success"
            }

            message = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: ""
            }
        }

        close = <View> {
            width: 20
            height: 20
            cursor: Hand
        }
    }

    // ============================================================
    // Error Notification
    // ============================================================

    pub MpNotificationError = <MpNotificationBase> {
        draw_bg: {
            border_color: #fecaca
        }

        icon = <View> {
            width: 20
            height: 20
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance icon_color: (DANGER)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let r = min(c.x, c.y) - 1.0;

                    // Circle
                    sdf.circle(c.x, c.y, r);
                    sdf.stroke(self.icon_color, 1.5);

                    // X mark
                    let size = 4.0;
                    sdf.move_to(c.x - size, c.y - size);
                    sdf.line_to(c.x + size, c.y + size);
                    sdf.stroke(self.icon_color, 1.5);

                    sdf.move_to(c.x + size, c.y - size);
                    sdf.line_to(c.x - size, c.y + size);
                    sdf.stroke(self.icon_color, 1.5);

                    return sdf.result;
                }
            }
        }

        content = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 4

            title = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: (DANGER)
                }
                text: "Error"
            }

            message = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: ""
            }
        }

        close = <View> {
            width: 20
            height: 20
            cursor: Hand
        }
    }

    // ============================================================
    // Warning Notification
    // ============================================================

    pub MpNotificationWarning = <MpNotificationBase> {
        draw_bg: {
            border_color: #fde68a
        }

        icon = <View> {
            width: 20
            height: 20
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance icon_color: (WARNING)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;

                    // Triangle
                    sdf.move_to(c.x, 2.0);
                    sdf.line_to(self.rect_size.x - 2.0, self.rect_size.y - 2.0);
                    sdf.line_to(2.0, self.rect_size.y - 2.0);
                    sdf.close_path();
                    sdf.stroke(self.icon_color, 1.5);

                    // Exclamation mark
                    sdf.rect(c.x - 1.0, 7.0, 2.0, 5.0);
                    sdf.fill(self.icon_color);
                    sdf.circle(c.x, 15.0, 1.5);
                    sdf.fill(self.icon_color);

                    return sdf.result;
                }
            }
        }

        content = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 4

            title = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: #b45309
                }
                text: "Warning"
            }

            message = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: ""
            }
        }

        close = <View> {
            width: 20
            height: 20
            cursor: Hand
        }
    }

    // ============================================================
    // Info Notification
    // ============================================================

    pub MpNotificationInfo = <MpNotificationBase> {
        draw_bg: {
            border_color: #a5f3fc
        }

        icon = <View> {
            width: 20
            height: 20
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance icon_color: (INFO)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let r = min(c.x, c.y) - 1.0;

                    // Circle
                    sdf.circle(c.x, c.y, r);
                    sdf.stroke(self.icon_color, 1.5);

                    // Letter i
                    sdf.circle(c.x, c.y - 3.0, 1.5);
                    sdf.fill(self.icon_color);
                    sdf.rect(c.x - 1.0, c.y, 2.0, 5.0);
                    sdf.fill(self.icon_color);

                    return sdf.result;
                }
            }
        }

        content = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 4

            title = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: (INFO)
                }
                text: "Info"
            }

            message = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: ""
            }
        }

        close = <View> {
            width: 20
            height: 20
            cursor: Hand
        }
    }

    // ============================================================
    // Notification Container (for positioning)
    // ============================================================

    pub MpNotificationContainer = <View> {
        width: Fill
        height: Fill
        flow: Overlay

        // Top-right corner positioning
        align: { x: 1.0, y: 0.0 }
        padding: 16

        notifications = <View> {
            width: Fit
            height: Fit
            flow: Down
            spacing: 8
        }
    }

    // ============================================================
    // Interactive Notification Widget
    // ============================================================

    pub MpNotificationWidget = {{MpNotificationWidget}} {
        width: 320
        height: Fit
        padding: 16
        flow: Right
        spacing: 12
        align: { y: 0.0 }
        visible: false

        show_bg: true
        draw_bg: {
            instance bg_color: (CARD)
            instance border_radius: 8.0
            instance border_color: (BORDER)
            instance shadow_color: #0000001A
            instance shadow_offset_y: 4.0
            instance shadow_blur: 12.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Shadow
                sdf.box(
                    0.0,
                    self.shadow_offset_y,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.border_radius
                );
                sdf.blur = self.shadow_blur;
                sdf.fill(self.shadow_color);
                sdf.blur = 0.0;

                // Main card
                sdf.box(
                    0.5,
                    0.5,
                    self.rect_size.x - 1.0,
                    self.rect_size.y - 1.0,
                    self.border_radius
                );
                sdf.fill_keep(self.bg_color);
                sdf.stroke(self.border_color, 1.0);

                return sdf.result;
            }
        }

        content = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 4

            title = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: (FOREGROUND)
                }
                text: "Notification"
            }

            message = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: ""
            }
        }

        close = <View> {
            width: 20
            height: 20
            cursor: Hand
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance icon_color: #94a3b8
                instance hover: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let size = 5.0;

                    let final_color = mix(self.icon_color, #64748b, self.hover);

                    sdf.move_to(c.x - size, c.y - size);
                    sdf.line_to(c.x + size, c.y + size);
                    sdf.stroke(final_color, 1.5);

                    sdf.move_to(c.x + size, c.y - size);
                    sdf.line_to(c.x - size, c.y + size);
                    sdf.stroke(final_color, 1.5);

                    return sdf.result;
                }
            }
        }
    }
}

/// Notification actions
#[derive(Clone, Debug, DefaultNone)]
pub enum MpNotificationAction {
    None,
    Closed,
}

/// Interactive notification widget
#[derive(Live, LiveHook, Widget)]
pub struct MpNotificationWidget {
    #[deref]
    view: View,

    #[live]
    visible: bool,
}

impl Widget for MpNotificationWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }

        self.view.handle_event(cx, event, scope);

        // Handle close button
        let close_btn = self.view.view(ids!(close));
        match event.hits(cx, close_btn.area()) {
            Hit::FingerHoverIn(_) => {
                close_btn.apply_over(cx, live!{ draw_bg: { hover: 1.0 } });
                close_btn.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                close_btn.apply_over(cx, live!{ draw_bg: { hover: 0.0 } });
                close_btn.redraw(cx);
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    self.visible = false;
                    self.redraw(cx);
                    cx.widget_action(self.widget_uid(), &scope.path, MpNotificationAction::Closed);
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpNotificationWidget {
    /// Show the notification
    pub fn show(&mut self, cx: &mut Cx) {
        self.visible = true;
        self.redraw(cx);
    }

    /// Hide the notification
    pub fn hide(&mut self, cx: &mut Cx) {
        self.visible = false;
        self.redraw(cx);
    }

    /// Set the notification title
    pub fn set_title(&mut self, cx: &mut Cx, title: &str) {
        self.view.label(ids!(content.title)).set_text(cx, title);
    }

    /// Set the notification message
    pub fn set_message(&mut self, cx: &mut Cx, message: &str) {
        self.view.label(ids!(content.message)).set_text(cx, message);
    }

    /// Show with title and message
    pub fn show_message(&mut self, cx: &mut Cx, title: &str, message: &str) {
        self.set_title(cx, title);
        self.set_message(cx, message);
        self.show(cx);
    }
}

impl MpNotificationWidgetRef {
    pub fn show(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show(cx);
        }
    }

    pub fn hide(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hide(cx);
        }
    }

    pub fn set_title(&self, cx: &mut Cx, title: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_title(cx, title);
        }
    }

    pub fn set_message(&self, cx: &mut Cx, message: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_message(cx, message);
        }
    }

    pub fn show_message(&self, cx: &mut Cx, title: &str, message: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show_message(cx, title, message);
        }
    }
}
