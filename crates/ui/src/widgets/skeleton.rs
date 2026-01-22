use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================================
    // MpSkeleton - Loading placeholder component with shimmer animation
    // ============================================================

    // Rectangular skeleton (default)
    pub MpSkeleton = <View> {
        width: Fill
        height: 20

        show_bg: true
        draw_bg: {
            uniform color_base: #e5e7eb
            uniform color_shimmer: #f3f4f6
            uniform shimmer_width: 0.3
            uniform shimmer_speed: 0.8

            fn pixel(self) -> vec4 {
                let shimmer_pos = fract(self.time * self.shimmer_speed) * (1.0 + self.shimmer_width * 2.0) - self.shimmer_width;
                let dist = abs(self.pos.x - shimmer_pos);
                let shimmer = 1.0 - smoothstep(0.0, self.shimmer_width, dist);
                let result_color = mix(self.color_base, self.color_shimmer, shimmer);
                return result_color;
            }
        }
    }

    // Rounded skeleton
    pub MpSkeletonRounded = <View> {
        width: Fill
        height: 20

        show_bg: true
        draw_bg: {
            instance radius: 4.0

            uniform color_base: #e5e7eb
            uniform color_shimmer: #f3f4f6
            uniform shimmer_speed: 0.8

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let shimmer_pos = fract(self.time * self.shimmer_speed) * 1.6 - 0.3;
                let dist = abs(self.pos.x - shimmer_pos);
                let shimmer = 1.0 - smoothstep(0.0, 0.3, dist);
                let result_color = mix(self.color_base, self.color_shimmer, shimmer);

                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.radius);
                sdf.fill(result_color);

                return sdf.result;
            }
        }
    }

    // Circle skeleton (for avatars)
    pub MpSkeletonCircle = <View> {
        width: 40
        height: 40

        show_bg: true
        draw_bg: {
            uniform color_base: #e5e7eb
            uniform color_shimmer: #f3f4f6
            uniform shimmer_speed: 0.8

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y);

                let shimmer_pos = fract(self.time * self.shimmer_speed) * 1.6 - 0.3;
                let dist = abs(self.pos.x - shimmer_pos);
                let shimmer = 1.0 - smoothstep(0.0, 0.3, dist);
                let result_color = mix(self.color_base, self.color_shimmer, shimmer);

                sdf.circle(c.x, c.y, r);
                sdf.fill(result_color);

                return sdf.result;
            }
        }
    }

    // ============================================================
    // Size variants
    // ============================================================

    // Text line skeleton
    pub MpSkeletonText = <MpSkeletonRounded> {
        width: Fill
        height: 16
        draw_bg: { radius: 2.0 }
    }

    // Title skeleton
    pub MpSkeletonTitle = <MpSkeletonRounded> {
        width: 200
        height: 24
        draw_bg: { radius: 4.0 }
    }

    // Paragraph skeleton
    pub MpSkeletonParagraph = <View> {
        width: Fill
        height: Fit
        flow: Down
        spacing: 8

        <MpSkeletonText> { width: Fill }
        <MpSkeletonText> { width: Fill }
        <MpSkeletonText> { width: 280 }
    }

    // Avatar skeleton sizes
    pub MpSkeletonAvatarSmall = <MpSkeletonCircle> {
        width: 32
        height: 32
    }

    pub MpSkeletonAvatarLarge = <MpSkeletonCircle> {
        width: 56
        height: 56
    }

    // ============================================================
    // Card skeleton
    // ============================================================

    pub MpSkeletonCard = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: (CARD)
            border_radius: 8.0
            border_color: (BORDER)
        }

        <View> {
            width: Fill
            height: Fit
            flow: Right
            spacing: 12
            align: { y: 0.5 }

            <MpSkeletonCircle> {}

            <View> {
                width: Fill
                height: Fit
                flow: Down
                spacing: 8

                <MpSkeletonRounded> { width: 120, height: 16 }
                <MpSkeletonRounded> { width: 80, height: 12 }
            }
        }

        <MpSkeletonParagraph> {}
    }

    // ============================================================
    // List item skeleton
    // ============================================================

    pub MpSkeletonListItem = <View> {
        width: Fill
        height: Fit
        flow: Right
        padding: 12
        spacing: 12
        align: { y: 0.5 }

        <MpSkeletonCircle> {
            width: 40
            height: 40
        }

        <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 6

            <MpSkeletonRounded> { width: 150, height: 16 }
            <MpSkeletonRounded> { width: 100, height: 12 }
        }
    }

    // ============================================================
    // Interactive Skeleton Widget
    // ============================================================

    pub MpSkeletonWidget = {{MpSkeletonWidget}} {
        width: Fill
        height: Fit
        flow: Overlay

        skeleton = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 8
            visible: true

            <MpSkeletonRounded> { width: Fill, height: 20 }
        }

        content = <View> {
            width: Fill
            height: Fit
            visible: false
        }
    }
}

/// Skeleton widget actions
#[derive(Clone, Debug, DefaultNone)]
pub enum MpSkeletonAction {
    None,
    LoadingStarted,
    LoadingFinished,
}

/// Interactive skeleton widget with loading state control
#[derive(Live, LiveHook, Widget)]
pub struct MpSkeletonWidget {
    #[deref]
    view: View,

    #[live]
    loading: bool,
}

impl Widget for MpSkeletonWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MpSkeletonWidget {
    /// Set loading state - shows skeleton when true, content when false
    pub fn set_loading(&mut self, cx: &mut Cx, loading: bool) {
        self.loading = loading;
        self.view.view(ids!(skeleton)).set_visible(cx, loading);
        self.view.view(ids!(content)).set_visible(cx, !loading);
        self.redraw(cx);
    }

    /// Check if currently loading
    pub fn is_loading(&self) -> bool {
        self.loading
    }

    /// Start loading (show skeleton)
    pub fn start_loading(&mut self, cx: &mut Cx) {
        self.set_loading(cx, true);
    }

    /// Finish loading (hide skeleton, show content)
    pub fn finish_loading(&mut self, cx: &mut Cx) {
        self.set_loading(cx, false);
    }
}

impl MpSkeletonWidgetRef {
    pub fn set_loading(&self, cx: &mut Cx, loading: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_loading(cx, loading);
        }
    }

    pub fn is_loading(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.is_loading()
        } else {
            false
        }
    }

    pub fn start_loading(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.start_loading(cx);
        }
    }

    pub fn finish_loading(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.finish_loading(cx);
        }
    }
}
