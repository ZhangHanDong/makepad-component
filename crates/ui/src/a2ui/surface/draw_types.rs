use makepad_widgets::*;
use crate::a2ui::message::UserAction;

live_design! {
    use link::shaders::*;

    pub DrawA2uiImage = {{DrawA2uiImage}} {
        texture image: texture2d
        instance border_radius: 4.0

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
            let img_color = sample2d(self.image, self.pos);
            sdf.fill(img_color);
            return sdf.result;
        }
    }

    pub DrawA2uiTextField = {{DrawA2uiTextField}} {
        instance border_color: #5588bb
        instance bg_color: #2a3a5a
        instance border_radius: 6.0
        instance border_width: 1.0

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            sdf.box(
                self.border_width,
                self.border_width,
                self.rect_size.x - self.border_width * 2.0,
                self.rect_size.y - self.border_width * 2.0,
                self.border_radius
            );
            sdf.fill_keep(self.bg_color);
            let border = mix(self.border_color, vec4(0.231, 0.51, 0.965, 1.0), self.focus);
            sdf.stroke(border, self.border_width);
            return sdf.result;
        }
    }

    pub DrawA2uiCheckBox = {{DrawA2uiCheckBox}} {
        instance border_color: #5588bb
        instance bg_color: #2a3a5a
        instance check_color: #3B82F6
        instance border_radius: 4.0
        instance border_width: 1.5

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let size = min(self.rect_size.x, self.rect_size.y);
            sdf.box(
                self.border_width,
                self.border_width,
                size - self.border_width * 2.0,
                size - self.border_width * 2.0,
                self.border_radius
            );
            let bg = mix(self.bg_color, self.check_color, self.checked);
            sdf.fill_keep(bg);
            let border = mix(self.border_color, self.check_color, self.hover);
            sdf.stroke(border, self.border_width);
            if self.checked > 0.5 {
                let cx = size * 0.5;
                let cy = size * 0.5;
                let scale = size * 0.25;
                sdf.move_to(cx - scale * 0.8, cy);
                sdf.line_to(cx - scale * 0.2, cy + scale * 0.6);
                sdf.line_to(cx + scale * 0.8, cy - scale * 0.5);
                sdf.stroke(#FFFFFF, 2.0);
            }
            return sdf.result;
        }
    }

    pub DrawA2uiSliderTrack = {{DrawA2uiSliderTrack}} {
        instance track_color: #3a4a6a
        instance fill_color: #3B82F6
        instance border_radius: 3.0

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
            sdf.fill(self.track_color);
            let fill_width = self.rect_size.x * self.progress;
            if fill_width > 0.0 {
                sdf.box(0.0, 0.0, fill_width, self.rect_size.y, self.border_radius);
                sdf.fill(self.fill_color);
            }
            return sdf.result;
        }
    }

    pub DrawA2uiSliderThumb = {{DrawA2uiSliderThumb}} {
        instance thumb_color: #FFFFFF
        instance shadow_color: #00000040

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let radius = min(self.rect_size.x, self.rect_size.y) * 0.5;
            let center = self.rect_size * 0.5;
            sdf.circle(center.x, center.y + 1.0, radius - 1.0);
            sdf.fill(self.shadow_color);
            let thumb_scale = 1.0 + self.hover * 0.1 - self.pressed * 0.05;
            sdf.circle(center.x, center.y, (radius - 2.0) * thumb_scale);
            sdf.fill(self.thumb_color);
            return sdf.result;
        }
    }

    pub DrawA2uiChartLine = {{DrawA2uiChartLine}} {
        fn pixel(self) -> vec4 {
            let uv = self.pos;
            let p1 = vec2(self.x1, self.y1);
            let p2 = vec2(self.x2, self.y2);
            let p = uv;
            let line_vec = p2 - p1;
            let line_len = length(line_vec);
            if line_len < 0.001 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
            let t = clamp(dot(p - p1, line_vec) / (line_len * line_len), 0.0, 1.0);
            let closest = p1 + t * line_vec;
            let dist = length(p - closest);
            let half_width = self.line_width * 0.5;
            let aa = 0.02;
            let alpha = 1.0 - smoothstep(half_width - aa, half_width + aa, dist);
            if alpha < 0.01 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
            return vec4(self.color.rgb * self.color.a * alpha, self.color.a * alpha);
        }
    }

    pub DrawA2uiArc = {{DrawA2uiArc}} {
        fn pixel(self) -> vec4 {
            let two_pi_val = 6.28318530;
            let px = self.pos.x - 0.5;
            let py = self.pos.y - 0.5;
            let distance = sqrt(px * px + py * py);
            let inner_rad = self.inner_radius * 0.5;
            let outer_rad = 0.5;
            let dist_mask = step(inner_rad, distance) * step(distance, outer_rad);
            let pixel_ang = atan(py, px);
            let sweep_val = self.end_angle - self.start_angle;
            let rel_ang = pixel_ang - self.start_angle;
            let norm_ang = rel_ang + two_pi_val * 4.0;
            let wrap_ang = mod(norm_ang, two_pi_val);
            let ang_mask = step(wrap_ang, sweep_val) * step(0.001, sweep_val);
            let final_mask = dist_mask * ang_mask;
            let edge_aa = 0.008;
            let outer_aa = 1.0 - smoothstep(outer_rad - edge_aa, outer_rad + edge_aa, distance);
            let inner_aa = smoothstep(inner_rad - edge_aa, inner_rad + edge_aa, distance);
            let aa_alpha = outer_aa * inner_aa;
            let alpha_val = final_mask * aa_alpha;
            return vec4(self.color.rgb * alpha_val, self.color.a * alpha_val);
        }
    }

    pub DrawA2uiQuad = {{DrawA2uiQuad}} {
        fn pixel(self) -> vec4 {
            let p = self.pos * self.rect_size;
            let p0 = vec2(self.p0x, self.p0y);
            let p1 = vec2(self.p1x, self.p1y);
            let p2 = vec2(self.p2x, self.p2y);
            let p3 = vec2(self.p3x, self.p3y);
            let e0 = p1 - p0; let v0 = p - p0;
            let e1 = p2 - p1; let v1 = p - p1;
            let e2 = p3 - p2; let v2 = p - p2;
            let e3 = p0 - p3; let v3 = p - p3;
            let c0 = e0.x * v0.y - e0.y * v0.x;
            let c1 = e1.x * v1.y - e1.y * v1.x;
            let c2 = e2.x * v2.y - e2.y * v2.x;
            let c3 = e3.x * v3.y - e3.y * v3.x;
            let all_pos = step(0.0, c0) * step(0.0, c1) * step(0.0, c2) * step(0.0, c3);
            let all_neg = step(c0, 0.0) * step(c1, 0.0) * step(c2, 0.0) * step(c3, 0.0);
            let inside = min(all_pos + all_neg, 1.0);
            if inside < 0.5 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
            let len0 = max(length(e0), 0.001);
            let len2 = max(length(e2), 0.001);
            let d0 = abs(c0) / len0;
            let d2 = abs(c2) / len2;
            let min_dist = min(d0, d2);
            let aa = smoothstep(0.0, 1.5, min_dist);
            let alpha = aa * self.opacity;
            return vec4(self.color.rgb * self.color.a * alpha, self.color.a * alpha);
        }
    }

    pub DrawAudioBars = {{DrawAudioBars}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
            sdf.fill(vec4(0.08, 0.08, 0.15, 0.9));
            let num_bars = 5.0;
            let gap = 3.0;
            let padding = 4.0;
            let usable_width = self.rect_size.x - padding * 2.0;
            let bar_width = (usable_width - gap * (num_bars - 1.0)) / num_bars;
            for i in 0..5 {
                let fi = float(i);
                let x = padding + fi * (bar_width + gap);
                let phase = fi * 1.2;
                let wave = sin(self.time * 5.0 + phase) * 0.5 + 0.5;
                let bar_max_height = self.rect_size.y - padding * 2.0;
                let height = mix(0.2, wave, self.is_playing) * bar_max_height;
                let t = fi / 4.0;
                let cyan = vec3(0.0, 1.0, 1.0);
                let purple = vec3(0.5, 0.0, 1.0);
                let pink = vec3(1.0, 0.2, 0.6);
                let color = mix(mix(cyan, purple, t), pink, t * t);
                let y = self.rect_size.y - padding - height;
                sdf.box(x, y, bar_width, height, 1.5);
                sdf.fill(vec4(color, 1.0));
            }
            return sdf.result;
        }
    }
}

// ============================================================================
// A2UI Theme Colors
// ============================================================================

/// Theme colors for A2UI surface and its components
#[derive(Clone, Copy, Debug)]
pub struct A2uiThemeColors {
    /// Background color for the surface
    pub bg_surface: Vec4,
    /// Background color for cards
    pub bg_card: Vec4,
    /// Border color for cards, inputs, etc.
    pub border_color: Vec4,
    /// Primary text color
    pub text_primary: Vec4,
    /// Secondary/muted text color
    pub text_secondary: Vec4,
    /// Accent/primary button color
    pub accent: Vec4,
    /// Accent hover color
    pub accent_hover: Vec4,
    /// Accent pressed color
    pub accent_pressed: Vec4,
    /// Input field background color
    pub input_bg: Vec4,
    /// Slider track color
    pub slider_track: Vec4,
    /// Checkbox/slider fill color
    pub control_fill: Vec4,
}

impl Default for A2uiThemeColors {
    fn default() -> Self {
        // Default dark purple theme
        Self {
            bg_surface: vec4(0.102, 0.102, 0.180, 1.0),      // #1a1a2e
            bg_card: vec4(0.165, 0.227, 0.353, 1.0),         // #2a3a5a
            border_color: vec4(0.333, 0.533, 0.733, 1.0),    // #5588bb
            text_primary: vec4(1.0, 1.0, 1.0, 1.0),          // #FFFFFF
            text_secondary: vec4(0.533, 0.533, 0.533, 1.0),  // #888888
            accent: vec4(0.231, 0.51, 0.965, 1.0),           // #3B82F6
            accent_hover: vec4(0.145, 0.388, 0.922, 1.0),    // slightly darker
            accent_pressed: vec4(0.114, 0.306, 0.847, 1.0),  // even darker
            input_bg: vec4(0.165, 0.227, 0.353, 1.0),        // #2a3a5a
            slider_track: vec4(0.227, 0.290, 0.416, 1.0),    // #3a4a6a
            control_fill: vec4(0.231, 0.51, 0.965, 1.0),     // #3B82F6
        }
    }
}

impl A2uiThemeColors {
    /// Create dark purple theme colors (default)
    pub fn dark_purple() -> Self {
        Self::default()
    }

    /// Create light iOS-like theme colors
    pub fn light() -> Self {
        Self {
            bg_surface: vec4(1.0, 1.0, 1.0, 1.0),            // #FFFFFF
            bg_card: vec4(0.96, 0.96, 0.97, 1.0),            // #f5f5f8
            border_color: vec4(0.85, 0.85, 0.87, 1.0),       // #d9d9de
            text_primary: vec4(0.11, 0.11, 0.118, 1.0),      // #1c1c1e
            text_secondary: vec4(0.557, 0.557, 0.576, 1.0),  // #8e8e93
            accent: vec4(0.0, 0.478, 1.0, 1.0),              // #007AFF
            accent_hover: vec4(0.0, 0.4, 0.85, 1.0),         // slightly darker
            accent_pressed: vec4(0.0, 0.35, 0.75, 1.0),      // even darker
            input_bg: vec4(0.95, 0.95, 0.97, 1.0),           // light gray
            slider_track: vec4(0.9, 0.9, 0.92, 1.0),         // light gray
            control_fill: vec4(0.0, 0.478, 1.0, 1.0),        // #007AFF
        }
    }

    /// Create soft gray mid-tone theme colors
    pub fn soft() -> Self {
        Self {
            bg_surface: vec4(0.533, 0.553, 0.588, 1.0),      // #888d96
            bg_card: vec4(0.6, 0.62, 0.66, 1.0),             // slightly lighter
            border_color: vec4(0.7, 0.72, 0.76, 1.0),        // light border
            text_primary: vec4(1.0, 1.0, 1.0, 1.0),          // #FFFFFF
            text_secondary: vec4(0.2, 0.2, 0.25, 1.0),       // dark gray for contrast
            accent: vec4(0.231, 0.51, 0.965, 1.0),           // #3B82F6 (vibrant blue)
            accent_hover: vec4(0.145, 0.388, 0.922, 1.0),    // slightly darker
            accent_pressed: vec4(0.114, 0.306, 0.847, 1.0),  // even darker
            input_bg: vec4(0.5, 0.52, 0.56, 1.0),            // medium gray
            slider_track: vec4(0.45, 0.47, 0.51, 1.0),       // darker gray
            control_fill: vec4(0.231, 0.51, 0.965, 1.0),     // #3B82F6 (vibrant blue)
        }
    }
}

// ============================================================================
// A2UI Surface Actions
// ============================================================================

/// Actions emitted by A2uiSurface widget
#[derive(Clone, Debug, DefaultNone)]
pub enum A2uiSurfaceAction {
    None,
    /// User triggered an action (e.g., button click)
    UserAction(UserAction),
    /// Data model value changed (two-way binding)
    DataModelChanged {
        surface_id: String,
        path: String,
        value: serde_json::Value,
    },
    /// Audio player play button clicked - open URL in browser
    PlayAudio {
        component_id: String,
        url: String,
        title: String,
    },
}

// ============================================================================
// DrawA2uiImage - for rendering images with border radius
// ============================================================================

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawA2uiImage {
    #[deref]
    draw_super: DrawQuad,
}

// ============================================================================
// DrawA2uiTextField - for rendering text field backgrounds
// ============================================================================

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawA2uiTextField {
    #[deref]
    draw_super: DrawQuad,
    #[live(0.0)]
    pub focus: f32,
}

// ============================================================================
// DrawA2uiCheckBox - for rendering checkbox with checkmark
// ============================================================================

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawA2uiCheckBox {
    #[deref]
    draw_super: DrawQuad,
    #[live(0.0)]
    pub checked: f32,
    #[live(0.0)]
    pub hover: f32,
}

// ============================================================================
// DrawA2uiSliderTrack - for rendering slider track
// ============================================================================

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawA2uiSliderTrack {
    #[deref]
    draw_super: DrawQuad,
    #[live(0.0)]
    pub progress: f32,
}

// ============================================================================
// DrawA2uiSliderThumb - for rendering slider thumb
// ============================================================================

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawA2uiSliderThumb {
    #[deref]
    draw_super: DrawQuad,
    #[live(0.0)]
    pub hover: f32,
    #[live(0.0)]
    pub pressed: f32,
}

// ============================================================================
// DrawA2uiBar - for rendering bar chart bars
// ============================================================================
// DrawA2uiChartLine - for rendering line chart segments (chord chart)
// ============================================================================

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawA2uiChartLine {
    #[deref]
    draw_super: DrawQuad,
    #[live]
    pub color: Vec4,
    #[live(0.0)]
    pub x1: f32,
    #[live(0.0)]
    pub y1: f32,
    #[live(1.0)]
    pub x2: f32,
    #[live(1.0)]
    pub y2: f32,
    #[live(0.05)]
    pub line_width: f32,
}

// ============================================================================
// DrawA2uiArc - for rendering pie chart slices
// ============================================================================

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawA2uiArc {
    #[deref]
    draw_super: DrawQuad,
    #[live]
    pub color: Vec4,
    #[live(0.0)]
    pub start_angle: f32,
    #[live(1.0)]
    pub end_angle: f32,
    #[live(0.0)]
    pub inner_radius: f32,
}

// ============================================================================
// DrawA2uiQuad - arbitrary convex quadrilateral with AA edges (chord chart)
// DrawAudioBars - for rendering audio waveform visualization
// ============================================================================

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawA2uiQuad {
    #[deref]
    draw_super: DrawQuad,
    #[live]
    pub color: Vec4,
    #[live(0.3)]
    pub opacity: f32,
    // Corner positions in pixel coords relative to the draw quad origin
    #[live(0.0)]
    pub p0x: f32,
    #[live(0.0)]
    pub p0y: f32,
    #[live(1.0)]
    pub p1x: f32,
    #[live(0.0)]
    pub p1y: f32,
    #[live(1.0)]
    pub p2x: f32,
    #[live(1.0)]
    pub p2y: f32,
    #[live(0.0)]
    pub p3x: f32,
    #[live(1.0)]
    pub p3y: f32,
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawAudioBars {
    #[deref]
    draw_super: DrawQuad,
    #[live(0.0)]
    pub is_playing: f32,
}
