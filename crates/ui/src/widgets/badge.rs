use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // Badge indicator constants
    BADGE_HEIGHT = 20.0
    BADGE_FONT_SIZE = 10.0
    BADGE_PADDING_H = 7.0  // Horizontal padding
    BADGE_PADDING_V = 2.0  // Vertical padding

    DOT_SIZE = 8.0
    DOT_RADIUS = 4.0

    // Badge indicator (count badge)
    // Uses SDF capsule: two circles + middle rect for perfect rounded ends
    MpBadgeIndicator = <View> {
        width: Fit,
        height: (BADGE_HEIGHT),
        padding: { left: (BADGE_PADDING_H), right: (BADGE_PADDING_H), top: (BADGE_PADDING_V), bottom: (BADGE_PADDING_V) }
        align: { x: 0.5, y: 0.5 }

        show_bg: true,
        draw_bg: {
            instance bg_color: #EF4444

            // True capsule: two semicircles + middle rectangle
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let r = self.rect_size.y * 0.5;
                let w = self.rect_size.x;
                // Left semicircle
                sdf.circle(r, r, r);
                // Middle rectangle
                sdf.rect(r, 0.0, w - 2.0 * r, 2.0 * r);
                // Right semicircle
                sdf.circle(w - r, r, r);
                sdf.fill(self.bg_color);
                return sdf.result;
            }
        }

        label = <Label> {
            width: Fit,
            height: Fit,
            draw_text: {
                text_style: <THEME_FONT_BOLD> {
                    font_size: (BADGE_FONT_SIZE)
                }
                color: #FFFFFF
            }
            text: ""
        }
    }

    // Dot indicator (small circle) - also uses SDF for consistency
    MpBadgeDotIndicator = <View> {
        width: (DOT_SIZE),
        height: (DOT_SIZE),

        show_bg: true,
        draw_bg: {
            instance bg_color: #EF4444

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let r = min(self.rect_size.x, self.rect_size.y) * 0.5;
                sdf.circle(r, r, r);
                sdf.fill(self.bg_color);
                return sdf.result;
            }
        }
    }

    // Base badge wrapper
    // - Only content padding for badge space (no container padding)
    // - badge_wrapper Fill + align for positioning
    // - badge_offset for fine-tuning
    MpBadgeBase = {{MpBadge}} {
        width: Fit,
        height: Fit,

        flow: Overlay,

        // Content with padding to reserve space for badge
        content = <View> {
            width: Fit,
            height: Fit,
            padding: { top: 9, right: 9 }
        }

        // Badge wrapper: Fill enables align to work
        badge_wrapper = <View> {
            width: Fill,
            height: Fill,
            align: { x: 1.0, y: 0.0 }

            indicator = <MpBadgeIndicator> {}
        }
    }

    // Default badge (red)
    pub MpBadge = <MpBadgeBase> {}

    // Success badge (green)
    pub MpBadgeSuccess = <MpBadgeBase> {
        badge_wrapper = {
            indicator = {
                draw_bg: { bg_color: #22C55E }
            }
        }
    }

    // Warning badge (yellow/orange)
    pub MpBadgeWarning = <MpBadgeBase> {
        badge_wrapper = {
            indicator = {
                draw_bg: { bg_color: #F59E0B }
            }
        }
    }

    // Info badge (blue)
    pub MpBadgeInfo = <MpBadgeBase> {
        badge_wrapper = {
            indicator = {
                draw_bg: { bg_color: #3B82F6 }
            }
        }
    }

    // Secondary badge (gray)
    pub MpBadgeSecondary = <MpBadgeBase> {
        badge_wrapper = {
            indicator = {
                draw_bg: { bg_color: #6B7280 }
            }
        }
    }

    // Dot badge (small indicator) - shares same layout logic
    pub MpBadgeDot = <MpBadgeBase> {
        dot_mode: true,

        content = {
            padding: { top: 4, right: 4 }
        }

        badge_wrapper = {
            indicator = <MpBadgeDotIndicator> {}
        }
    }

    // Dot badge variants (use color since MpBadgeDotIndicator is RoundedView)
    pub MpBadgeDotSuccess = <MpBadgeDot> {
        badge_wrapper = {
            indicator = {
                draw_bg: { bg_color: #22C55E }
            }
        }
    }

    pub MpBadgeDotWarning = <MpBadgeDot> {
        badge_wrapper = {
            indicator = {
                draw_bg: { bg_color: #F59E0B }
            }
        }
    }

    // Standalone badge (inline, not positioned)
    pub MpBadgeStandalone = <MpBadgeIndicator> {}

    pub MpBadgeStandaloneSuccess = <MpBadgeIndicator> {
        draw_bg: { bg_color: #22C55E }
    }

    pub MpBadgeStandaloneWarning = <MpBadgeIndicator> {
        draw_bg: { bg_color: #F59E0B }
    }

    pub MpBadgeStandaloneInfo = <MpBadgeIndicator> {
        draw_bg: { bg_color: #3B82F6 }
    }
}

/// Badge widget for displaying counts, dots, or status indicators
#[derive(Live, Widget)]
pub struct MpBadge {
    #[deref]
    view: View,

    #[live(99)]
    max_count: i64,
    #[live(0)]
    count: i64,
    #[live(false)]
    show_zero: bool,
    #[live(false)]
    dot_mode: bool,

    /// Offset for fine-tuning badge position (shared by Dot and Count)
    #[live]
    badge_offset: DVec2,

    /// Track if display needs update
    #[rust]
    display_dirty: bool,
}

impl LiveHook for MpBadge {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // Mark display as dirty after any live changes
        self.display_dirty = true;
        self.apply_badge_offset(cx);
    }
}

impl Widget for MpBadge {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Only update display if dirty (avoid layout changes during render)
        if self.display_dirty {
            self.sync_badge_display(cx);
            self.display_dirty = false;
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpBadge {
    /// Apply badge_offset to indicator margin
    fn apply_badge_offset(&mut self, cx: &mut Cx) {
        let indicator = self.view.view(ids!(badge_wrapper.indicator));
        indicator.apply_over(cx, live! {
            margin: { top: (self.badge_offset.y), right: (-self.badge_offset.x) }
        });
    }

    /// Sync badge visibility and text (called when dirty)
    fn sync_badge_display(&mut self, cx: &mut Cx) {
        // Determine visibility
        let visible = if self.dot_mode {
            true
        } else if self.count == 0 {
            self.show_zero
        } else {
            true
        };

        // Update badge wrapper visibility
        self.view.view(ids!(badge_wrapper)).set_visible(cx, visible);

        // Update badge text (for non-dot mode)
        if !self.dot_mode && visible {
            let text = if self.count > self.max_count {
                format!("{}+", self.max_count)
            } else {
                self.count.to_string()
            };
            self.view.label(ids!(badge_wrapper.indicator.label)).set_text(cx, &text);
        }
    }

    /// Set the count value
    pub fn set_count(&mut self, cx: &mut Cx, count: i64) {
        if self.count != count {
            self.count = count;
            self.display_dirty = true;
            self.redraw(cx);
        }
    }

    /// Get the current count
    pub fn count(&self) -> i64 {
        self.count
    }

    /// Set whether to show the badge when count is zero
    pub fn set_show_zero(&mut self, cx: &mut Cx, show: bool) {
        if self.show_zero != show {
            self.show_zero = show;
            self.display_dirty = true;
            self.redraw(cx);
        }
    }

    /// Set dot mode
    pub fn set_dot_mode(&mut self, cx: &mut Cx, dot: bool) {
        if self.dot_mode != dot {
            self.dot_mode = dot;
            self.display_dirty = true;
            self.redraw(cx);
        }
    }

    /// Set badge offset for fine-tuning position
    pub fn set_badge_offset(&mut self, cx: &mut Cx, offset: DVec2) {
        self.badge_offset = offset;
        self.apply_badge_offset(cx);
        self.redraw(cx);
    }
}

impl MpBadgeRef {
    /// Set the count value
    pub fn set_count(&self, cx: &mut Cx, count: i64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_count(cx, count);
        }
    }

    /// Get the current count
    pub fn count(&self) -> i64 {
        if let Some(inner) = self.borrow() {
            inner.count()
        } else {
            0
        }
    }

    /// Set whether to show the badge when count is zero
    pub fn set_show_zero(&self, cx: &mut Cx, show: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_show_zero(cx, show);
        }
    }

    /// Set dot mode
    pub fn set_dot_mode(&self, cx: &mut Cx, dot: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_dot_mode(cx, dot);
        }
    }

    /// Set badge offset for fine-tuning position
    pub fn set_badge_offset(&self, cx: &mut Cx, offset: DVec2) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_badge_offset(cx, offset);
        }
    }
}
