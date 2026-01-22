use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;
    use crate::widgets::button::*;

    // ============================================================
    // MpModal - Modal/Dialog component
    // ============================================================

    // Modal backdrop (overlay)
    MpModalBackdrop = <View> {
        width: Fill
        height: Fill

        show_bg: true
        draw_bg: {
            color: #00000080

            fn pixel(self) -> vec4 {
                return self.color;
            }
        }
    }

    // Modal container (centers the dialog)
    pub MpModalContainer = <View> {
        width: Fill
        height: Fill
        flow: Overlay
        align: { x: 0.5, y: 0.5 }

        backdrop = <MpModalBackdrop> {}
    }

    // ============================================================
    // Modal Dialog
    // ============================================================

    // Base modal dialog
    pub MpModal = <View> {
        width: 400
        height: Fit
        flow: Down

        show_bg: true
        draw_bg: {
            instance bg_color: (CARD)
            instance border_radius: 12.0
            instance border_color: (BORDER)
            instance shadow_color: #00000033
            instance shadow_offset_y: 8.0
            instance shadow_blur: 24.0

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

        header = <View> {
            width: Fill
            height: Fit
            padding: { left: 24, right: 24, top: 20, bottom: 16 }
            flow: Right
            align: { y: 0.5 }

            title = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 18.0 }
                    color: (FOREGROUND)
                }
                text: "Modal Title"
            }

            close = <View> {
                width: 24
                height: 24
                cursor: Hand
                align: { x: 0.5, y: 0.5 }

                show_bg: true
                draw_bg: {
                    instance icon_color: #94a3b8
                    instance hover: 0.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let c = self.rect_size * 0.5;
                        let size = 6.0;

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

        body = <View> {
            width: Fill
            height: Fit
            padding: { left: 24, right: 24, top: 0, bottom: 16 }
            flow: Down
            spacing: 8

            <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: "Modal content goes here."
            }
        }

        footer = <View> {
            width: Fill
            height: Fit
            padding: { left: 24, right: 24, top: 16, bottom: 20 }
            flow: Right
            spacing: 8
            align: { x: 1.0, y: 0.5 }
        }
    }

    // ============================================================
    // Modal Size Variants
    // ============================================================

    pub MpModalSmall = <MpModal> {
        width: 320
    }

    pub MpModalLarge = <MpModal> {
        width: 560
    }

    pub MpModalFullWidth = <MpModal> {
        width: Fill
        margin: 24
    }

    // ============================================================
    // Alert Dialog (simple confirmation)
    // ============================================================

    pub MpAlertDialog = <MpModal> {
        width: 360

        header = <View> {
            width: Fill
            height: Fit
            padding: { left: 24, right: 24, top: 24, bottom: 12 }
            align: { x: 0.5 }

            title = <Label> {
                width: Fit
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 18.0 }
                    color: (FOREGROUND)
                }
                text: "Are you sure?"
            }
        }

        body = <View> {
            width: Fill
            height: Fit
            padding: { left: 24, right: 24, top: 0, bottom: 20 }
            align: { x: 0.5 }

            <Label> {
                width: Fit
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: "This action cannot be undone."
            }
        }

        footer = <View> {
            width: Fill
            height: Fit
            padding: { left: 24, right: 24, top: 0, bottom: 24 }
            flow: Right
            spacing: 12
            align: { x: 0.5, y: 0.5 }
        }
    }

    // ============================================================
    // Danger Alert Dialog
    // ============================================================

    pub MpAlertDialogDanger = <MpAlertDialog> {
        header = <View> {
            width: Fill
            height: Fit
            padding: { left: 24, right: 24, top: 24, bottom: 12 }
            flow: Down
            spacing: 12
            align: { x: 0.5 }

            icon = <View> {
                width: 48
                height: 48
                align: { x: 0.5, y: 0.5 }

                show_bg: true
                draw_bg: {
                    instance bg_color: #fef2f2
                    instance icon_color: (DANGER)

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        let c = self.rect_size * 0.5;
                        let r = min(c.x, c.y);

                        // Circle background
                        sdf.circle(c.x, c.y, r);
                        sdf.fill(self.bg_color);

                        // Warning icon (triangle with !)
                        let size = 12.0;
                        sdf.move_to(c.x, c.y - size + 4.0);
                        sdf.line_to(c.x + size - 2.0, c.y + size - 6.0);
                        sdf.line_to(c.x - size + 2.0, c.y + size - 6.0);
                        sdf.close_path();
                        sdf.stroke(self.icon_color, 2.0);

                        // Exclamation mark
                        sdf.rect(c.x - 1.0, c.y - 4.0, 2.0, 6.0);
                        sdf.fill(self.icon_color);
                        sdf.circle(c.x, c.y + 6.0, 1.5);
                        sdf.fill(self.icon_color);

                        return sdf.result;
                    }
                }
            }

            title = <Label> {
                width: Fit
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 18.0 }
                    color: (FOREGROUND)
                }
                text: "Delete item?"
            }
        }
    }

    // ============================================================
    // Modal header/footer components
    // ============================================================

    pub MpModalHeader = <View> {
        width: Fill
        height: Fit
        padding: { left: 24, right: 24, top: 20, bottom: 16 }
        flow: Right
        align: { y: 0.5 }
    }

    pub MpModalBody = <View> {
        width: Fill
        height: Fit
        padding: { left: 24, right: 24, top: 0, bottom: 16 }
        flow: Down
        spacing: 8
    }

    pub MpModalFooter = <View> {
        width: Fill
        height: Fit
        padding: { left: 24, right: 24, top: 16, bottom: 20 }
        flow: Right
        spacing: 8
        align: { x: 1.0, y: 0.5 }
    }

    // Divider for modal sections
    pub MpModalDivider = <View> {
        width: Fill
        height: 1
        show_bg: true
        draw_bg: {
            color: (BORDER)
        }
    }

    // ============================================================
    // Interactive Modal Widget
    // ============================================================

    pub MpModalWidget = {{MpModalWidget}} {
        width: Fill
        height: Fill
        flow: Overlay
        visible: false

        backdrop = <View> {
            width: Fill
            height: Fill
            show_bg: true
            draw_bg: { color: #00000080 }
        }

        content = <View> {
            width: Fill
            height: Fill
            align: { x: 0.5, y: 0.5 }

            dialog = <MpModal> {}
        }
    }
}

/// Modal actions
#[derive(Clone, Debug, DefaultNone)]
pub enum MpModalAction {
    None,
    Opened,
    Closed,
    CloseRequested,
}

/// Interactive modal widget with open/close functionality
#[derive(Live, LiveHook, Widget)]
pub struct MpModalWidget {
    #[deref]
    view: View,

    #[live]
    visible: bool,
}

impl Widget for MpModalWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.visible {
            return;
        }

        self.view.handle_event(cx, event, scope);

        // Handle backdrop click to close
        let backdrop = self.view.view(ids!(backdrop));
        if let Hit::FingerUp(fe) = event.hits(cx, backdrop.area()) {
            if fe.is_over {
                cx.widget_action(self.widget_uid(), &scope.path, MpModalAction::CloseRequested);
            }
        }

        // Handle close button click
        let close_btn = self.view.view(ids!(content.dialog.header.close));
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
                    cx.widget_action(self.widget_uid(), &scope.path, MpModalAction::CloseRequested);
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

impl MpModalWidget {
    /// Open the modal
    pub fn open(&mut self, cx: &mut Cx) {
        self.visible = true;
        self.redraw(cx);
    }

    /// Close the modal
    pub fn close(&mut self, cx: &mut Cx) {
        self.visible = false;
        self.redraw(cx);
    }

    /// Check if modal is visible
    pub fn is_open(&self) -> bool {
        self.visible
    }

    /// Set the modal title
    pub fn set_title(&mut self, cx: &mut Cx, title: &str) {
        self.view.label(ids!(content.dialog.header.title)).set_text(cx, title);
    }
}

impl MpModalWidgetRef {
    pub fn open(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.open(cx);
        }
    }

    pub fn close(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.close(cx);
        }
    }

    pub fn is_open(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_open()
        } else {
            false
        }
    }

    pub fn set_title(&self, cx: &mut Cx, title: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_title(cx, title);
        }
    }

    pub fn close_requested(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            if let Some(item) = actions.find_widget_action(inner.widget_uid()) {
                return matches!(item.cast::<MpModalAction>(), MpModalAction::CloseRequested);
            }
        }
        false
    }
}
