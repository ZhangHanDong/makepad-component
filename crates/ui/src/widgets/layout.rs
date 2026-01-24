use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================================
    // MpLayout - Layout container components (Header / Body / Sider / Content / Footer)
    // ============================================================

    pub MpLayout = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        spacing: 0
        padding: 1

        draw_bg: {
            color: (CARD)
            border_radius: 12.0
            border_color: (BORDER)
        }
    }

    pub MpLayoutHeader = <View> {
        width: Fill
        height: 64
        padding: { left: 20, right: 20, top: 14, bottom: 14 }
        flow: Right
        align: { y: 0.5 }

        show_bg: true
        draw_bg: {
            instance bg_color: #f8fafc
            instance line_color: (BORDER)
            instance line_width: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 0.0);
                sdf.fill(self.bg_color);

                if self.line_width > 0.0 {
                    sdf.box(
                        0.0,
                        self.rect_size.y - self.line_width,
                        self.rect_size.x,
                        self.line_width,
                        0.0
                    );
                    sdf.fill(self.line_color);
                }
                return sdf.result;
            }
        }
    }

    pub MpLayoutFooter = <View> {
        width: Fill
        height: 48
        padding: { left: 20, right: 20, top: 12, bottom: 12 }
        flow: Right
        align: { y: 0.5 }

        show_bg: true
        draw_bg: {
            instance bg_color: #f8fafc
            instance line_color: (BORDER)
            instance line_width: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 0.0);
                sdf.fill(self.bg_color);

                if self.line_width > 0.0 {
                    sdf.box(0.0, 0.0, self.rect_size.x, self.line_width, 0.0);
                    sdf.fill(self.line_color);
                }
                return sdf.result;
            }
        }
    }

    pub MpLayoutBody = <View> {
        width: Fill
        height: Fill
        flow: Right
    }

    pub MpLayoutContent = <View> {
        width: Fill
        height: Fill
        padding: 20
        flow: Down
        spacing: 12

        show_bg: true
        draw_bg: { color: (CARD) }
    }

    pub MpLayoutSider = <View> {
        width: 220
        height: Fill
        padding: 16
        flow: Down
        spacing: 10

        show_bg: true
        draw_bg: {
            instance bg_color: (MUTED)
            instance line_color: (BORDER)
            instance line_width: 1.0
            instance line_side: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 0.0);
                sdf.fill(self.bg_color);

                if self.line_width > 0.0 {
                    if self.line_side > 0.0 {
                        sdf.box(
                            self.rect_size.x - self.line_width,
                            0.0,
                            self.line_width,
                            self.rect_size.y,
                            0.0
                        );
                    } else {
                        sdf.box(0.0, 0.0, self.line_width, self.rect_size.y, 0.0);
                    }
                    sdf.fill(self.line_color);
                }
                return sdf.result;
            }
        }
    }

    pub MpLayoutSiderRight = <MpLayoutSider> {
        draw_bg: {
            line_side: -1.0
        }
    }
}
