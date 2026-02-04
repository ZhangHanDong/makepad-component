// Drawing elements for plots

use makepad_widgets::*;

live_design! {
    use link::shaders::*;

    // Line drawing shader - draws a line segment using distance field
    // Supports solid, dashed, dotted, and dash-dot line styles
    pub DrawPlotLine = {{DrawPlotLine}} {
        fn pixel(self) -> vec4 {
            // Convert normalized pos to pixel coordinates within the rect
            let p = self.pos * self.rect_size;

            // Line endpoints in local coordinates (passed as uniforms)
            let a = vec2(self.line_x1, self.line_y1);
            let b = vec2(self.line_x2, self.line_y2);

            // Calculate distance from point to line segment
            let pa = p - a;
            let ba = b - a;
            let ba_len_sq = dot(ba, ba);

            // Handle degenerate case (point)
            let h = clamp(dot(pa, ba) / max(ba_len_sq, 0.0001), 0.0, 1.0);
            let dist = length(pa - ba * h);

            // Calculate position along line for dash pattern
            let line_len = sqrt(ba_len_sq);
            let pos_along_line = h * line_len + self.dash_offset;

            // Line style: 0=solid, 1=dashed, 2=dotted, 3=dashdot
            let dash_alpha = 1.0;

            // Dashed pattern (dash=10, gap=5)
            let dashed_pattern = step(0.5, mod(pos_along_line / 15.0, 1.0) - 5.0 / 15.0 + 0.5);

            // Dotted pattern (dot=2, gap=4)
            let dotted_pattern = step(0.5, mod(pos_along_line / 6.0, 1.0) - 4.0 / 6.0 + 0.5);

            // Dash-dot pattern (dash=10, gap=4, dot=2, gap=4) = period 20
            let dashdot_pos = mod(pos_along_line, 20.0);
            let dashdot_pattern = max(
                step(dashdot_pos, 10.0),  // dash part (0-10)
                step(14.0, dashdot_pos) * step(dashdot_pos, 16.0)  // dot part (14-16)
            );

            // Select pattern based on line_style
            let pattern = mix(
                mix(
                    mix(1.0, dashed_pattern, step(0.5, self.line_style) * step(self.line_style, 1.5)),
                    dotted_pattern,
                    step(1.5, self.line_style) * step(self.line_style, 2.5)
                ),
                dashdot_pattern,
                step(2.5, self.line_style)
            );

            // Anti-aliased line with half_width
            let half_width = self.line_width * 0.5;
            let edge = 1.0;
            let alpha = (1.0 - smoothstep(half_width - edge, half_width + edge, dist)) * pattern;

            return vec4(self.color.rgb * alpha, alpha * self.color.a);
        }
    }

    // Marker drawing shader - supports multiple marker shapes
    // marker_style: 0=circle, 1=square, 2=triangle_up, 3=triangle_down, 4=diamond, 5=cross, 6=plus, 7=star
    pub DrawPlotPoint = {{DrawPlotPoint}} {
        fn pixel(self) -> vec4 {
            let uv = self.pos - vec2(0.5, 0.5);
            let dist = length(uv);
            let edge = 0.05;

            // Circle (default, style=0)
            let circle_alpha = 1.0 - smoothstep(0.45 - edge, 0.45, dist);

            // Square (style=1)
            let square_dist = max(abs(uv.x), abs(uv.y));
            let square_alpha = 1.0 - smoothstep(0.4 - edge, 0.4, square_dist);

            // Triangle up (style=2)
            let tri_up_y = uv.y + 0.25;
            let tri_up_dist = max(
                -tri_up_y * 0.866 + abs(uv.x) * 0.5,
                tri_up_y - 0.25
            );
            let tri_up_alpha = 1.0 - smoothstep(-edge, edge, tri_up_dist);

            // Triangle down (style=3)
            let tri_down_y = -uv.y + 0.25;
            let tri_down_dist = max(
                -tri_down_y * 0.866 + abs(uv.x) * 0.5,
                tri_down_y - 0.25
            );
            let tri_down_alpha = 1.0 - smoothstep(-edge, edge, tri_down_dist);

            // Diamond (style=4)
            let diamond_dist = abs(uv.x) + abs(uv.y);
            let diamond_alpha = 1.0 - smoothstep(0.4 - edge, 0.4, diamond_dist);

            // Cross/X (style=5)
            let cross_dist = min(
                abs(uv.x - uv.y) / 1.414,
                abs(uv.x + uv.y) / 1.414
            );
            let cross_alpha = (1.0 - smoothstep(0.08 - edge, 0.08, cross_dist)) * step(dist, 0.45);

            // Plus (style=6)
            let plus_dist = min(abs(uv.x), abs(uv.y));
            let plus_alpha = (1.0 - smoothstep(0.08 - edge, 0.08, plus_dist)) * step(dist, 0.45);

            // Star (style=7) - 5-pointed
            let angle = atan(uv.y, uv.x);
            let star_r = 0.35 + 0.15 * cos(angle * 5.0 + 1.57);
            let star_alpha = 1.0 - smoothstep(star_r - edge, star_r, dist);

            // Select shape based on marker_style
            let alpha = mix(
                mix(
                    mix(
                        mix(circle_alpha, square_alpha, step(0.5, self.marker_style) * step(self.marker_style, 1.5)),
                        mix(tri_up_alpha, tri_down_alpha, step(2.5, self.marker_style) * step(self.marker_style, 3.5)),
                        step(1.5, self.marker_style) * step(self.marker_style, 3.5)
                    ),
                    mix(
                        diamond_alpha,
                        mix(cross_alpha, plus_alpha, step(5.5, self.marker_style) * step(self.marker_style, 6.5)),
                        step(4.5, self.marker_style)
                    ),
                    step(3.5, self.marker_style) * step(self.marker_style, 6.5)
                ),
                star_alpha,
                step(6.5, self.marker_style)
            );

            return vec4(self.color.rgb * alpha, alpha * self.color.a);
        }
    }

    // Bar drawing shader with gradient support
    pub DrawPlotBar = {{DrawPlotBar}} {
        fn pixel(self) -> vec4 {
            // Vertical gradient: interpolate from bottom to top
            if self.gradient_enabled > 0.5 {
                let t = 1.0 - self.pos.y; // 0 at bottom, 1 at top
                let final_color = mix(self.gradient_bottom_color, self.gradient_top_color, t);
                return vec4(final_color.rgb * final_color.a, final_color.a);
            }
            return vec4(self.color.rgb * self.color.a, self.color.a);
        }
    }

    // Fill region shader (for fill_between and area charts) with gradient support
    pub DrawPlotFill = {{DrawPlotFill}} {
        fn pixel(self) -> vec4 {
            // Vertical gradient for area fills
            if self.gradient_enabled > 0.5 {
                let t = 1.0 - self.pos.y; // 0 at bottom, 1 at top
                let final_color = mix(self.gradient_bottom_color, self.gradient_top_color, t);
                return vec4(final_color.rgb * final_color.a, final_color.a);
            }
            return vec4(self.color.rgb * self.color.a, self.color.a);
        }
    }

    // Pie slice drawing shader with radial gradient
    pub DrawPieSlice = {{DrawPieSlice}} {
        fn pixel(self) -> vec4 {
            let uv = self.pos - vec2(0.5, 0.5);
            let dist = length(uv);

            // Check if outside radius
            if dist > 0.5 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }

            // Calculate angle (atan2 returns -PI to PI)
            let angle = atan(uv.y, uv.x);

            // Normalize angle to 0 to 2*PI (using 6.28318... = 2*PI)
            let norm_angle = mod(angle + 6.28318530718, 6.28318530718);

            // Compute inside using step functions (avoid boolean variables)
            let after_start = step(self.start_angle, norm_angle);
            let before_end = step(norm_angle, self.end_angle);
            let inside = after_start * before_end;

            // Handle wrap-around with modulo
            let wrapped_end = mod(self.end_angle, 6.28318530718);
            let is_wrapped = step(6.28318530718, self.end_angle);
            let in_wrapped = step(norm_angle, wrapped_end) * is_wrapped * step(0.0, wrapped_end - norm_angle);

            let in_slice = max(inside, in_wrapped);

            if in_slice > 0.5 {
                let edge = 0.01;
                let alpha = 1.0 - smoothstep(0.5 - edge, 0.5, dist);

                // Radial gradient: from center to edge
                if self.gradient_enabled > 0.5 {
                    let t = dist * 2.0; // 0 at center, 1 at edge
                    let final_color = mix(self.gradient_center_color, self.gradient_outer_color, t);
                    return vec4(final_color.rgb * alpha, alpha * final_color.a);
                }
                return vec4(self.color.rgb * alpha, alpha * self.color.a);
            }

            return vec4(0.0, 0.0, 0.0, 0.0);
        }
    }

    // Arc drawing shader for donut charts with gradient support
    pub DrawArc = {{DrawArc}} {
        fn pixel(self) -> vec4 {
            let pi_val = 3.14159265;
            let two_pi_val = 6.28318530;

            let px = self.pos.x - 0.5;
            let py = self.pos.y - 0.5;

            let distance = sqrt(px * px + py * py);
            let inner_rad = self.inner_radius * 0.5;
            let outer_rad = 0.5;

            // Distance mask: check if in annulus (ring)
            let dist_mask = step(inner_rad, distance) * step(distance, outer_rad);

            // Calculate angle using atan2
            let pixel_ang = atan(py, px);
            let sweep_val = self.end_angle - self.start_angle;
            let rel_ang = pixel_ang - self.start_angle;
            let norm_ang = rel_ang + two_pi_val * 4.0;
            let wrap_ang = mod(norm_ang, two_pi_val);

            // Angle mask: check if within sweep
            let ang_mask = step(wrap_ang, sweep_val) * step(0.001, sweep_val);

            let final_mask = dist_mask * ang_mask;

            // Anti-aliased edges
            let edge_aa = 0.008;
            let outer_aa = 1.0 - smoothstep(outer_rad - edge_aa, outer_rad + edge_aa, distance);
            let inner_aa = smoothstep(inner_rad - edge_aa, inner_rad + edge_aa, distance);
            let aa_alpha = outer_aa * inner_aa;

            let alpha_val = final_mask * aa_alpha;

            if alpha_val < 0.01 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }

            // Radial or angular gradient
            if self.gradient_enabled > 0.5 {
                if self.gradient_type < 0.5 {
                    // Radial gradient: interpolate from inner to outer radius
                    let ring_width = outer_rad - inner_rad;
                    let t = clamp((distance - inner_rad) / ring_width, 0.0, 1.0);
                    let final_color = mix(self.gradient_inner_color, self.gradient_outer_color, t);
                    return vec4(final_color.rgb * alpha_val, final_color.a * alpha_val);
                } else {
                    // Angular gradient: interpolate along arc sweep
                    let t = clamp(wrap_ang / sweep_val, 0.0, 1.0);
                    let final_color = mix(self.gradient_inner_color, self.gradient_outer_color, t);
                    return vec4(final_color.rgb * alpha_val, final_color.a * alpha_val);
                }
            }

            return vec4(self.color.rgb * alpha_val, self.color.a * alpha_val);
        }
    }

    // Point shader with radial gradient support
    pub DrawPlotPointGradient = {{DrawPlotPointGradient}} {
        fn pixel(self) -> vec4 {
            let uv = self.pos;
            let center = vec2(0.5, 0.5);
            let dist = distance(uv, center) * 2.0;

            if dist > 1.0 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }

            let aa = 0.05;
            let alpha = 1.0 - smoothstep(1.0 - aa, 1.0, dist);

            // Radial gradient support
            if self.gradient_enabled > 0.5 {
                let final_color = mix(self.gradient_center_color, self.gradient_outer_color, dist);
                return vec4(final_color.rgb * final_color.a * alpha, final_color.a * alpha);
            }
            return vec4(self.color.rgb * self.color.a * alpha, self.color.a * alpha);
        }
    }

    // Triangle shader with barycentric coordinates for radar fills (from makepad-chart)
    pub DrawTriangle = {{DrawTriangle}} {
        fn pixel(self) -> vec4 {
            // Triangle vertices in normalized coordinates (0-1)
            let v0 = vec2(self.v0x, self.v0y);
            let v1 = vec2(self.v1x, self.v1y);
            let v2 = vec2(self.v2x, self.v2y);

            let p = self.pos;

            // Compute barycentric coordinates
            let d00 = dot(v1 - v0, v1 - v0);
            let d01 = dot(v1 - v0, v2 - v0);
            let d11 = dot(v2 - v0, v2 - v0);
            let d20 = dot(p - v0, v1 - v0);
            let d21 = dot(p - v0, v2 - v0);

            let denom = d00 * d11 - d01 * d01;
            if abs(denom) < 0.0001 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }

            let inv_denom = 1.0 / denom;
            let u = (d11 * d20 - d01 * d21) * inv_denom;
            let v = (d00 * d21 - d01 * d20) * inv_denom;
            let w = 1.0 - u - v;

            // Anti-aliasing: smooth edges using distance to triangle boundary
            let edge_dist = min(min(u, v), w);  // Distance to nearest edge
            let alpha = smoothstep(0.0, 0.03, edge_dist);  // 0.03 = AA width

            // Check if point is inside triangle (with AA margin)
            if u >= -0.03 && v >= -0.03 && (u + v) <= 1.03 {
                // Calculate final color with gradient support
                if self.gradient_enabled > 0.5 {
                    if self.gradient_type < 0.5 {
                        // Radial gradient: v0 is center, v1/v2 are edges
                        let final_color = mix(self.gradient_outer_color, self.gradient_center_color, w);
                        return vec4(final_color.rgb * final_color.a * alpha, final_color.a * alpha);
                    } else {
                        // Vertical gradient: top to bottom based on Y position
                        let final_color = mix(self.gradient_center_color, self.gradient_outer_color, p.y);
                        return vec4(final_color.rgb * final_color.a * alpha, final_color.a * alpha);
                    }
                }
                return vec4(self.color.rgb * self.color.a * alpha, self.color.a * alpha);
            }

            return vec4(0.0, 0.0, 0.0, 0.0);
        }
    }
}

/// Line style enumeration
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LineStyle {
    #[default]
    Solid = 0,
    Dashed = 1,
    Dotted = 2,
    DashDot = 3,
}

/// Marker style enumeration
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum MarkerStyle {
    None = -1,
    #[default]
    Circle = 0,
    Square = 1,
    TriangleUp = 2,
    TriangleDown = 3,
    Diamond = 4,
    Cross = 5,
    Plus = 6,
    Star = 7,
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawPlotLine {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live] pub line_x1: f32,
    #[live] pub line_y1: f32,
    #[live] pub line_x2: f32,
    #[live] pub line_y2: f32,
    #[live] pub line_width: f32,
    #[live] pub line_style: f32,    // 0=solid, 1=dashed, 2=dotted, 3=dashdot
    #[live] pub dash_offset: f32,   // Offset along line for continuous dash pattern
}

impl DrawPlotLine {
    pub fn draw_line(&mut self, cx: &mut Cx2d, p1: DVec2, p2: DVec2, width: f64) {
        self.draw_line_styled(cx, p1, p2, width, LineStyle::Solid, 0.0);
    }

    pub fn draw_line_styled(&mut self, cx: &mut Cx2d, p1: DVec2, p2: DVec2, width: f64, style: LineStyle, dash_offset: f64) {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        let len = (dx * dx + dy * dy).sqrt();

        if len < 0.1 {
            return;
        }

        // Calculate bounding rect with padding for the line width
        let padding = width + 2.0; // Extra padding for anti-aliasing
        let rect = Rect {
            pos: dvec2(p1.x.min(p2.x) - padding, p1.y.min(p2.y) - padding),
            size: dvec2((p2.x - p1.x).abs() + padding * 2.0, (p2.y - p1.y).abs() + padding * 2.0),
        };

        // Set line endpoints in local coordinates (relative to rect)
        self.line_x1 = (p1.x - rect.pos.x) as f32;
        self.line_y1 = (p1.y - rect.pos.y) as f32;
        self.line_x2 = (p2.x - rect.pos.x) as f32;
        self.line_y2 = (p2.y - rect.pos.y) as f32;
        self.line_width = width as f32;
        self.line_style = style as i32 as f32;
        self.dash_offset = dash_offset as f32;

        self.draw_abs(cx, rect);
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawPlotPoint {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live] pub marker_style: f32,  // 0=circle, 1=square, 2=tri_up, 3=tri_down, 4=diamond, 5=cross, 6=plus, 7=star
}

impl DrawPlotPoint {
    pub fn draw_point(&mut self, cx: &mut Cx2d, center: DVec2, radius: f64) {
        self.draw_marker(cx, center, radius, MarkerStyle::Circle);
    }

    pub fn draw_marker(&mut self, cx: &mut Cx2d, center: DVec2, radius: f64, style: MarkerStyle) {
        if style == MarkerStyle::None {
            return;
        }
        self.marker_style = style as i32 as f32;
        let rect = Rect {
            pos: dvec2(center.x - radius, center.y - radius),
            size: dvec2(radius * 2.0, radius * 2.0),
        };
        self.draw_abs(cx, rect);
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawPlotBar {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live(0.0)] pub gradient_enabled: f32,
    #[live] pub gradient_bottom_color: Vec4,
    #[live] pub gradient_top_color: Vec4,
}

impl DrawPlotBar {
    pub fn draw_bar(&mut self, cx: &mut Cx2d, rect: Rect) {
        self.gradient_enabled = 0.0;
        self.draw_abs(cx, rect);
    }

    pub fn draw_bar_gradient(&mut self, cx: &mut Cx2d, rect: Rect, bottom_color: Vec4, top_color: Vec4) {
        self.gradient_enabled = 1.0;
        self.gradient_bottom_color = bottom_color;
        self.gradient_top_color = top_color;
        self.draw_abs(cx, rect);
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawPlotFill {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live(0.0)] pub gradient_enabled: f32,
    #[live] pub gradient_bottom_color: Vec4,
    #[live] pub gradient_top_color: Vec4,
}

impl DrawPlotFill {
    /// Draw a filled vertical strip (used for fill_between)
    pub fn draw_fill_strip(&mut self, cx: &mut Cx2d, x: f64, width: f64, y1: f64, y2: f64) {
        self.gradient_enabled = 0.0;
        let top = y1.min(y2);
        let bottom = y1.max(y2);
        let rect = Rect {
            pos: dvec2(x, top),
            size: dvec2(width, bottom - top),
        };
        self.draw_abs(cx, rect);
    }

    /// Draw a filled strip with vertical gradient
    pub fn draw_fill_strip_gradient(&mut self, cx: &mut Cx2d, x: f64, width: f64, y1: f64, y2: f64, bottom_color: Vec4, top_color: Vec4) {
        self.gradient_enabled = 1.0;
        self.gradient_bottom_color = bottom_color;
        self.gradient_top_color = top_color;
        let top = y1.min(y2);
        let bottom = y1.max(y2);
        let rect = Rect {
            pos: dvec2(x, top),
            size: dvec2(width, bottom - top),
        };
        self.draw_abs(cx, rect);
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawPieSlice {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live] pub start_angle: f32,
    #[live] pub end_angle: f32,
    #[live(0.0)] pub gradient_enabled: f32,
    #[live] pub gradient_center_color: Vec4,
    #[live] pub gradient_outer_color: Vec4,
}

impl DrawPieSlice {
    pub fn draw_slice(&mut self, cx: &mut Cx2d, center: DVec2, radius: f64, start_angle: f64, end_angle: f64) {
        self.gradient_enabled = 0.0;
        self.start_angle = start_angle as f32;
        self.end_angle = end_angle as f32;

        let rect = Rect {
            pos: dvec2(center.x - radius, center.y - radius),
            size: dvec2(radius * 2.0, radius * 2.0),
        };
        self.draw_abs(cx, rect);
    }

    pub fn draw_slice_gradient(&mut self, cx: &mut Cx2d, center: DVec2, radius: f64, start_angle: f64, end_angle: f64, center_color: Vec4, outer_color: Vec4) {
        self.gradient_enabled = 1.0;
        self.gradient_center_color = center_color;
        self.gradient_outer_color = outer_color;
        self.start_angle = start_angle as f32;
        self.end_angle = end_angle as f32;

        let rect = Rect {
            pos: dvec2(center.x - radius, center.y - radius),
            size: dvec2(radius * 2.0, radius * 2.0),
        };
        self.draw_abs(cx, rect);
    }
}

/// Arc drawing for donut charts
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawArc {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live] pub start_angle: f32,
    #[live] pub end_angle: f32,
    #[live(0.0)] pub inner_radius: f32,  // 0-1 ratio of outer radius
    #[live(0.0)] pub gradient_enabled: f32,
    #[live(0.0)] pub gradient_type: f32, // 0=radial, 1=angular
    #[live] pub gradient_inner_color: Vec4,
    #[live] pub gradient_outer_color: Vec4,
}

impl DrawArc {
    pub fn draw_arc(&mut self, cx: &mut Cx2d, center: DVec2, outer_radius: f64, inner_radius_ratio: f64, start_angle: f64, end_angle: f64) {
        self.gradient_enabled = 0.0;
        self.start_angle = start_angle as f32;
        self.end_angle = end_angle as f32;
        self.inner_radius = inner_radius_ratio as f32;

        let rect = Rect {
            pos: dvec2(center.x - outer_radius, center.y - outer_radius),
            size: dvec2(outer_radius * 2.0, outer_radius * 2.0),
        };
        self.draw_abs(cx, rect);
    }

    pub fn draw_arc_gradient(&mut self, cx: &mut Cx2d, center: DVec2, outer_radius: f64, inner_radius_ratio: f64, start_angle: f64, end_angle: f64, inner_color: Vec4, outer_color: Vec4, gradient_type: i32) {
        self.gradient_enabled = 1.0;
        self.gradient_type = gradient_type as f32;
        self.gradient_inner_color = inner_color;
        self.gradient_outer_color = outer_color;
        self.start_angle = start_angle as f32;
        self.end_angle = end_angle as f32;
        self.inner_radius = inner_radius_ratio as f32;

        let rect = Rect {
            pos: dvec2(center.x - outer_radius, center.y - outer_radius),
            size: dvec2(outer_radius * 2.0, outer_radius * 2.0),
        };
        self.draw_abs(cx, rect);
    }
}

/// Point shader with gradient support
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawPlotPointGradient {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live(0.0)] pub gradient_enabled: f32,
    #[live] pub gradient_center_color: Vec4,
    #[live] pub gradient_outer_color: Vec4,
}

impl DrawPlotPointGradient {
    pub fn draw_point(&mut self, cx: &mut Cx2d, center: DVec2, radius: f64) {
        self.gradient_enabled = 0.0;
        let rect = Rect {
            pos: dvec2(center.x - radius, center.y - radius),
            size: dvec2(radius * 2.0, radius * 2.0),
        };
        self.draw_abs(cx, rect);
    }

    pub fn draw_point_gradient(&mut self, cx: &mut Cx2d, center: DVec2, radius: f64, center_color: Vec4, outer_color: Vec4) {
        self.gradient_enabled = 1.0;
        self.gradient_center_color = center_color;
        self.gradient_outer_color = outer_color;
        let rect = Rect {
            pos: dvec2(center.x - radius, center.y - radius),
            size: dvec2(radius * 2.0, radius * 2.0),
        };
        self.draw_abs(cx, rect);
    }
}

// DrawTriangle - for radar chart fills and other triangular shapes
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawTriangle {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live] pub v0x: f32,
    #[live] pub v0y: f32,
    #[live] pub v1x: f32,
    #[live] pub v1y: f32,
    #[live] pub v2x: f32,
    #[live] pub v2y: f32,
    #[live(0.0)] pub gradient_enabled: f32,
    #[live(0.0)] pub gradient_type: f32, // 0 = center-to-vertices, 1 = top-to-bottom
    #[live] pub gradient_center_color: Vec4,
    #[live] pub gradient_outer_color: Vec4,
}

impl DrawTriangle {
    /// Draw a filled triangle with the given vertices (in absolute coordinates)
    pub fn draw_triangle(&mut self, cx: &mut Cx2d, p0: DVec2, p1: DVec2, p2: DVec2) {
        // Calculate bounding box
        let min_x = p0.x.min(p1.x).min(p2.x);
        let min_y = p0.y.min(p1.y).min(p2.y);
        let max_x = p0.x.max(p1.x).max(p2.x);
        let max_y = p0.y.max(p1.y).max(p2.y);

        let width = max_x - min_x;
        let height = max_y - min_y;

        // Avoid division by zero
        if width < 0.001 || height < 0.001 {
            return;
        }

        // Convert to normalized coordinates (0-1 within bounding box)
        self.v0x = ((p0.x - min_x) / width) as f32;
        self.v0y = ((p0.y - min_y) / height) as f32;
        self.v1x = ((p1.x - min_x) / width) as f32;
        self.v1y = ((p1.y - min_y) / height) as f32;
        self.v2x = ((p2.x - min_x) / width) as f32;
        self.v2y = ((p2.y - min_y) / height) as f32;

        self.gradient_enabled = 0.0;

        let rect = Rect {
            pos: dvec2(min_x, min_y),
            size: dvec2(width, height),
        };
        self.draw_abs(cx, rect);
    }

    /// Draw a filled triangle with gradient (center to vertices)
    pub fn draw_triangle_gradient(&mut self, cx: &mut Cx2d, p0: DVec2, p1: DVec2, p2: DVec2, center_color: Vec4, outer_color: Vec4) {
        // Calculate bounding box
        let min_x = p0.x.min(p1.x).min(p2.x);
        let min_y = p0.y.min(p1.y).min(p2.y);
        let max_x = p0.x.max(p1.x).max(p2.x);
        let max_y = p0.y.max(p1.y).max(p2.y);

        let width = max_x - min_x;
        let height = max_y - min_y;

        if width < 0.001 || height < 0.001 {
            return;
        }

        // Convert to normalized coordinates
        self.v0x = ((p0.x - min_x) / width) as f32;
        self.v0y = ((p0.y - min_y) / height) as f32;
        self.v1x = ((p1.x - min_x) / width) as f32;
        self.v1y = ((p1.y - min_y) / height) as f32;
        self.v2x = ((p2.x - min_x) / width) as f32;
        self.v2y = ((p2.y - min_y) / height) as f32;

        self.gradient_enabled = 1.0;
        self.gradient_type = 0.0; // center-to-vertices gradient
        self.gradient_center_color = center_color;
        self.gradient_outer_color = outer_color;

        let rect = Rect {
            pos: dvec2(min_x, min_y),
            size: dvec2(width, height),
        };
        self.draw_abs(cx, rect);
    }
}
