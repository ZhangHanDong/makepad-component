use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================================
    // MpPopover - Popover/Dropdown panel component
    // ============================================================

    // Base popover container
    pub MpPopoverBase = <View> {
        width: Fit
        height: Fit
        padding: 8

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
    // Default Popover
    // ============================================================

    pub MpPopover = <MpPopoverBase> {
        width: 240
        height: Fit
        padding: 12
        flow: Down
        spacing: 8
    }

    // Popover with arrow pointing up
    pub MpPopoverArrowUp = <View> {
        width: Fit
        height: Fit
        flow: Down
        align: { x: 0.5 }

        arrow = <View> {
            width: 16
            height: 8
            margin: { bottom: -1 }

            show_bg: true
            draw_bg: {
                instance arrow_color: (CARD)
                instance arrow_border_color: (BORDER)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let w = self.rect_size.x;
                    let h = self.rect_size.y;

                    // Triangle pointing up
                    sdf.move_to(w * 0.5, 0.0);
                    sdf.line_to(w, h);
                    sdf.line_to(0.0, h);
                    sdf.close_path();
                    sdf.fill_keep(self.arrow_color);
                    sdf.stroke(self.arrow_border_color, 1.0);

                    return sdf.result;
                }
            }
        }

        content = <MpPopoverBase> {
            width: 240
            height: Fit
            padding: 12
            flow: Down
            spacing: 8
        }
    }

    // Popover with arrow pointing down
    pub MpPopoverArrowDown = <View> {
        width: Fit
        height: Fit
        flow: Down
        align: { x: 0.5 }

        content = <MpPopoverBase> {
            width: 240
            height: Fit
            padding: 12
            flow: Down
            spacing: 8
        }

        arrow = <View> {
            width: 16
            height: 8
            margin: { top: -1 }

            show_bg: true
            draw_bg: {
                instance arrow_color: (CARD)
                instance arrow_border_color: (BORDER)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let w = self.rect_size.x;
                    let h = self.rect_size.y;

                    // Triangle pointing down
                    sdf.move_to(0.0, 0.0);
                    sdf.line_to(w, 0.0);
                    sdf.line_to(w * 0.5, h);
                    sdf.close_path();
                    sdf.fill_keep(self.arrow_color);
                    sdf.stroke(self.arrow_border_color, 1.0);

                    return sdf.result;
                }
            }
        }
    }

    // ============================================================
    // Popover Menu (for dropdown menus)
    // ============================================================

    pub MpPopoverMenu = <MpPopoverBase> {
        width: 200
        height: Fit
        padding: 4
        flow: Down
    }

    // Menu item
    pub MpPopoverMenuItem = <View> {
        width: Fill
        height: Fit
        padding: { left: 12, right: 12, top: 8, bottom: 8 }
        flow: Right
        align: { y: 0.5 }
        spacing: 8
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance bg_color: #00000000
            instance bg_color_hover: #f1f5f9
            instance border_radius: 4.0
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let result_color = mix(self.bg_color, self.bg_color_hover, self.hover);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(result_color);
                return sdf.result;
            }
        }

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.05 } }
                    apply: { draw_bg: { hover: 1.0 } }
                }
            }
        }

        label = <Label> {
            width: Fill
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                color: (FOREGROUND)
            }
            text: "Menu Item"
        }
    }

    // Danger menu item
    pub MpPopoverMenuItemDanger = <MpPopoverMenuItem> {
        draw_bg: {
            bg_color_hover: #fef2f2
        }

        label = <Label> {
            draw_text: {
                color: (DANGER)
            }
        }
    }

    // Menu divider
    pub MpPopoverMenuDivider = <View> {
        width: Fill
        height: 1
        margin: { top: 4, bottom: 4 }
        show_bg: true
        draw_bg: {
            color: (BORDER)
        }
    }

    // Menu section header
    pub MpPopoverMenuHeader = <View> {
        width: Fill
        height: Fit
        padding: { left: 12, right: 12, top: 8, bottom: 4 }

        <Label> {
            width: Fill
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 11.0 }
                color: (MUTED_FOREGROUND)
            }
        }
    }

    // ============================================================
    // Popover Content Variants
    // ============================================================

    // Simple text popover (for tooltips with more content)
    pub MpPopoverText = <MpPopoverBase> {
        width: 240
        height: Fit
        padding: 12

        <Label> {
            width: Fill
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                color: (FOREGROUND)
            }
            text: "Popover content"
        }
    }

    // Popover with header
    pub MpPopoverWithHeader = <MpPopoverBase> {
        width: 280
        height: Fit
        flow: Down

        header = <View> {
            width: Fill
            height: Fit
            padding: { left: 12, right: 12, top: 12, bottom: 8 }

            title_label = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: (FOREGROUND)
                }
                text: "Popover Title"
            }
        }

        body = <View> {
            width: Fill
            height: Fit
            padding: { left: 12, right: 12, top: 0, bottom: 12 }

            desc_label = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: "Popover description text."
            }
        }
    }

    // ============================================================
    // Interactive Popover Widget
    // ============================================================

    pub MpPopoverWidget = {{MpPopoverWidget}} {
        width: Fit
        height: Fit
        flow: Overlay
        opened: false

        content = <MpPopoverBase> {
            width: 200
            height: Fit
            padding: 8
            flow: Down
            spacing: 4
        }
    }

    // ============================================================
    // Interactive Menu Item Widget
    // ============================================================

    pub MpPopoverMenuItemWidget = {{MpPopoverMenuItemWidget}} {
        width: Fill
        height: Fit
        padding: { left: 12, right: 12, top: 8, bottom: 8 }
        flow: Right
        align: { y: 0.5 }
        spacing: 8
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance bg_color: #00000000
            instance bg_color_hover: #f1f5f9
            instance border_radius: 4.0
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let result_color = mix(self.bg_color, self.bg_color_hover, self.hover);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(result_color);
                return sdf.result;
            }
        }

        label = <Label> {
            width: Fill
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                color: (FOREGROUND)
            }
            text: "Menu Item"
        }
    }
}

/// Popover actions
#[derive(Clone, Debug, DefaultNone)]
pub enum MpPopoverAction {
    None,
    Opened,
    Closed,
}

/// Menu item actions
#[derive(Clone, Debug, DefaultNone)]
pub enum MpPopoverMenuItemAction {
    None,
    Clicked,
}

/// Interactive popover widget with show/hide functionality
#[derive(Live, LiveHook, Widget)]
pub struct MpPopoverWidget {
    #[deref]
    view: View,

    #[live]
    opened: bool,
}

impl Widget for MpPopoverWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if !self.opened {
            return;
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.opened {
            return DrawStep::done();
        }
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpPopoverWidget {
    /// Open the popover
    pub fn open(&mut self, cx: &mut Cx) {
        self.opened = true;
        self.redraw(cx);
    }

    /// Close the popover
    pub fn close(&mut self, cx: &mut Cx) {
        self.opened = false;
        self.redraw(cx);
    }

    /// Toggle popover visibility
    pub fn toggle(&mut self, cx: &mut Cx) {
        if self.opened {
            self.close(cx);
        } else {
            self.open(cx);
        }
    }

    /// Check if popover is visible
    pub fn is_open(&self) -> bool {
        self.opened
    }
}

impl MpPopoverWidgetRef {
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

    pub fn toggle(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.toggle(cx);
        }
    }

    pub fn is_open(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_open()
        } else {
            false
        }
    }
}

/// Interactive menu item widget with click handling
#[derive(Live, LiveHook, Widget)]
pub struct MpPopoverMenuItemWidget {
    #[deref]
    view: View,
}

impl Widget for MpPopoverMenuItemWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        match event.hits(cx, self.view.area()) {
            Hit::FingerHoverIn(_) => {
                self.view.apply_over(cx, live!{ draw_bg: { hover: 1.0 } });
                self.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                self.view.apply_over(cx, live!{ draw_bg: { hover: 0.0 } });
                self.redraw(cx);
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    cx.widget_action(self.widget_uid(), &scope.path, MpPopoverMenuItemAction::Clicked);
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpPopoverMenuItemWidget {
    /// Set the menu item label
    pub fn set_text(&mut self, cx: &mut Cx, text: &str) {
        self.view.label(ids!(label)).set_text(cx, text);
    }
}

impl MpPopoverMenuItemWidgetRef {
    pub fn set_text(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_text(cx, text);
        }
    }

    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            if let Some(item) = actions.find_widget_action(inner.widget_uid()) {
                return matches!(item.cast::<MpPopoverMenuItemAction>(), MpPopoverMenuItemAction::Clicked);
            }
        }
        false
    }
}
