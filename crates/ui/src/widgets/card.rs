use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================================
    // MpCard - Card container component
    // ============================================================

    // Base card style (using RoundedView for border support)
    pub MpCard = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: (CARD)
            border_radius: 8.0
        }
    }

    // Card with shadow
    pub MpCardShadow = <View> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        show_bg: true
        draw_bg: {
            instance bg_color: (CARD)
            instance border_radius: 8.0
            instance shadow_color: #0000001A
            instance shadow_offset_x: 0.0
            instance shadow_offset_y: 2.0
            instance shadow_blur: 8.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Shadow (drawn first, behind the card)
                let shadow_x = self.shadow_offset_x;
                let shadow_y = self.shadow_offset_y;
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

                // Main card
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(self.bg_color);

                return sdf.result;
            }
        }
    }

    // Card with hover effect
    pub MpCardHover = <View> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance bg_color: (CARD)
            instance bg_color_hover: #f8fafc
            instance border_radius: 8.0
            instance border_color: (BORDER)
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let bg = mix(self.bg_color, self.bg_color_hover, self.hover);

                sdf.box(
                    0.5,
                    0.5,
                    self.rect_size.x - 1.0,
                    self.rect_size.y - 1.0,
                    self.border_radius
                );
                sdf.fill_keep(bg);
                sdf.stroke(self.border_color, 1.0);

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

    // Outlined card (no background fill)
    pub MpCardOutline = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: #00000000
            border_radius: 8.0
            border_color: (BORDER)
        }
    }

    // Ghost card (transparent, no border)
    pub MpCardGhost = <View> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12
    }

    // ============================================================
    // Card sub-components
    // ============================================================

    // Card header section
    pub MpCardHeader = <View> {
        width: Fill
        height: Fit
        flow: Down
        spacing: 4
    }

    // Card title
    pub MpCardTitle = <Label> {
        width: Fit
        height: Fit
        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 18.0 }
            color: (CARD_FOREGROUND)
        }
    }

    // Card description
    pub MpCardDescription = <Label> {
        width: Fill
        height: Fit
        draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
            color: (MUTED_FOREGROUND)
        }
    }

    // Card content section
    pub MpCardContent = <View> {
        width: Fill
        height: Fit
        flow: Down
        spacing: 8
    }

    // Card footer section
    pub MpCardFooter = <View> {
        width: Fill
        height: Fit
        flow: Right
        spacing: 8
        align: { y: 0.5 }
    }

    // ============================================================
    // Size Variants
    // ============================================================

    // Small card
    pub MpCardSmall = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 12
        spacing: 8

        draw_bg: {
            color: (CARD)
            border_radius: 6.0
        }
    }

    // Large card
    pub MpCardLarge = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 24
        spacing: 16

        draw_bg: {
            color: (CARD)
            border_radius: 12.0
        }
    }

    // ============================================================
    // Color Variants
    // ============================================================

    // Primary card
    pub MpCardPrimary = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: (PRIMARY)
            border_radius: 8.0
        }
    }

    // Danger card
    pub MpCardDanger = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: #fef2f2
            border_radius: 8.0
            border_color: #fecaca
        }
    }

    // Success card
    pub MpCardSuccess = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: #f0fdf4
            border_radius: 8.0
            border_color: #bbf7d0
        }
    }

    // Warning card
    pub MpCardWarning = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: #fffbeb
            border_radius: 8.0
            border_color: #fde68a
        }
    }

    // Info card
    pub MpCardInfo = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: #ecfeff
            border_radius: 8.0
            border_color: #a5f3fc
        }
    }

    // ============================================================
    // Interactive Card (with click support)
    // ============================================================

    pub MpCardClickable = {{MpCardClickable}} {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance bg_color: (CARD)
            instance bg_color_hover: #f8fafc
            instance border_radius: 8.0
            instance border_color: (BORDER)
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let bg = mix(self.bg_color, self.bg_color_hover, self.hover);

                sdf.box(
                    0.5,
                    0.5,
                    self.rect_size.x - 1.0,
                    self.rect_size.y - 1.0,
                    self.border_radius
                );
                sdf.fill_keep(bg);
                sdf.stroke(self.border_color, 1.0);

                return sdf.result;
            }
        }
    }
}

/// Card action emitted when clicked
#[derive(Clone, Debug, DefaultNone)]
pub enum MpCardAction {
    None,
    Clicked,
}

/// Interactive card widget with hover effect and click support
#[derive(Live, LiveHook, Widget)]
pub struct MpCardClickable {
    #[deref]
    view: View,

    #[rust]
    hover: f32,
}

impl Widget for MpCardClickable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        match event.hits(cx, self.view.area()) {
            Hit::FingerHoverIn(_) => {
                self.hover = 1.0;
                self.view.apply_over(cx, live!{ draw_bg: { hover: 1.0 } });
                self.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                self.hover = 0.0;
                self.view.apply_over(cx, live!{ draw_bg: { hover: 0.0 } });
                self.redraw(cx);
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    cx.widget_action(self.widget_uid(), &scope.path, MpCardAction::Clicked);
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpCardClickable {
    /// Check if the card was clicked
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast::<MpCardAction>(), MpCardAction::Clicked)
        } else {
            false
        }
    }
}

impl MpCardClickableRef {
    /// Check if the card was clicked
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(inner) = self.borrow() {
            inner.clicked(actions)
        } else {
            false
        }
    }
}
