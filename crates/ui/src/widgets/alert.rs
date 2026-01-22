use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================================
    // Alert Icons - SDF drawn icons for each variant
    // ============================================================

    // Info icon (circle with i)
    MpAlertIconInfo = <View> {
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

                // Letter i - dot
                sdf.circle(c.x, c.y - 3.0, 1.5);
                sdf.fill(self.icon_color);
                // Letter i - stem
                sdf.rect(c.x - 1.0, c.y, 2.0, 5.0);
                sdf.fill(self.icon_color);

                return sdf.result;
            }
        }
    }

    // Success icon (checkmark in circle)
    MpAlertIconSuccess = <View> {
        width: 20
        height: 20
        align: { x: 0.5, y: 0.5 }

        show_bg: true
        draw_bg: {
            instance icon_color: (SUCCESS)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y) - 1.0;

                // Circle
                sdf.circle(c.x, c.y, r);
                sdf.stroke(self.icon_color, 1.5);

                // Checkmark
                sdf.move_to(c.x - 4.0, c.y);
                sdf.line_to(c.x - 1.0, c.y + 3.0);
                sdf.line_to(c.x + 5.0, c.y - 3.0);
                sdf.stroke(self.icon_color, 1.5);

                return sdf.result;
            }
        }
    }

    // Warning icon (triangle with !)
    MpAlertIconWarning = <View> {
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

                // Exclamation mark - stem
                sdf.rect(c.x - 1.0, 7.0, 2.0, 5.0);
                sdf.fill(self.icon_color);
                // Exclamation mark - dot
                sdf.circle(c.x, 15.0, 1.5);
                sdf.fill(self.icon_color);

                return sdf.result;
            }
        }
    }

    // Error icon (X in circle)
    MpAlertIconError = <View> {
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

    // Secondary icon (info style, neutral)
    MpAlertIconSecondary = <View> {
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

    // ============================================================
    // Close button
    // ============================================================

    MpAlertCloseButton = <View> {
        width: 20
        height: 20
        cursor: Hand
        align: { x: 0.5, y: 0.5 }

        show_bg: true
        draw_bg: {
            instance icon_color: #94a3b8
            instance hover: 0.0
            instance bg_hover_color: #00000010

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let size = 5.0;
                let r = min(c.x, c.y);

                // Hover background
                sdf.circle(c.x, c.y, r);
                sdf.fill(mix(#0000, self.bg_hover_color, self.hover));

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

    // ============================================================
    // Base Alert
    // ============================================================

    MpAlertBase = {{MpAlert}} {
        width: Fill
        height: Fit
        padding: { left: 16, right: 16, top: 12, bottom: 12 }
        flow: Right
        spacing: 12
        align: { y: 0.0 }

        show_bg: true
        draw_bg: {
            instance bg_color: #f1f5f920
            instance border_radius: 8.0
            instance border_color: (BORDER)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Main box with rounded corners
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

        icon = <MpAlertIconSecondary> {}

        content = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 4

            title_wrapper = <View> {
                width: Fill
                height: Fit
                visible: false

                title = <Label> {
                    width: Fill
                    height: Fit
                    draw_text: {
                        text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                        color: (FOREGROUND)
                    }
                    text: ""
                }
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

        close_button = <MpAlertCloseButton> {}
    }

    // ============================================================
    // Alert Variants
    // ============================================================

    // Default/Secondary Alert
    pub MpAlert = <MpAlertBase> {}

    // Info Alert
    pub MpAlertInfo = <MpAlertBase> {
        draw_bg: {
            bg_color: #06b6d410
            border_color: #a5f3fc
        }

        icon = <MpAlertIconInfo> {}

        content = {
            title_wrapper = {
                title = {
                    draw_text: { color: (INFO) }
                }
            }
            message = {
                draw_text: { color: #0e7490 }
            }
        }

        close_button = {
            draw_bg: {
                icon_color: #06b6d4
                bg_hover_color: #06b6d420
            }
        }
    }

    // Success Alert
    pub MpAlertSuccess = <MpAlertBase> {
        draw_bg: {
            bg_color: #16a34a10
            border_color: #bbf7d0
        }

        icon = <MpAlertIconSuccess> {}

        content = {
            title_wrapper = {
                title = {
                    draw_text: { color: (SUCCESS) }
                }
            }
            message = {
                draw_text: { color: #15803d }
            }
        }

        close_button = {
            draw_bg: {
                icon_color: #16a34a
                bg_hover_color: #16a34a20
            }
        }
    }

    // Warning Alert
    pub MpAlertWarning = <MpAlertBase> {
        draw_bg: {
            bg_color: #f59a0b10
            border_color: #fde68a
        }

        icon = <MpAlertIconWarning> {}

        content = {
            title_wrapper = {
                title = {
                    draw_text: { color: #b45309 }
                }
            }
            message = {
                draw_text: { color: #854d0e }
            }
        }

        close_button = {
            draw_bg: {
                icon_color: #f59a0b
                bg_hover_color: #f59a0b20
            }
        }
    }

    // Error/Danger Alert
    pub MpAlertError = <MpAlertBase> {
        draw_bg: {
            bg_color: #dc262610
            border_color: #fecaca
        }

        icon = <MpAlertIconError> {}

        content = {
            title_wrapper = {
                title = {
                    draw_text: { color: (DANGER) }
                }
            }
            message = {
                draw_text: { color: #b91c1c }
            }
        }

        close_button = {
            draw_bg: {
                icon_color: #dc2626
                bg_hover_color: #dc262620
            }
        }
    }

    // ============================================================
    // Banner Variants (full width, no border radius)
    // ============================================================

    MpAlertBannerBase = <MpAlertBase> {
        align: { y: 0.5 }

        draw_bg: {
            border_radius: 0.0
        }
    }

    pub MpAlertBanner = <MpAlertBannerBase> {}

    pub MpAlertBannerInfo = <MpAlertBannerBase> {
        draw_bg: {
            bg_color: #06b6d410
            border_color: #a5f3fc
        }

        icon = <MpAlertIconInfo> {}

        content = {
            message = {
                draw_text: { color: #0e7490 }
            }
        }

        close_button = {
            draw_bg: {
                icon_color: #06b6d4
                bg_hover_color: #06b6d420
            }
        }
    }

    pub MpAlertBannerSuccess = <MpAlertBannerBase> {
        draw_bg: {
            bg_color: #16a34a10
            border_color: #bbf7d0
        }

        icon = <MpAlertIconSuccess> {}

        content = {
            message = {
                draw_text: { color: #15803d }
            }
        }

        close_button = {
            draw_bg: {
                icon_color: #16a34a
                bg_hover_color: #16a34a20
            }
        }
    }

    pub MpAlertBannerWarning = <MpAlertBannerBase> {
        draw_bg: {
            bg_color: #f59a0b10
            border_color: #fde68a
        }

        icon = <MpAlertIconWarning> {}

        content = {
            message = {
                draw_text: { color: #854d0e }
            }
        }

        close_button = {
            draw_bg: {
                icon_color: #f59a0b
                bg_hover_color: #f59a0b20
            }
        }
    }

    pub MpAlertBannerError = <MpAlertBannerBase> {
        draw_bg: {
            bg_color: #dc262610
            border_color: #fecaca
        }

        icon = <MpAlertIconError> {}

        content = {
            message = {
                draw_text: { color: #b91c1c }
            }
        }

        close_button = {
            draw_bg: {
                icon_color: #dc2626
                bg_hover_color: #dc262620
            }
        }
    }
}

/// Alert action emitted when the close button is clicked
#[derive(Clone, Debug, DefaultNone)]
pub enum MpAlertAction {
    None,
    Close,
}

/// Alert widget for displaying important messages to users
#[derive(Live, Widget)]
pub struct MpAlert {
    #[deref]
    view: View,

    /// Whether the alert is visible
    #[live(true)]
    visible: bool,

    /// Whether to show the close button
    #[live(false)]
    closable: bool,
}

impl LiveHook for MpAlert {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.sync_visibility(cx);
    }
}

impl Widget for MpAlert {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        // Handle close button hover and click
        let close_button = self.view.view(ids!(close_button));
        match event.hits(cx, close_button.area()) {
            Hit::FingerHoverIn(_) => {
                close_button.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                close_button.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerDown(_) => {
                cx.widget_action(self.widget_uid(), &scope.path, MpAlertAction::Close);
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

impl MpAlert {
    /// Sync visibility of close button
    fn sync_visibility(&mut self, cx: &mut Cx) {
        self.view.view(ids!(close_button)).set_visible(cx, self.closable);
    }

    /// Set the alert title
    pub fn set_title(&mut self, cx: &mut Cx, title: &str) {
        let title_label = self.view.label(ids!(content.title_wrapper.title));
        title_label.set_text(cx, title);
        self.view.view(ids!(content.title_wrapper)).set_visible(cx, !title.is_empty());
        self.redraw(cx);
    }

    /// Set the alert message
    pub fn set_message(&mut self, cx: &mut Cx, message: &str) {
        self.view.label(ids!(content.message)).set_text(cx, message);
        self.redraw(cx);
    }

    /// Set whether the alert is visible
    pub fn set_visible(&mut self, cx: &mut Cx, visible: bool) {
        self.visible = visible;
        self.redraw(cx);
    }

    /// Get whether the alert is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Set whether the close button is shown
    pub fn set_closable(&mut self, cx: &mut Cx, closable: bool) {
        self.closable = closable;
        self.sync_visibility(cx);
        self.redraw(cx);
    }

    /// Close the alert (set visible to false)
    pub fn close(&mut self, cx: &mut Cx) {
        self.set_visible(cx, false);
    }
}

impl MpAlertRef {
    /// Set the alert title
    pub fn set_title(&self, cx: &mut Cx, title: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_title(cx, title);
        }
    }

    /// Set the alert message
    pub fn set_message(&self, cx: &mut Cx, message: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_message(cx, message);
        }
    }

    /// Set whether the alert is visible
    pub fn set_visible(&self, cx: &mut Cx, visible: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_visible(cx, visible);
        }
    }

    /// Get whether the alert is visible
    pub fn is_visible(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_visible()
        } else {
            false
        }
    }

    /// Set whether the close button is shown
    pub fn set_closable(&self, cx: &mut Cx, closable: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_closable(cx, closable);
        }
    }

    /// Close the alert
    pub fn close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.close(cx);
        }
    }

    /// Check if the close action was triggered
    pub fn closed(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast::<MpAlertAction>(), MpAlertAction::Close)
        } else {
            false
        }
    }
}
