use makepad_widgets::*;

// ============================================================
// ShaderCanvas - Animated shader widget
// ============================================================
#[derive(Live, LiveHook, Widget)]
pub struct ShaderCanvas {
    #[deref] view: View,
    #[animator] animator: Animator,
}

impl Widget for ShaderCanvas {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Start time animation
        self.animator_play(cx, ids!(anim.on));
        self.view.draw_walk(cx, scope, walk)
    }
}

// ============================================================
// ShaderArtCanvas - Observer shader widget
// ============================================================
#[derive(Live, LiveHook, Widget)]
pub struct ShaderArtCanvas {
    #[deref] view: View,
    #[animator] animator: Animator,
    #[live] speed: f64,
}

impl Widget for ShaderArtCanvas {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Apply speed to shader
        self.view.apply_over(cx, live!{
            draw_bg: { speed: (self.speed) }
        });
        // Start time animation
        self.animator_play(cx, ids!(anim.on));
        self.view.draw_walk(cx, scope, walk)
    }
}


// ============================================================
// ShaderArt2Canvas - FBM noise art widget
// ============================================================
#[derive(Live, LiveHook, Widget)]
pub struct ShaderArt2Canvas {
    #[deref] view: View,
    #[animator] animator: Animator,
    #[live] speed: f64,
}

impl Widget for ShaderArt2Canvas {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.apply_over(cx, live!{
            draw_bg: { speed: (self.speed) }
        });
        self.animator_play(cx, ids!(anim.on));
        self.view.draw_walk(cx, scope, walk)
    }
}

// ============================================================
// ShaderMathCanvas - Parametric flow field widget
// ============================================================
#[derive(Live, LiveHook, Widget)]
pub struct ShaderMathCanvas {
    #[deref] view: View,
    #[animator] animator: Animator,
    #[live] speed: f64,
}

impl Widget for ShaderMathCanvas {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.apply_over(cx, live!{
            draw_bg: { speed: (self.speed) }
        });
        self.animator_play(cx, ids!(anim.on));
        self.view.draw_walk(cx, scope, walk)
    }
}

