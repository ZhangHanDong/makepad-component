use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // Shared spinner shader function - uses angle-based arc drawing
    SPINNER_SHADER = {
        fn spinner_pixel(self, center: vec2, radius: f32, stroke_width: f32, rotation: f32, arc_ratio: f32, track_color: vec4, spin_color: vec4) -> vec4 {
            let pos = self.pos * self.rect_size - center;
            let dist = length(pos);

            // Ring mask with anti-aliasing
            let inner = radius - stroke_width * 0.5;
            let outer = radius + stroke_width * 0.5;
            let ring = smoothstep(inner - 0.5, inner + 0.5, dist) * smoothstep(outer + 0.5, outer - 0.5, dist);

            // Calculate angle (0 to 1)
            let angle = atan(pos.y, pos.x);
            let norm_angle = mod(angle / (2.0 * PI) + 0.5, 1.0);

            // Arc position relative to rotation
            let arc_pos = mod(norm_angle - rotation + 1.0, 1.0);

            // Arc mask
            let in_arc = step(arc_pos, arc_ratio);

            // Mix colors
            let color = mix(track_color, spin_color, in_arc);
            return vec4(color.rgb, color.a * ring);
        }
    }

    // ============================================================
    // MpSpinner - Base rotating loading spinner component
    // ============================================================

    pub MpSpinner = {{MpSpinner}} {
        width: 24, height: 24
        show_bg: true
        draw_bg: {
            instance rotation: 0.0
            instance spinner_color: #3b82f6
            instance spinner_track: #e5e7eb
            instance stroke_width: 3.0
            instance arc_ratio: 0.25

            fn pixel(self) -> vec4 {
                let center = self.rect_size * 0.5;
                let radius = min(center.x, center.y) - self.stroke_width * 0.5 - 1.0;
                let pos = self.pos * self.rect_size - center;
                let dist = length(pos);

                let inner = radius - self.stroke_width * 0.5;
                let outer = radius + self.stroke_width * 0.5;
                let ring = smoothstep(inner - 0.5, inner + 0.5, dist) * smoothstep(outer + 0.5, outer - 0.5, dist);

                let angle = atan(pos.y, pos.x);
                let norm_angle = mod(angle / (2.0 * PI) + 0.5, 1.0);
                let arc_pos = mod(norm_angle - self.rotation + 1.0, 1.0);
                let in_arc = step(arc_pos, self.arc_ratio);

                let color = mix(self.spinner_track, self.spinner_color, in_arc);
                return vec4(color.rgb, color.a * ring);
            }
        }
        animator: {
            spin = {
                default: on,
                on = {
                    from: { all: Loop { duration: 0.8, end: 1.0 } }
                    apply: { draw_bg: { rotation: [{ time: 0.0, value: 0.0 }, { time: 1.0, value: 1.0 }] } }
                }
            }
        }
    }

    // ============================================================
    // Size variants
    // ============================================================

    pub MpSpinnerXs = <MpSpinner> { width: 16, height: 16, draw_bg: { stroke_width: 2.0 } }
    pub MpSpinnerSm = <MpSpinner> { width: 20, height: 20, draw_bg: { stroke_width: 2.5 } }
    pub MpSpinnerMd = <MpSpinner> { width: 24, height: 24, draw_bg: { stroke_width: 3.0 } }
    pub MpSpinnerLg = <MpSpinner> { width: 32, height: 32, draw_bg: { stroke_width: 4.0 } }
    pub MpSpinnerXl = <MpSpinner> { width: 48, height: 48, draw_bg: { stroke_width: 5.0 } }

    // ============================================================
    // Color variants
    // ============================================================

    pub MpSpinnerPrimary = <MpSpinner> { draw_bg: { spinner_color: (PRIMARY) } }
    pub MpSpinnerSuccess = <MpSpinner> { draw_bg: { spinner_color: #22c55e, spinner_track: #dcfce7 } }
    pub MpSpinnerWarning = <MpSpinner> { draw_bg: { spinner_color: #f59e0b, spinner_track: #fef3c7 } }
    pub MpSpinnerDanger = <MpSpinner> { draw_bg: { spinner_color: #dc2626, spinner_track: #fee2e2 } }

    // ============================================================
    // Style variants
    // ============================================================

    pub MpSpinnerNoTrack = <MpSpinner> { draw_bg: { spinner_track: #00000000 } }
    pub MpSpinnerThin = <MpSpinner> { draw_bg: { stroke_width: 2.0 } }
    pub MpSpinnerThick = <MpSpinner> { draw_bg: { stroke_width: 5.0 } }

    // Speed variants
    pub MpSpinnerFast = <MpSpinner> {
        animator: {
            spin = {
                default: on,
                on = {
                    from: { all: Loop { duration: 0.5, end: 1.0 } }
                    apply: { draw_bg: { rotation: [{ time: 0.0, value: 0.0 }, { time: 1.0, value: 1.0 }] } }
                }
            }
        }
    }

    pub MpSpinnerSlow = <MpSpinner> {
        animator: {
            spin = {
                default: on,
                on = {
                    from: { all: Loop { duration: 1.5, end: 1.0 } }
                    apply: { draw_bg: { rotation: [{ time: 0.0, value: 0.0 }, { time: 1.0, value: 1.0 }] } }
                }
            }
        }
    }

    // ============================================================
    // Spinner with label
    // ============================================================

    pub MpSpinnerWithLabel = <View> {
        width: Fit
        height: Fit
        flow: Right
        spacing: 8
        align: { y: 0.5 }

        spinner = <MpSpinner> {}
        label = <Label> {
            draw_text: {
                color: (MUTED_FOREGROUND)
                text_style: { font_size: 14 }
            }
            text: "Loading..."
        }
    }

    pub MpSpinnerWithLabelVertical = <View> {
        width: Fit
        height: Fit
        flow: Down
        spacing: 8
        align: { x: 0.5 }

        spinner = <MpSpinner> {}
        label = <Label> {
            draw_text: {
                color: (MUTED_FOREGROUND)
                text_style: { font_size: 14 }
            }
            text: "Loading..."
        }
    }

    // ============================================================
    // Dots spinner (alternative style)
    // ============================================================

    pub MpSpinnerDots = {{MpSpinnerDots}} {
        width: 40
        height: 12
        show_bg: true
        draw_bg: {
            instance phase: 0.0
            instance spinner_color: #3b82f6
            fn pixel(self) -> vec4 {
                let sz = self.rect_size;
                let dot_r = sz.y * 0.35;
                let spacing = sz.x / 3.0;
                let dot1_x = spacing * 0.5;
                let dot2_x = spacing * 1.5;
                let dot3_x = spacing * 2.5;
                let dot_y = sz.y * 0.5;
                let phase1 = fract(self.phase);
                let phase2 = fract(self.phase + 0.33);
                let phase3 = fract(self.phase + 0.66);
                let scale1 = 0.5 + 0.5 * sin(phase1 * 2.0 * PI);
                let scale2 = 0.5 + 0.5 * sin(phase2 * 2.0 * PI);
                let scale3 = 0.5 + 0.5 * sin(phase3 * 2.0 * PI);
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.circle(dot1_x, dot_y, dot_r * scale1);
                sdf.fill_keep(self.spinner_color);
                sdf.circle(dot2_x, dot_y, dot_r * scale2);
                sdf.fill_keep(self.spinner_color);
                sdf.circle(dot3_x, dot_y, dot_r * scale3);
                sdf.fill(self.spinner_color);
                return sdf.result;
            }
        }
        animator: {
            spin = {
                default: on,
                on = {
                    from: { all: Loop { duration: 1.0, end: 1.0 } }
                    apply: { draw_bg: { phase: [{ time: 0.0, value: 0.0 }, { time: 1.0, value: 1.0 }] } }
                }
            }
        }
    }

    // ============================================================
    // Pulse spinner (alternative style)
    // ============================================================

    pub MpSpinnerPulse = {{MpSpinnerPulse}} {
        width: 24
        height: 24
        show_bg: true
        draw_bg: {
            instance scale: 0.0
            instance spinner_color: #3b82f6
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y) - 2.0;
                let actual_scale = 0.3 + 0.7 * self.scale;
                let alpha = 1.0 - self.scale * 0.7;
                sdf.circle(c.x, c.y, r * actual_scale);
                sdf.fill(vec4(self.spinner_color.xyz, alpha));
                return sdf.result;
            }
        }
        animator: {
            spin = {
                default: on,
                on = {
                    from: { all: Loop { duration: 1.0, end: 1.0 } }
                    apply: { draw_bg: { scale: [{ time: 0.0, value: 0.0 }, { time: 1.0, value: 1.0 }] } }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MpSpinner {
    #[deref]
    view: View,
    #[animator]
    animator: Animator,
}

impl Widget for MpSpinner {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.animator_play(cx, ids!(spin.on));
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MpSpinnerDots {
    #[deref]
    view: View,
    #[animator]
    animator: Animator,
}

impl Widget for MpSpinnerDots {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.animator_play(cx, ids!(spin.on));
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MpSpinnerPulse {
    #[deref]
    view: View,
    #[animator]
    animator: Animator,
}

impl Widget for MpSpinnerPulse {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.animator_play(cx, ids!(spin.on));
        self.view.draw_walk(cx, scope, walk)
    }
}
