use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================================
    // MpDrawer - Drawer component
    // ============================================================

    // Drawer backdrop (mask)
    MpDrawerBackdrop = <View> {
        width: Fill
        height: Fill

        show_bg: true
        draw_bg: {
            color: #00000066

            fn pixel(self) -> vec4 {
                return self.color;
            }
        }
    }

    // Base drawer panel
    MpDrawerBase = {{MpDrawer}} {
        fixed: true
        width: 320
        height: Fill
        flow: Down

        show_bg: true
        draw_bg: {
            instance bg_color: (CARD)
            instance border_radius: 12.0
            instance border_color: (BORDER)
            instance shadow_color: #00000026
            instance shadow_offset_x: 0.0
            instance shadow_offset_y: 0.0
            instance shadow_blur: 24.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let shadow_x = self.shadow_offset_x;
                let shadow_y = self.shadow_offset_y;

                // Shadow
                sdf.box(
                    shadow_x,
                    shadow_y,
                    self.rect_size.x - abs(shadow_x),
                    self.rect_size.y - abs(shadow_y),
                    self.border_radius
                );
                sdf.blur = self.shadow_blur;
                sdf.fill(self.shadow_color);
                sdf.blur = 0.0;

                // Main panel
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
                text: "Drawer Title"
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

        body = <ScrollYView> {
            width: Fill
            height: Fill
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
                text: "Drawer content goes here."
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
    // Drawer Placement Variants
    // ============================================================

    pub MpDrawerRight = <MpDrawerBase> {
        draw_bg: { shadow_offset_x: -8.0 }
    }

    pub MpDrawerLeft = <MpDrawerBase> {
        draw_bg: { shadow_offset_x: 8.0 }
    }

    pub MpDrawerTop = <MpDrawerBase> {
        width: Fill
        height: 240
        draw_bg: { shadow_offset_y: 8.0 }
    }

    pub MpDrawerBottom = <MpDrawerBase> {
        width: Fill
        height: 240
        draw_bg: { shadow_offset_y: -8.0 }
    }

    pub MpDrawer = <MpDrawerRight> {}

    // ============================================================
    // Drawer Containers (with backdrop)
    // ============================================================

    pub MpDrawerContainerRight = <View> {
        width: Fill
        height: Fill
        flow: Overlay
        align: { x: 1.0, y: 0.0 }

        backdrop = <MpDrawerBackdrop> {}
        drawer = <MpDrawerRight> {}
    }

    pub MpDrawerContainerLeft = <View> {
        width: Fill
        height: Fill
        flow: Overlay
        align: { x: 0.0, y: 0.0 }

        backdrop = <MpDrawerBackdrop> {}
        drawer = <MpDrawerLeft> {}
    }

    pub MpDrawerContainerTop = <View> {
        width: Fill
        height: Fill
        flow: Overlay
        align: { x: 0.0, y: 0.0 }

        backdrop = <MpDrawerBackdrop> {}
        drawer = <MpDrawerTop> {}
    }

    pub MpDrawerContainerBottom = <View> {
        width: Fill
        height: Fill
        flow: Overlay
        align: { x: 0.0, y: 1.0 }

        backdrop = <MpDrawerBackdrop> {}
        drawer = <MpDrawerBottom> {}
    }

    // ============================================================
    // Drawer Layout Helpers
    // ============================================================

    pub MpDrawerHeader = <View> {
        width: Fill
        height: Fit
        padding: { left: 24, right: 24, top: 20, bottom: 16 }
        flow: Right
        align: { y: 0.5 }
    }

    pub MpDrawerBody = <ScrollYView> {
        width: Fill
        height: Fill
        padding: { left: 24, right: 24, top: 0, bottom: 16 }
        flow: Down
        spacing: 8
    }

    pub MpDrawerFooter = <View> {
        width: Fill
        height: Fit
        padding: { left: 24, right: 24, top: 16, bottom: 20 }
        flow: Right
        spacing: 8
        align: { x: 1.0, y: 0.5 }
    }

    pub MpDrawerDivider = <View> {
        width: Fill
        height: 1
        show_bg: true
        draw_bg: {
            color: (BORDER)
        }
    }
}

#[derive(Live, Widget)]
pub struct MpDrawer {
    #[deref]
    view: View,
    #[live(true)]
    fixed: bool,
    #[rust]
    base_height: Option<Size>,
}

impl LiveHook for MpDrawer {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.sync_fixed_layout(cx);
    }
}

impl Widget for MpDrawer {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpDrawer {
    fn sync_fixed_layout(&mut self, cx: &mut Cx) {
        if self.base_height.is_none() {
            self.base_height = Some(self.view.walk.height);
        }

        if self.fixed {
            if let Some(base_height) = self.base_height {
                self.view.walk.height = base_height;
            }
            self.view(ids!(body)).apply_over(cx, live! {
                walk: { height: Fill }
                scroll_bars: {
                    show_scroll_x: false,
                    show_scroll_y: true,
                    scroll_bar_y: { drag_scrolling: true }
                }
            });
        } else {
            self.view.walk.height = Size::fit();
            self.view(ids!(body)).apply_over(cx, live! {
                walk: { height: Fit }
                scroll_bars: {
                    show_scroll_x: false,
                    show_scroll_y: false
                }
            });
        }

        self.view.redraw(cx);
    }
}
