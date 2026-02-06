//! A2UI Surface Widget
//!
//! The A2uiSurface widget is the root container for rendering A2UI component trees.
//! It manages the A2uiMessageProcessor and dynamically renders components.

use makepad_widgets::*;
use makepad_plot::*;

use super::{
    chart_bridge,
    data_model::DataModel,
    message::*,
    processor::{
        resolve_boolean_value_scoped, resolve_number_value_scoped,
        resolve_string_value_scoped, A2uiMessageProcessor, ProcessorEvent,
    },
};

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

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use makepad_plot::plot::LinePlot;
    use makepad_plot::plot::BarPlot;
    use makepad_plot::plot::ScatterPlot;
    use makepad_plot::plot::PieChart;
    use makepad_plot::plot::AreaChart;
    use makepad_plot::plot::RadarChart;
    use makepad_plot::plot::GaugeChart;
    use makepad_plot::plot::BubbleChart;
    use makepad_plot::plot::CandlestickChart;
    use makepad_plot::plot::HeatmapChart;
    use makepad_plot::plot::Treemap;
    use makepad_plot::plot::SankeyDiagram;
    use makepad_plot::plot::HistogramChart;
    use makepad_plot::plot::BoxPlotChart;
    use makepad_plot::plot::DonutChart;
    use makepad_plot::plot::StemPlot;
    use makepad_plot::plot::ViolinPlot;
    use makepad_plot::plot::PolarPlot;
    use makepad_plot::plot::ContourPlot;
    use makepad_plot::plot::WaterfallChart;
    use makepad_plot::plot::FunnelChart;
    use makepad_plot::plot::StepPlot;
    use makepad_plot::plot::Stackplot;
    use makepad_plot::plot::HexbinChart;
    use makepad_plot::plot::Streamgraph;
    use makepad_plot::plot::Surface3D;
    use makepad_plot::plot::Scatter3D;
    use makepad_plot::plot::Line3D;

    use crate::theme::colors::*;

    // DrawImage for rendering actual images with rounded corners
    DrawA2uiImage = {{DrawA2uiImage}} {
        texture image: texture2d
        instance border_radius: 4.0

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);

            // Sample image texture
            let img_color = sample2d(self.image, self.pos);

            sdf.fill(img_color);
            return sdf.result;
        }
    }

    // ============================================================================
    // A2UI TextField - Text input component shader
    // ============================================================================
    DrawA2uiTextField = {{DrawA2uiTextField}} {
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

            // Highlight border on focus
            let border = mix(self.border_color, vec4(0.231, 0.51, 0.965, 1.0), self.focus);
            sdf.stroke(border, self.border_width);
            return sdf.result;
        }
    }

    // ============================================================================
    // A2UI CheckBox - Checkbox component shader
    // ============================================================================
    DrawA2uiCheckBox = {{DrawA2uiCheckBox}} {
        instance border_color: #5588bb
        instance bg_color: #2a3a5a
        instance check_color: #3B82F6
        instance border_radius: 4.0
        instance border_width: 1.5

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let size = min(self.rect_size.x, self.rect_size.y);

            // Checkbox box
            sdf.box(
                self.border_width,
                self.border_width,
                size - self.border_width * 2.0,
                size - self.border_width * 2.0,
                self.border_radius
            );

            // Fill with check color when checked
            let bg = mix(self.bg_color, self.check_color, self.checked);
            sdf.fill_keep(bg);

            // Border with hover effect
            let border = mix(self.border_color, self.check_color, self.hover);
            sdf.stroke(border, self.border_width);

            // Draw checkmark when checked
            if self.checked > 0.5 {
                let cx = size * 0.5;
                let cy = size * 0.5;
                let scale = size * 0.25;

                // Checkmark path (two lines)
                sdf.move_to(cx - scale * 0.8, cy);
                sdf.line_to(cx - scale * 0.2, cy + scale * 0.6);
                sdf.line_to(cx + scale * 0.8, cy - scale * 0.5);
                sdf.stroke(#FFFFFF, 2.0);
            }

            return sdf.result;
        }
    }

    // ============================================================================
    // A2UI Slider - Slider component shaders
    // ============================================================================
    DrawA2uiSliderTrack = {{DrawA2uiSliderTrack}} {
        instance track_color: #3a4a6a
        instance fill_color: #3B82F6
        instance border_radius: 3.0

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);

            // Track background
            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
            sdf.fill(self.track_color);

            // Progress fill
            let fill_width = self.rect_size.x * self.progress;
            if fill_width > 0.0 {
                sdf.box(0.0, 0.0, fill_width, self.rect_size.y, self.border_radius);
                sdf.fill(self.fill_color);
            }

            return sdf.result;
        }
    }

    DrawA2uiSliderThumb = {{DrawA2uiSliderThumb}} {
        instance thumb_color: #FFFFFF
        instance shadow_color: #00000040

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let radius = min(self.rect_size.x, self.rect_size.y) * 0.5;
            let center = self.rect_size * 0.5;

            // Shadow
            sdf.circle(center.x, center.y + 1.0, radius - 1.0);
            sdf.fill(self.shadow_color);

            // Thumb circle
            let thumb_scale = 1.0 + self.hover * 0.1 - self.pressed * 0.05;
            sdf.circle(center.x, center.y, (radius - 2.0) * thumb_scale);
            sdf.fill(self.thumb_color);

            return sdf.result;
        }
    }

    // ============================================================================
    // A2UI Chart Drawing Primitives (adapted from makepad-d3)
    // ============================================================================

    // Line chart segment shader (used by chord chart)
    DrawA2uiChartLine = {{DrawA2uiChartLine}} {
        fn pixel(self) -> vec4 {
            let uv = self.pos;

            // Line endpoints in normalized coordinates (0-1)
            let p1 = vec2(self.x1, self.y1);
            let p2 = vec2(self.x2, self.y2);
            let p = uv;

            // Vector from p1 to p2
            let line_vec = p2 - p1;
            let line_len = length(line_vec);

            if line_len < 0.001 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }

            // Project point onto line
            let t = clamp(dot(p - p1, line_vec) / (line_len * line_len), 0.0, 1.0);
            let closest = p1 + t * line_vec;

            // Distance from point to line
            let dist = length(p - closest);

            // Line width in normalized coordinates
            let half_width = self.line_width * 0.5;

            // Anti-aliased edge
            let aa = 0.02;
            let alpha = 1.0 - smoothstep(half_width - aa, half_width + aa, dist);

            if alpha < 0.01 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }

            return vec4(self.color.rgb * self.color.a * alpha, self.color.a * alpha);
        }
    }

    // Pie/donut chart arc shader
    DrawA2uiArc = {{DrawA2uiArc}} {
        fn pixel(self) -> vec4 {
            let pi_val = 3.14159265;
            let two_pi_val = 6.28318530;

            // Center of the quad is at (0.5, 0.5)
            let px = self.pos.x - 0.5;
            let py = self.pos.y - 0.5;

            // Distance from center
            let distance = sqrt(px * px + py * py);

            // Radii in normalized space
            let inner_rad = self.inner_radius * 0.5;
            let outer_rad = 0.5;

            // Distance mask
            let dist_mask = step(inner_rad, distance) * step(distance, outer_rad);

            // Angle using atan2
            let pixel_ang = atan(py, px);

            // Angle calculation
            let sweep_val = self.end_angle - self.start_angle;
            let rel_ang = pixel_ang - self.start_angle;
            let norm_ang = rel_ang + two_pi_val * 4.0;
            let wrap_ang = mod(norm_ang, two_pi_val);

            // Angle mask
            let ang_mask = step(wrap_ang, sweep_val) * step(0.001, sweep_val);

            // Combined mask
            let final_mask = dist_mask * ang_mask;

            // Anti-aliased edges
            let edge_aa = 0.008;
            let outer_aa = 1.0 - smoothstep(outer_rad - edge_aa, outer_rad + edge_aa, distance);
            let inner_aa = smoothstep(inner_rad - edge_aa, inner_rad + edge_aa, distance);
            let aa_alpha = outer_aa * inner_aa;

            let alpha_val = final_mask * aa_alpha;

            return vec4(self.color.rgb * alpha_val, self.color.a * alpha_val);
        }
    }

    // Arbitrary convex quadrilateral shader (used by chord chart) (for chord ribbons, sankey bands)
    // Renders pixels inside the quad. AA only on edges 0,2 (boundary);
    // edges 1,3 (crossbars shared between adjacent quads) use hard clip.
    DrawA2uiQuad = {{DrawA2uiQuad}} {
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

            // AA only on boundary edges (0: top curve, 2: bottom curve)
            // Crossbar edges (1, 3) get hard clip — no AA to prevent seams
            let len0 = max(length(e0), 0.001);
            let len2 = max(length(e2), 0.001);
            let d0 = abs(c0) / len0;
            let d2 = abs(c2) / len2;
            let min_dist = min(d0, d2);
            let aa = smoothstep(0.0, 1.5, min_dist);

            let alpha = aa * self.opacity;
            return vec4(self.color.rgb * self.color.a * alpha, self.color.a * alpha);
    // A2UI Audio Bars - Neon waveform visualization for audio player
    // ============================================================================
    DrawAudioBars = {{DrawAudioBars}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);

            // Draw dark background with rounded corners
            sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, 4.0);
            sdf.fill(vec4(0.08, 0.08, 0.15, 0.9));

            // 5 bars configuration
            let num_bars = 5.0;
            let gap = 3.0;
            let padding = 4.0;
            let usable_width = self.rect_size.x - padding * 2.0;
            let bar_width = (usable_width - gap * (num_bars - 1.0)) / num_bars;

            // Draw each bar
            for i in 0..5 {
                let fi = float(i);
                let x = padding + fi * (bar_width + gap);

                // Dynamic height based on time and bar index
                let phase = fi * 1.2;
                let wave = sin(self.time * 5.0 + phase) * 0.5 + 0.5;
                let bar_max_height = self.rect_size.y - padding * 2.0;
                let height = mix(0.2, wave, self.is_playing) * bar_max_height;

                // Neon gradient: cyan → purple → pink
                let t = fi / 4.0;
                let cyan = vec3(0.0, 1.0, 1.0);
                let purple = vec3(0.5, 0.0, 1.0);
                let pink = vec3(1.0, 0.2, 0.6);
                let color = mix(mix(cyan, purple, t), pink, t * t);

                // Draw bar from bottom
                let y = self.rect_size.y - padding - height;
                sdf.box(x, y, bar_width, height, 1.5);
                sdf.fill(vec4(color, 1.0));
            }

            return sdf.result;
        }
    }

    // A2UI Surface - Root container for A2UI component rendering
    pub A2uiSurface = {{A2uiSurface}} {
        width: Fill
        height: Fill
        flow: Down

        draw_bg: {
            instance bg_color: #1a1a2e

            fn pixel(self) -> vec4 {
                return self.bg_color;
            }
        }

        // Text rendering settings (for text outside cards)
        draw_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
                line_spacing: 1.4
            }
            color: #FFFFFF
        }

        // Text rendering for content inside cards (separate draw item)
        draw_card_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
                line_spacing: 1.4
            }
            color: #FFFFFF
        }

        // Card background
        draw_card: {
            color: #2a3a5a
            instance border_color: #5588bb
            instance border_radius: 8.0
            instance border_width: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    max(1.0, self.border_radius)
                );
                sdf.fill_keep(self.color);
                sdf.stroke(self.border_color, self.border_width);
                return sdf.result;
            }
        }

        // Button background with rounded corners
        draw_button: {
            instance border_radius: 6.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, self.border_radius);
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        // Text rendering for button labels (drawn after button background)
        draw_button_text: {
            text_style: <THEME_FONT_BOLD> {
                font_size: 14.0
                line_spacing: 1.4
            }
            color: #FFFFFF
        }

        // Image placeholder background
        draw_image_placeholder: {
            instance border_radius: 4.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, self.border_radius);

                // Diagonal stripes pattern for placeholder
                let stripe_width = 8.0;
                let pos = self.pos * self.rect_size;
                let stripe = mod(pos.x + pos.y, stripe_width * 2.0);
                let is_stripe = step(stripe_width, stripe);

                let color1 = vec4(0.25, 0.28, 0.35, 1.0);  // Dark gray
                let color2 = vec4(0.30, 0.33, 0.40, 1.0);  // Slightly lighter

                let bg_color = mix(color1, color2, is_stripe);
                sdf.fill(bg_color);
                return sdf.result;
            }
        }

        // Text for image placeholder label
        draw_image_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 11.0
            }
            color: #888888
        }

        // Actual image drawing
        draw_image: <DrawA2uiImage> {}

        // TextField background
        draw_text_field: <DrawA2uiTextField> {
            border_color: #5588bb
            bg_color: #2a3a5a
        }

        // TextField input text
        draw_text_field_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }
            color: #FFFFFF
        }

        // TextField placeholder text
        draw_text_field_placeholder: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }
            color: #888888
        }

        // Checkbox drawing
        draw_checkbox: <DrawA2uiCheckBox> {
            border_color: #5588bb
            bg_color: #2a3a5a
            check_color: #3B82F6
        }

        // Checkbox label text
        draw_checkbox_label: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }
            color: #FFFFFF
        }

        // Slider track
        draw_slider_track: <DrawA2uiSliderTrack> {
            track_color: #3a4a6a
            fill_color: #3B82F6
        }

        // Slider thumb
        draw_slider_thumb: <DrawA2uiSliderThumb> {
            thumb_color: #FFFFFF
        }

        // Chart drawing (chord chart custom shaders)
        draw_chart_line: <DrawA2uiChartLine> {}
        draw_chart_arc: <DrawA2uiArc> {}
        draw_chart_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 10.0
            }
            color: #AABBCC
        }
        draw_chart_quad: <DrawA2uiQuad> {}

        // makepad-plot chart widgets
        plot_line: <LinePlot> {}
        plot_bar: <BarPlot> {}
        plot_scatter: <ScatterPlot> {}
        plot_pie: <PieChart> {}
        plot_area: <AreaChart> {}
        plot_radar: <RadarChart> {}
        plot_gauge: <GaugeChart> {}
        plot_bubble: <BubbleChart> {}
        plot_candlestick: <CandlestickChart> {}
        plot_heatmap: <HeatmapChart> {}
        plot_treemap: <Treemap> {}
        plot_sankey: <SankeyDiagram> {}
        plot_histogram: <HistogramChart> {}
        plot_boxplot: <BoxPlotChart> {}
        plot_donut: <DonutChart> {}
        plot_stem: <StemPlot> {}
        plot_violin: <ViolinPlot> {}
        plot_polar: <PolarPlot> {}
        plot_contour: <ContourPlot> {}
        plot_waterfall: <WaterfallChart> {}
        plot_funnel: <FunnelChart> {}
        plot_step: <StepPlot> {}
        plot_stackplot: <Stackplot> {}
        plot_hexbin: <HexbinChart> {}
        plot_streamgraph: <Streamgraph> {}
        plot_surface3d: <Surface3D> {}
        plot_scatter3d: <Scatter3D> {}
        plot_line3d: <Line3D> {}
        // Audio bars visualization
        draw_audio_bars: <DrawAudioBars> {}

        // Image resources
        img_headphones: dep("crate://self/resources/headphones.jpg")
        img_mouse: dep("crate://self/resources/mouse.jpg")
        img_keyboard: dep("crate://self/resources/keyboard.jpg")
        img_alipay: dep("crate://self/resources/alipay.png")
        img_wechat: dep("crate://self/resources/wechat.png")
    }

    // A2UI Text component
    pub A2uiText = {{A2uiText}} {
        width: Fit
        height: Fit

        draw_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
                line_spacing: 1.4
            }
            color: (FOREGROUND)
        }
    }

    // A2UI Column layout
    pub A2uiColumn = {{A2uiColumn}} {
        width: Fill
        height: Fit
        flow: Down
        spacing: 8.0
    }

    // A2UI Row layout
    pub A2uiRow = {{A2uiRow}} {
        width: Fill
        height: Fit
        flow: Right
        spacing: 8.0
        align: { y: 0.5 }
    }

    // A2UI Card container
    pub A2uiCard = {{A2uiCard}} {
        width: Fill
        height: Fit
        flow: Down
        padding: 16.0
        margin: { top: 4.0, bottom: 4.0 }

        show_bg: true
        draw_bg: {
            instance radius: 8.0
            instance border_width: 1.0
            instance border_color: (BORDER)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.border_width,
                    self.border_width,
                    self.rect_size.x - self.border_width * 2.0,
                    self.rect_size.y - self.border_width * 2.0,
                    max(1.0, self.radius)
                );

                sdf.fill_keep(#FFFFFF);
                sdf.stroke(self.border_color, self.border_width);

                return sdf.result;
            }
        }
    }

    // A2UI Button
    pub A2uiButton = {{A2uiButton}} {
        width: Fit
        height: Fit
        align: { x: 0.5, y: 0.5 }
        padding: { left: 16, right: 16, top: 8, bottom: 8 }

        draw_bg: {
            instance radius: 6.0
            instance hover: 0.0
            instance pressed: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(1.0, 1.0, self.rect_size.x - 2.0, self.rect_size.y - 2.0, self.radius);

                let base_color = vec4(0.231, 0.51, 0.965, 1.0);
                let hover_color = vec4(0.145, 0.388, 0.922, 1.0);
                let pressed_color = vec4(0.114, 0.306, 0.847, 1.0);

                let color = mix(base_color, hover_color, self.hover);
                let final_color = mix(color, pressed_color, self.pressed);

                sdf.fill(final_color);
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
            color: #FFFFFF
        }

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_bg: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.15 } }
                    apply: { draw_bg: { hover: 1.0 } }
                }
            }
            pressed = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { pressed: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { pressed: 1.0 } }
                }
            }
        }
    }
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
pub struct DrawAudioBars {
    #[deref]
    draw_super: DrawQuad,
    #[live(0.0)]
    pub is_playing: f32,
}

// ============================================================================
// A2UI Surface Widget
// ============================================================================

/// The root container for rendering A2UI component trees.
#[derive(Live, LiveHook, Widget)]
pub struct A2uiSurface {
    #[redraw]
    #[live]
    draw_bg: DrawQuad,

    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    /// Draw text for rendering text components (outside cards)
    #[live]
    draw_text: DrawText,

    /// Draw text for content inside cards (separate draw item for correct z-order)
    #[live]
    draw_card_text: DrawText,

    /// Draw card background
    #[redraw]
    #[live]
    draw_card: DrawColor,

    /// Draw button background (with rounded corners shader)
    #[redraw]
    #[live]
    draw_button: DrawColor,

    /// Draw text for button labels (drawn after button background)
    #[live]
    draw_button_text: DrawText,

    /// Draw image placeholder background
    #[redraw]
    #[live]
    draw_image_placeholder: DrawColor,

    /// Draw text for image placeholder
    #[live]
    draw_image_text: DrawText,

    /// Draw actual image
    #[redraw]
    #[live]
    draw_image: DrawA2uiImage,

    /// Draw text field background
    #[redraw]
    #[live]
    draw_text_field: DrawA2uiTextField,

    /// Draw text for text field input
    #[live]
    draw_text_field_text: DrawText,

    /// Draw text for text field placeholder
    #[live]
    draw_text_field_placeholder: DrawText,

    /// Draw checkbox
    #[redraw]
    #[live]
    draw_checkbox: DrawA2uiCheckBox,

    /// Draw checkbox label
    #[live]
    draw_checkbox_label: DrawText,

    /// Draw slider track
    #[redraw]
    #[live]
    draw_slider_track: DrawA2uiSliderTrack,

    /// Draw slider thumb
    #[redraw]
    #[live]
    draw_slider_thumb: DrawA2uiSliderThumb,

    /// Draw chart line segment (chord chart)
    #[redraw]
    #[live]
    draw_chart_line: DrawA2uiChartLine,

    /// Draw chart arc (chord chart)
    #[redraw]
    #[live]
    draw_chart_arc: DrawA2uiArc,

    /// Draw chart text (chord chart labels)
    #[live]
    draw_chart_text: DrawText,

    /// Draw chart arbitrary quadrilateral (chord ribbons)
    #[redraw]
    #[live]
    draw_chart_quad: DrawA2uiQuad,

    // makepad-plot chart widget instances
    #[live] plot_line: LinePlot,
    #[live] plot_bar: BarPlot,
    #[live] plot_scatter: ScatterPlot,
    #[live] plot_pie: PieChart,
    #[live] plot_area: AreaChart,
    #[live] plot_radar: RadarChart,
    #[live] plot_gauge: GaugeChart,
    #[live] plot_bubble: BubbleChart,
    #[live] plot_candlestick: CandlestickChart,
    #[live] plot_heatmap: HeatmapChart,
    #[live] plot_treemap: Treemap,
    #[live] plot_sankey: SankeyDiagram,
    #[live] plot_histogram: HistogramChart,
    #[live] plot_boxplot: BoxPlotChart,
    #[live] plot_donut: DonutChart,
    #[live] plot_stem: StemPlot,
    #[live] plot_violin: ViolinPlot,
    #[live] plot_polar: PolarPlot,
    #[live] plot_contour: ContourPlot,
    #[live] plot_waterfall: WaterfallChart,
    #[live] plot_funnel: FunnelChart,
    #[live] plot_step: StepPlot,
    #[live] plot_stackplot: Stackplot,
    #[live] plot_hexbin: HexbinChart,
    #[live] plot_streamgraph: Streamgraph,
    #[live] plot_surface3d: Surface3D,
    #[live] plot_scatter3d: Scatter3D,
    #[live] plot_line3d: Line3D,
    /// Draw audio bars visualization
    #[redraw]
    #[live]
    draw_audio_bars: DrawAudioBars,

    /// Image sources (preloaded)
    #[live]
    img_headphones: LiveDependency,
    #[live]
    img_mouse: LiveDependency,
    #[live]
    img_keyboard: LiveDependency,
    #[live]
    img_alipay: LiveDependency,
    #[live]
    img_wechat: LiveDependency,

    /// Loaded textures for images
    #[rust]
    texture_headphones: Option<Texture>,
    #[rust]
    texture_mouse: Option<Texture>,
    #[rust]
    texture_keyboard: Option<Texture>,
    #[rust]
    texture_alipay: Option<Texture>,
    #[rust]
    texture_wechat: Option<Texture>,

    /// Surface ID
    #[live]
    surface_id: LiveValue,

    /// The message processor (manages surfaces and data models)
    #[rust]
    processor: Option<A2uiMessageProcessor>,

    #[rust]
    area: Area,

    /// Flag to track if we're inside a card context (for correct text draw ordering)
    #[rust]
    inside_card: bool,

    /// Flag to track if we're inside a button context
    #[rust]
    inside_button: bool,

    /// Button areas for event.hits() detection - each button has independent Area
    #[rust]
    button_areas: Vec<Area>,

    /// Button metadata: (component_id, Option<ActionDefinition>, Option<scope>)
    #[rust]
    button_data: Vec<(String, Option<ActionDefinition>, Option<String>)>,

    /// Currently hovered button index (only one at a time)
    #[rust]
    hovered_button_idx: Option<usize>,

    /// Currently pressed button index (only one at a time)
    #[rust]
    pressed_button_idx: Option<usize>,

    /// Current template scope path for relative path resolution
    /// When rendering inside a template, this is set to the item path (e.g., "/products/0")
    #[rust]
    current_scope: Option<String>,

    // ============================================================================
    // TextField state tracking
    // ============================================================================

    /// TextField areas for event detection
    #[rust]
    text_field_areas: Vec<Area>,

    /// TextField metadata: (component_id, binding_path, current_value)
    #[rust]
    text_field_data: Vec<(String, Option<String>, String)>,

    /// Currently focused text field index
    #[rust]
    focused_text_field_idx: Option<usize>,

    /// Text input buffer for focused field
    #[rust]
    text_input_buffer: String,

    /// Cursor position in text input
    #[rust]
    cursor_pos: usize,

    // ============================================================================
    // CheckBox state tracking
    // ============================================================================

    /// CheckBox areas for event detection
    #[rust]
    checkbox_areas: Vec<Area>,

    /// CheckBox metadata: (component_id, binding_path, current_value)
    #[rust]
    checkbox_data: Vec<(String, Option<String>, bool)>,

    /// Currently hovered checkbox index
    #[rust]
    hovered_checkbox_idx: Option<usize>,

    // ============================================================================
    // Slider state tracking
    // ============================================================================

    /// Slider areas for event detection
    #[rust]
    slider_areas: Vec<Area>,

    /// Slider metadata: (component_id, binding_path, min, max, current_value)
    #[rust]
    slider_data: Vec<(String, Option<String>, f64, f64, f64)>,

    /// Currently dragging slider index
    #[rust]
    dragging_slider_idx: Option<usize>,

    /// Currently hovered slider index
    #[rust]
    hovered_slider_idx: Option<usize>,

    // ============================================================================
    // AudioPlayer state tracking
    // ============================================================================

    /// AudioPlayer button areas for event detection (play buttons)
    #[rust]
    audio_player_areas: Vec<Area>,

    /// AudioPlayer metadata: (component_id, audio_url, title)
    #[rust]
    audio_player_data: Vec<(String, String, String)>,

    /// Currently hovered audio player index
    #[rust]
    hovered_audio_player_idx: Option<usize>,

    /// Currently playing audio component ID (for Play/Stop toggle)
    #[rust]
    playing_component_id: Option<String>,
}

impl A2uiSurface {
    /// Initialize the surface with a processor
    pub fn init_processor(&mut self) {
        if self.processor.is_none() {
            self.processor = Some(A2uiMessageProcessor::with_standard_catalog());
        }
    }

    /// Clear all surfaces and reset the processor
    pub fn clear(&mut self) {
        // Reset the processor to clear all surfaces and components
        self.processor = Some(A2uiMessageProcessor::with_standard_catalog());
    }

    /// Apply theme colors to all A2UI components
    pub fn set_theme_colors(&mut self, cx: &mut Cx, colors: &A2uiThemeColors) {
        // Apply surface background
        self.draw_bg.apply_over(cx, live! {
            bg_color: (colors.bg_surface)
        });

        // Apply text colors
        self.draw_text.apply_over(cx, live! {
            color: (colors.text_primary)
        });

        self.draw_card_text.apply_over(cx, live! {
            color: (colors.text_primary)
        });

        // Apply card colors
        self.draw_card.apply_over(cx, live! {
            color: (colors.bg_card)
            border_color: (colors.border_color)
        });

        // Apply button colors - the shader uses hardcoded colors, so we update via instance
        self.draw_button.apply_over(cx, live! {
            color: (colors.accent)
        });

        self.draw_button_text.apply_over(cx, live! {
            color: (vec4(1.0, 1.0, 1.0, 1.0))
        });

        // Apply text field colors
        self.draw_text_field.apply_over(cx, live! {
            bg_color: (colors.input_bg)
            border_color: (colors.border_color)
        });

        self.draw_text_field_text.apply_over(cx, live! {
            color: (colors.text_primary)
        });

        self.draw_text_field_placeholder.apply_over(cx, live! {
            color: (colors.text_secondary)
        });

        // Apply checkbox colors
        self.draw_checkbox.apply_over(cx, live! {
            bg_color: (colors.input_bg)
            border_color: (colors.border_color)
            check_color: (colors.control_fill)
        });

        self.draw_checkbox_label.apply_over(cx, live! {
            color: (colors.text_primary)
        });

        // Apply slider colors
        self.draw_slider_track.apply_over(cx, live! {
            track_color: (colors.slider_track)
            fill_color: (colors.control_fill)
        });

        // Apply image placeholder text
        self.draw_image_text.apply_over(cx, live! {
            color: (colors.text_secondary)
        });
    }

    /// Load image textures from LiveDependency resources
    fn load_image_textures(&mut self, cx: &mut Cx) {
        use makepad_widgets::image_cache::ImageBuffer;

        // Load headphones image (JPG)
        if self.texture_headphones.is_none() {
            let path = self.img_headphones.as_str();
            if !path.is_empty() {
                if let Ok(data) = cx.get_dependency(path) {
                    if let Ok(image) = ImageBuffer::from_jpg(&data) {
                        self.texture_headphones = Some(image.into_new_texture(cx));
                    }
                }
            }
        }

        // Load mouse image (JPG)
        if self.texture_mouse.is_none() {
            let path = self.img_mouse.as_str();
            if !path.is_empty() {
                if let Ok(data) = cx.get_dependency(path) {
                    if let Ok(image) = ImageBuffer::from_jpg(&data) {
                        self.texture_mouse = Some(image.into_new_texture(cx));
                    }
                }
            }
        }

        // Load keyboard image (JPG)
        if self.texture_keyboard.is_none() {
            let path = self.img_keyboard.as_str();
            if !path.is_empty() {
                if let Ok(data) = cx.get_dependency(path) {
                    if let Ok(image) = ImageBuffer::from_jpg(&data) {
                        self.texture_keyboard = Some(image.into_new_texture(cx));
                    }
                }
            }
        }

        // Load Alipay icon (PNG)
        if self.texture_alipay.is_none() {
            let path = self.img_alipay.as_str();
            if !path.is_empty() {
                if let Ok(data) = cx.get_dependency(path) {
                    if let Ok(image) = ImageBuffer::from_png(&data) {
                        self.texture_alipay = Some(image.into_new_texture(cx));
                    }
                }
            }
        }

        // Load WeChat icon (PNG)
        if self.texture_wechat.is_none() {
            let path = self.img_wechat.as_str();
            if !path.is_empty() {
                if let Ok(data) = cx.get_dependency(path) {
                    if let Ok(image) = ImageBuffer::from_png(&data) {
                        self.texture_wechat = Some(image.into_new_texture(cx));
                    }
                }
            }
        }
    }

    /// Get texture index for a given URL (0=headphones, 1=mouse, 2=keyboard, 3=alipay, 4=wechat, None=not found)
    fn get_texture_index_for_url(&self, url: &str) -> Option<usize> {
        if url.contains("headphones") && self.texture_headphones.is_some() {
            Some(0)
        } else if url.contains("mouse") && self.texture_mouse.is_some() {
            Some(1)
        } else if url.contains("keyboard") && self.texture_keyboard.is_some() {
            Some(2)
        } else if url.contains("alipay") && self.texture_alipay.is_some() {
            Some(3)
        } else if url.contains("wechat") && self.texture_wechat.is_some() {
            Some(4)
        } else {
            None
        }
    }

    /// Get the processor
    pub fn processor(&self) -> Option<&A2uiMessageProcessor> {
        self.processor.as_ref()
    }

    /// Get mutable processor
    pub fn processor_mut(&mut self) -> Option<&mut A2uiMessageProcessor> {
        self.processor.as_mut()
    }

    /// Set the currently playing audio component ID (for Play/Stop toggle display)
    pub fn set_playing_component(&mut self, component_id: Option<String>) {
        self.playing_component_id = component_id;
    }

    /// Get the currently playing audio component ID
    pub fn playing_component_id(&self) -> Option<&String> {
        self.playing_component_id.as_ref()
    }

    /// Process A2UI JSON messages
    pub fn process_json(&mut self, json: &str) -> Result<Vec<ProcessorEvent>, serde_json::Error> {
        self.init_processor();
        if let Some(processor) = self.processor.as_mut() {
            processor.process_json(json)
        } else {
            Ok(vec![])
        }
    }

    /// Process a single A2UI message
    pub fn process_message(&mut self, message: A2uiMessage) -> Vec<ProcessorEvent> {
        self.init_processor();
        if let Some(processor) = self.processor.as_mut() {
            processor.process_message(message)
        } else {
            vec![]
        }
    }

    /// Get the current surface ID
    fn get_surface_id(&self) -> String {
        // For now, use "main" as default
        "main".to_string()
    }
}

impl Widget for A2uiSurface {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Forward events to 3D chart widgets FIRST for interactive rotation/zoom
        self.plot_surface3d.handle_event(cx, event, scope);
        self.plot_scatter3d.handle_event(cx, event, scope);
        self.plot_line3d.handle_event(cx, event, scope);

        let mut needs_redraw = false;
        let surface_id = self.get_surface_id();

        // Handle text input events for focused text field
        if let Some(focused_idx) = self.focused_text_field_idx {
            if let Event::TextInput(te) = event {
                // Insert text at cursor position
                self.text_input_buffer.insert_str(self.cursor_pos, &te.input);
                self.cursor_pos += te.input.len();
                needs_redraw = true;

                // Emit data model change
                if let Some((_, binding_path, _)) = self.text_field_data.get(focused_idx) {
                    if let Some(path) = binding_path {
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            A2uiSurfaceAction::DataModelChanged {
                                surface_id: surface_id.clone(),
                                path: path.clone(),
                                value: serde_json::Value::String(self.text_input_buffer.clone()),
                            },
                        );
                    }
                }
            }

            if let Event::KeyDown(ke) = event {
                match ke.key_code {
                    KeyCode::Backspace => {
                        if self.cursor_pos > 0 {
                            self.cursor_pos -= 1;
                            self.text_input_buffer.remove(self.cursor_pos);
                            needs_redraw = true;

                            // Emit data model change
                            if let Some((_, binding_path, _)) = self.text_field_data.get(focused_idx) {
                                if let Some(path) = binding_path {
                                    cx.widget_action(
                                        self.widget_uid(),
                                        &scope.path,
                                        A2uiSurfaceAction::DataModelChanged {
                                            surface_id: surface_id.clone(),
                                            path: path.clone(),
                                            value: serde_json::Value::String(self.text_input_buffer.clone()),
                                        },
                                    );
                                }
                            }
                        }
                    }
                    KeyCode::Delete => {
                        if self.cursor_pos < self.text_input_buffer.len() {
                            self.text_input_buffer.remove(self.cursor_pos);
                            needs_redraw = true;

                            if let Some((_, binding_path, _)) = self.text_field_data.get(focused_idx) {
                                if let Some(path) = binding_path {
                                    cx.widget_action(
                                        self.widget_uid(),
                                        &scope.path,
                                        A2uiSurfaceAction::DataModelChanged {
                                            surface_id: surface_id.clone(),
                                            path: path.clone(),
                                            value: serde_json::Value::String(self.text_input_buffer.clone()),
                                        },
                                    );
                                }
                            }
                        }
                    }
                    KeyCode::ArrowLeft => {
                        if self.cursor_pos > 0 {
                            self.cursor_pos -= 1;
                            needs_redraw = true;
                        }
                    }
                    KeyCode::ArrowRight => {
                        if self.cursor_pos < self.text_input_buffer.len() {
                            self.cursor_pos += 1;
                            needs_redraw = true;
                        }
                    }
                    KeyCode::Escape => {
                        self.focused_text_field_idx = None;
                        needs_redraw = true;
                    }
                    _ => {}
                }
            }
        }

        // Handle button events
        for (idx, area) in self.button_areas.iter().enumerate() {
            match event.hits(cx, *area) {
                Hit::FingerHoverIn(_) => {
                    if self.hovered_button_idx != Some(idx) {
                        self.hovered_button_idx = Some(idx);
                        cx.set_cursor(MouseCursor::Hand);
                        needs_redraw = true;
                    }
                }
                Hit::FingerHoverOut(_) => {
                    if self.hovered_button_idx == Some(idx) {
                        self.hovered_button_idx = None;
                        cx.set_cursor(MouseCursor::Default);
                        needs_redraw = true;
                    }
                }
                Hit::FingerDown(_) => {
                    self.pressed_button_idx = Some(idx);
                    self.hovered_button_idx = Some(idx);
                    needs_redraw = true;
                }
                Hit::FingerUp(fe) => {
                    if self.pressed_button_idx == Some(idx) {
                        self.pressed_button_idx = None;
                        needs_redraw = true;

                        // Check if released over this button (click confirmed)
                        if fe.is_over {
                            if let Some((component_id, action_def, btn_scope)) =
                                self.button_data.get(idx)
                            {
                                if let Some(action_def) = action_def {
                                    // Create resolved UserAction with data model values
                                    if let Some(processor) = &self.processor {
                                        let user_action = processor.create_action(
                                            &surface_id,
                                            component_id,
                                            action_def,
                                            btn_scope.as_deref(),
                                        );
                                        // Emit widget action for app layer to handle
                                        cx.widget_action(
                                            self.widget_uid(),
                                            &scope.path,
                                            A2uiSurfaceAction::UserAction(user_action),
                                        );
                                    }
                                }
                            }
                            self.hovered_button_idx = Some(idx);
                        } else {
                            self.hovered_button_idx = None;
                            cx.set_cursor(MouseCursor::Default);
                        }
                    }
                }
                _ => {}
            }
        }

        // Handle text field events
        for (idx, area) in self.text_field_areas.iter().enumerate() {
            match event.hits(cx, *area) {
                Hit::FingerDown(_) => {
                    // Focus this text field
                    self.focused_text_field_idx = Some(idx);
                    if let Some((_, _, current_value)) = self.text_field_data.get(idx) {
                        self.text_input_buffer = current_value.clone();
                        self.cursor_pos = self.text_input_buffer.len();
                    }
                    cx.set_key_focus(self.area);
                    needs_redraw = true;
                }
                _ => {}
            }
        }

        // Handle checkbox events
        for (idx, area) in self.checkbox_areas.iter().enumerate() {
            match event.hits(cx, *area) {
                Hit::FingerHoverIn(_) => {
                    if self.hovered_checkbox_idx != Some(idx) {
                        self.hovered_checkbox_idx = Some(idx);
                        cx.set_cursor(MouseCursor::Hand);
                        needs_redraw = true;
                    }
                }
                Hit::FingerHoverOut(_) => {
                    if self.hovered_checkbox_idx == Some(idx) {
                        self.hovered_checkbox_idx = None;
                        cx.set_cursor(MouseCursor::Default);
                        needs_redraw = true;
                    }
                }
                Hit::FingerDown(_) => {
                    // Must handle FingerDown to receive FingerUp
                    self.hovered_checkbox_idx = Some(idx);
                    needs_redraw = true;
                }
                Hit::FingerUp(fe) => {
                    if fe.is_over {
                        // Toggle checkbox value
                        if let Some((_, binding_path, current_value)) =
                            self.checkbox_data.get(idx).cloned()
                        {
                            let new_value = !current_value;
                            if let Some(path) = binding_path {
                                cx.widget_action(
                                    self.widget_uid(),
                                    &scope.path,
                                    A2uiSurfaceAction::DataModelChanged {
                                        surface_id: surface_id.clone(),
                                        path,
                                        value: serde_json::Value::Bool(new_value),
                                    },
                                );
                            }
                        }
                        needs_redraw = true;
                    }
                }
                _ => {}
            }
        }

        // Handle audio player events
        for (idx, area) in self.audio_player_areas.iter().enumerate() {
            match event.hits(cx, *area) {
                Hit::FingerHoverIn(_) => {
                    if self.hovered_audio_player_idx != Some(idx) {
                        self.hovered_audio_player_idx = Some(idx);
                        cx.set_cursor(MouseCursor::Hand);
                        needs_redraw = true;
                    }
                }
                Hit::FingerHoverOut(_) => {
                    if self.hovered_audio_player_idx == Some(idx) {
                        self.hovered_audio_player_idx = None;
                        cx.set_cursor(MouseCursor::Default);
                        needs_redraw = true;
                    }
                }
                Hit::FingerDown(_) => {
                    log!("[AudioPlayer] Click idx={}", idx);
                    self.hovered_audio_player_idx = Some(idx);
                    // Trigger play immediately on FingerDown
                    if let Some((component_id, url, title)) = self.audio_player_data.get(idx).cloned() {
                        log!("[AudioPlayer] Emitting PlayAudio: {} - {}", title, url);
                        cx.widget_action(
                            self.widget_uid(),
                            &scope.path,
                            A2uiSurfaceAction::PlayAudio {
                                component_id,
                                url,
                                title,
                            },
                        );
                    }
                    needs_redraw = true;
                }
                _ => {}
            }
        }

        // Handle slider events
        for (idx, area) in self.slider_areas.iter().enumerate() {
            match event.hits(cx, *area) {
                Hit::FingerHoverIn(_) => {
                    if self.hovered_slider_idx != Some(idx) {
                        self.hovered_slider_idx = Some(idx);
                        cx.set_cursor(MouseCursor::Hand);
                        needs_redraw = true;
                    }
                }
                Hit::FingerHoverOut(_) => {
                    if self.hovered_slider_idx == Some(idx) && self.dragging_slider_idx != Some(idx)
                    {
                        self.hovered_slider_idx = None;
                        cx.set_cursor(MouseCursor::Default);
                        needs_redraw = true;
                    }
                }
                Hit::FingerDown(fe) => {
                    self.dragging_slider_idx = Some(idx);
                    self.hovered_slider_idx = Some(idx);

                    // Calculate value from position
                    if let Some((_, binding_path, min, max, _)) = self.slider_data.get(idx).cloned()
                    {
                        let rect = area.rect(cx);
                        let rel_x = (fe.abs.x - rect.pos.x) / rect.size.x;
                        let new_value = min + (max - min) * rel_x.clamp(0.0, 1.0);

                        if let Some(path) = binding_path {
                            cx.widget_action(
                                self.widget_uid(),
                                &scope.path,
                                A2uiSurfaceAction::DataModelChanged {
                                    surface_id: surface_id.clone(),
                                    path,
                                    value: serde_json::json!(new_value),
                                },
                            );
                        }
                    }
                    needs_redraw = true;
                }
                Hit::FingerMove(fe) => {
                    if self.dragging_slider_idx == Some(idx) {
                        if let Some((_, binding_path, min, max, _)) =
                            self.slider_data.get(idx).cloned()
                        {
                            let rect = area.rect(cx);
                            let rel_x = (fe.abs.x - rect.pos.x) / rect.size.x;
                            let new_value = min + (max - min) * rel_x.clamp(0.0, 1.0);

                            if let Some(path) = binding_path {
                                cx.widget_action(
                                    self.widget_uid(),
                                    &scope.path,
                                    A2uiSurfaceAction::DataModelChanged {
                                        surface_id: surface_id.clone(),
                                        path,
                                        value: serde_json::json!(new_value),
                                    },
                                );
                            }
                        }
                        needs_redraw = true;
                    }
                }
                Hit::FingerUp(_) => {
                    if self.dragging_slider_idx == Some(idx) {
                        self.dragging_slider_idx = None;
                        needs_redraw = true;
                    }
                }
                _ => {}
            }
        }

        if needs_redraw {
            self.redraw(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Load image textures if not loaded yet
        self.load_image_textures(cx);

        // Clear component data from previous frame
        // Keep areas - they will be updated in render_* to maintain event tracking
        self.button_data.clear();
        self.text_field_data.clear();
        self.checkbox_data.clear();
        self.slider_data.clear();
        self.audio_player_data.clear();

        self.draw_bg.begin(cx, walk, self.layout);

        // Get surface and data model - clone to avoid borrow issues
        let surface_id = self.get_surface_id();
        let render_data = if let Some(processor) = &self.processor {
            let surface_opt = processor.get_surface(&surface_id);
            let data_model_opt = processor.get_data_model(&surface_id);

            // Debug: Log what we found
            if surface_opt.is_none() {
                log!("[draw_walk] No surface found for id: {}", surface_id);
            }
            if data_model_opt.is_none() {
                log!("[draw_walk] No data model found for id: {}", surface_id);
            }

            if let (Some(surface), Some(data_model)) = (surface_opt, data_model_opt) {
                log!("[draw_walk] Found surface with root: {}, {} components", surface.root, surface.components.len());
                Some((surface.clone(), data_model.clone()))
            } else {
                None
            }
        } else {
            log!("[draw_walk] No processor!");
            None
        };

        // Render the component tree
        if let Some((surface, data_model)) = render_data {
            let root_id = surface.root.clone();
            if !root_id.is_empty() {
                self.render_component(cx, scope, &surface, &data_model, &root_id);
            }
        }

        // Trim areas if we have fewer components this frame
        let current_button_count = self.button_data.len();
        if current_button_count < self.button_areas.len() {
            self.button_areas.truncate(current_button_count);
        }

        let current_text_field_count = self.text_field_data.len();
        if current_text_field_count < self.text_field_areas.len() {
            self.text_field_areas.truncate(current_text_field_count);
        }

        let current_checkbox_count = self.checkbox_data.len();
        if current_checkbox_count < self.checkbox_areas.len() {
            self.checkbox_areas.truncate(current_checkbox_count);
        }

        let current_slider_count = self.slider_data.len();
        if current_slider_count < self.slider_areas.len() {
            self.slider_areas.truncate(current_slider_count);
        }

        let current_audio_player_count = self.audio_player_data.len();
        if current_audio_player_count < self.audio_player_areas.len() {
            self.audio_player_areas.truncate(current_audio_player_count);
        }

        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();

        DrawStep::done()
    }
}

impl A2uiSurface {
    /// Render a component and its children recursively
    fn render_component(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &super::processor::Surface,
        data_model: &DataModel,
        component_id: &str,
    ) {
        let Some(component_def) = surface.get_component(component_id) else {
            return;
        };

        // Clone component data to avoid borrow issues
        let component = component_def.component.clone();

        match &component {
            ComponentType::Column(col) => {
                self.render_column(cx, scope, surface, data_model, col);
            }
            ComponentType::Row(row) => {
                self.render_row(cx, scope, surface, data_model, row);
            }
            ComponentType::Text(text) => {
                self.render_text(cx, text, data_model);
            }
            ComponentType::Card(card) => {
                self.render_card(cx, scope, surface, data_model, card);
            }
            ComponentType::Button(btn) => {
                self.render_button(cx, scope, surface, data_model, btn, component_id);
            }
            ComponentType::Image(img) => {
                self.render_image(cx, img, data_model);
            }
            ComponentType::TextField(text_field) => {
                self.render_text_field(cx, text_field, data_model, component_id);
            }
            ComponentType::CheckBox(checkbox) => {
                self.render_checkbox(cx, checkbox, data_model, component_id);
            }
            ComponentType::Slider(slider) => {
                self.render_slider(cx, slider, data_model, component_id);
            }
            ComponentType::List(list) => {
                self.render_list(cx, scope, surface, data_model, list);
            }
            ComponentType::Chart(chart) => {
                self.render_chart(cx, scope, chart, data_model, component_id);
            }
            ComponentType::AudioPlayer(audio_player) => {
                self.render_audio_player(cx, audio_player, data_model, component_id);
            }
            _ => {
                // Unsupported component - skip for now
            }
        }
    }

    fn render_column(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &super::processor::Surface,
        data_model: &DataModel,
        col: &ColumnComponent,
    ) {
        // Start a vertical layout
        let walk = Walk::fill_fit();
        let layout = Layout {
            flow: Flow::Down,
            spacing: 8.0,
            ..Layout::default()
        };

        cx.begin_turtle(walk, layout);

        // Render children
        let children = col.children.clone();
        self.render_children(cx, scope, surface, data_model, &children);

        cx.end_turtle();
    }

    fn render_row(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &super::processor::Surface,
        data_model: &DataModel,
        row: &RowComponent,
    ) {
        // Start a horizontal layout - Fill width to allow spacer pattern
        let walk = Walk::fill_fit();
        let layout = Layout {
            flow: Flow::right(),
            spacing: 16.0,
            align: Align { x: 0.0, y: 0.5 },
            ..Layout::default()
        };

        cx.begin_turtle(walk, layout);

        // Render children with special handling for Row context
        let children = row.children.clone();
        self.render_row_children(cx, scope, surface, data_model, &children);

        cx.end_turtle();
    }

    /// Render children specifically for Row context (horizontal layout)
    /// If last child is a Button, it's placed in a Fill-width container with right alignment
    fn render_row_children(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &super::processor::Surface,
        data_model: &DataModel,
        children: &ChildrenRef,
    ) {
        match children {
            ChildrenRef::ExplicitList(ids) => {
                let len = ids.len();

                // Check if last child is a Button for right-alignment
                let last_is_button = if len > 0 {
                    if let Some(comp) = surface.get_component(&ids[len - 1]) {
                        matches!(comp.component, ComponentType::Button(_))
                    } else {
                        false
                    }
                } else {
                    false
                };

                if last_is_button && len > 1 {
                    // Render non-button children with fixed min-width for alignment
                    // 280px is enough for longest product name
                    for child_id in ids.iter().take(len - 1) {
                        self.render_row_child_with_min_width(cx, scope, surface, data_model, child_id, 280.0);
                    }

                    // Render button
                    self.render_row_child(cx, scope, surface, data_model, &ids[len - 1]);
                } else {
                    // Render all children normally
                    for child_id in ids.iter() {
                        self.render_row_child(cx, scope, surface, data_model, child_id);
                    }
                }
            }
            ChildrenRef::Template { .. } => {
                // For templates in Row, use regular rendering
                self.render_children(cx, scope, surface, data_model, children);
            }
        }
    }

    /// Render a single child in Row context
    fn render_row_child(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &super::processor::Surface,
        data_model: &DataModel,
        component_id: &str,
    ) {
        self.render_row_child_with_min_width(cx, scope, surface, data_model, component_id, 0.0);
    }

    /// Render a single child in Row context with minimum width for Column alignment
    fn render_row_child_with_min_width(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &super::processor::Surface,
        data_model: &DataModel,
        component_id: &str,
        min_width: f64,
    ) {
        let Some(component_def) = surface.get_component(component_id) else {
            return;
        };

        let component = component_def.component.clone();

        match &component {
            ComponentType::Column(col) => {
                // Column with fixed width ensures buttons align
                // Height is Fit to adapt to content
                let walk = if min_width > 0.0 {
                    // Fixed width, Fit height using Walk::new()
                    Walk::new(Size::Fixed(min_width), Size::fit())
                } else {
                    Walk::fit()
                };
                let layout = Layout {
                    flow: Flow::Down,
                    spacing: 4.0,
                    ..Layout::default()
                };

                cx.begin_turtle(walk, layout);

                // Render Column children
                if let ChildrenRef::ExplicitList(ids) = &col.children {
                    for child_id in ids {
                        self.render_component(cx, scope, surface, data_model, child_id);
                    }
                }

                cx.end_turtle();
            }
            _ => {
                // Other components render normally
                self.render_component(cx, scope, surface, data_model, component_id);
            }
        }
    }

    fn render_children(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &super::processor::Surface,
        data_model: &DataModel,
        children: &ChildrenRef,
    ) {
        match children {
            ChildrenRef::ExplicitList(ids) => {
                let ids_clone = ids.clone();
                for child_id in ids_clone {
                    self.render_component(cx, scope, surface, data_model, &child_id);
                }
            }
            ChildrenRef::Template {
                component_id,
                data_binding,
            } => {
                // Get array data from data model
                if let Some(array) = data_model.get_array(data_binding) {
                    let component_id = component_id.clone();
                    let data_binding = data_binding.clone();
                    for (index, _item) in array.iter().enumerate() {
                        // For template rendering, we need to set up item context
                        // For now, just render the template component
                        let item_path = format!("{}/{}", data_binding, index);
                        self.render_template_item(
                            cx,
                            scope,
                            surface,
                            data_model,
                            &component_id,
                            &item_path,
                        );
                    }
                }
            }
        }
    }

    fn render_template_item(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &super::processor::Surface,
        data_model: &DataModel,
        component_id: &str,
        item_path: &str,
    ) {
        // Set up scoped data model for template items
        // Save previous scope and set new one
        let previous_scope = self.current_scope.take();
        self.current_scope = Some(item_path.to_string());

        // Render the component with scoped path resolution
        self.render_component(cx, scope, surface, data_model, component_id);

        // Restore previous scope
        self.current_scope = previous_scope;
    }

    fn render_text(&mut self, cx: &mut Cx2d, text: &TextComponent, data_model: &DataModel) {
        // Use scoped resolution for template rendering
        let text_value = resolve_string_value_scoped(
            &text.text,
            data_model,
            self.current_scope.as_deref(),
        );



        // Determine font size based on usage hint
        let font_size = match text.usage_hint {
            Some(TextUsageHint::H1) => 28.0,
            Some(TextUsageHint::H2) => 22.0,
            Some(TextUsageHint::H3) => 18.0,
            Some(TextUsageHint::H4) => 16.0,
            Some(TextUsageHint::H5) => 14.0,
            Some(TextUsageHint::Caption) => 12.0,
            Some(TextUsageHint::Code) => 13.0,
            _ => 14.0, // Body default
        };

        // Use different DrawText based on context for correct z-ordering:
        // - Text inside button uses draw_button_text (drawn after draw_button)
        // - Text inside card uses draw_card_text (drawn after draw_card)
        // - Text outside both uses draw_text
        if self.inside_button {
            self.draw_button_text.text_style.font_size = font_size;
            self.draw_button_text.draw_walk(cx, Walk::fit(), Align::default(), &text_value);
        } else if self.inside_card {
            self.draw_card_text.text_style.font_size = font_size;
            self.draw_card_text.draw_walk(cx, Walk::fit(), Align::default(), &text_value);
        } else {
            self.draw_text.text_style.font_size = font_size;
            self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), &text_value);
        }
    }

    fn render_image(&mut self, cx: &mut Cx2d, img: &ImageComponent, data_model: &DataModel) {
        // Use scoped resolution for template rendering
        let url = resolve_string_value_scoped(
            &img.url,
            data_model,
            self.current_scope.as_deref(),
        );

        // Determine size based on usage hint
        let (width, height) = match img.usage_hint {
            Some(ImageUsageHint::Icon) => (24.0, 24.0),
            Some(ImageUsageHint::Avatar) => (48.0, 48.0),
            Some(ImageUsageHint::SmallFeature) => (64.0, 64.0),
            Some(ImageUsageHint::MediumFeature) => (120.0, 80.0),
            Some(ImageUsageHint::LargeFeature) => (200.0, 150.0),
            Some(ImageUsageHint::Header) => (300.0, 100.0),
            _ => (80.0, 80.0), // Default size
        };

        let walk = Walk::new(Size::Fixed(width), Size::Fixed(height));

        // Get texture index (avoid borrow conflict)
        let texture_idx = self.get_texture_index_for_url(&url);

        // Try to render actual image if texture is available
        if let Some(idx) = texture_idx {
            // Get texture reference by index
            let texture = match idx {
                0 => self.texture_headphones.as_ref(),
                1 => self.texture_mouse.as_ref(),
                2 => self.texture_keyboard.as_ref(),
                3 => self.texture_alipay.as_ref(),
                4 => self.texture_wechat.as_ref(),
                _ => None,
            };

            if let Some(tex) = texture {
                // Draw actual image with texture
                self.draw_image.draw_vars.set_texture(0, tex);
                self.draw_image.draw_walk(cx, walk);
                return;
            }
        }

        // Fallback to placeholder
        let layout = Layout {
            padding: Padding {
                left: 4.0,
                right: 4.0,
                top: 4.0,
                bottom: 4.0,
            },
            align: Align { x: 0.5, y: 0.5 },
            ..Layout::default()
        };

        self.draw_image_placeholder.begin(cx, walk, layout);
        self.draw_image_text.draw_walk(cx, Walk::fit(), Align::default(), "IMG");
        self.draw_image_placeholder.end(cx);
    }

    fn render_card(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &super::processor::Surface,
        data_model: &DataModel,
        card: &CardComponent,
    ) {
        // Use the standard Makepad pattern: begin/end with draw_bg
        // The key is that begin() adds background instance, then children are drawn, then end() finalizes
        let walk = Walk {
            margin: Margin { left: 0.0, right: 0.0, top: 8.0, bottom: 8.0 },
            ..Walk::fill_fit()
        };
        let layout = Layout {
            flow: Flow::Down,
            padding: Padding {
                left: 16.0,
                right: 16.0,
                top: 12.0,
                bottom: 12.0,
            },
            ..Layout::default()
        };


        // Begin card - this adds background instance and starts turtle
        self.draw_card.begin(cx, walk, layout);

        // Set flag to use card text (which will be drawn AFTER the card background)
        self.inside_card = true;

        // Render child content
        let child = card.child.clone();
        self.render_component(cx, scope, surface, data_model, &child);

        // Reset flag
        self.inside_card = false;

        // End card
        self.draw_card.end(cx);

    }

    fn render_button(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &super::processor::Surface,
        data_model: &DataModel,
        btn: &ButtonComponent,
        component_id: &str,
    ) {
        // Get button index (this is the button we're about to render)
        let button_idx = self.button_data.len();

        // Get button state (hover/pressed) for this specific button
        let is_hover = self.hovered_button_idx == Some(button_idx);
        let is_pressed = self.pressed_button_idx == Some(button_idx);

        // Set button color based on state
        let base_color = vec4(0.231, 0.51, 0.965, 1.0);     // #3B82F6 - blue
        let hover_color = vec4(0.145, 0.388, 0.922, 1.0);   // #2563EB - darker blue
        let pressed_color = vec4(0.114, 0.306, 0.847, 1.0); // #1D4ED8 - even darker

        let color = if is_pressed {
            pressed_color
        } else if is_hover {
            hover_color
        } else {
            base_color
        };

        // Button layout with padding - this ensures text has proper spacing
        let layout = Layout {
            padding: Padding {
                left: 16.0,
                right: 16.0,
                top: 8.0,
                bottom: 8.0,
            },
            align: Align { x: 0.5, y: 0.5 },
            ..Layout::default()
        };

        // Record starting position before drawing
        let start_pos = cx.turtle().pos();

        // Draw button background with proper padding
        self.draw_button.color = color;
        self.draw_button.begin(cx, Walk::fit(), layout);

        // Set flag to use button text (drawn after button background)
        self.inside_button = true;

        // Render button child (usually Text)
        let child = btn.child.clone();
        self.render_component(cx, scope, surface, data_model, &child);

        // Reset flag
        self.inside_button = false;

        // End button background
        self.draw_button.end(cx);

        // Calculate button rect from start position and current turtle position
        let end_pos = cx.turtle().pos();
        // For Flow::Right, the width is the difference in x, height needs to be calculated
        // Use the used rect from turtle
        let used_rect = cx.turtle().used();
        let button_rect = Rect {
            pos: start_pos,
            size: dvec2(end_pos.x - start_pos.x, used_rect.y),
        };

        // Update or create Area for this button using add_rect_area
        // Reuse existing Area if available to maintain event tracking across frames
        if button_idx < self.button_areas.len() {
            // Update existing area
            cx.add_rect_area(&mut self.button_areas[button_idx], button_rect);
        } else {
            // Create new area
            let mut button_area = Area::Empty;
            cx.add_rect_area(&mut button_area, button_rect);
            self.button_areas.push(button_area);
        }


        // Store button metadata including template scope for action context resolution
        self.button_data.push((
            component_id.to_string(),
            btn.action.clone(),
            self.current_scope.clone(),
        ));
    }

    // ============================================================================
    // TextField Rendering
    // ============================================================================

    fn render_text_field(
        &mut self,
        cx: &mut Cx2d,
        text_field: &TextFieldComponent,
        data_model: &DataModel,
        component_id: &str,
    ) {
        let text_field_idx = self.text_field_data.len();
        let is_focused = self.focused_text_field_idx == Some(text_field_idx);

        // Get current value - use input buffer if focused, otherwise from data model
        let current_value = if is_focused {
            self.text_input_buffer.clone()
        } else {
            resolve_string_value_scoped(&text_field.text, data_model, self.current_scope.as_deref())
        };

        // Get placeholder text
        let placeholder = text_field
            .placeholder
            .as_ref()
            .map(|p| resolve_string_value_scoped(p, data_model, self.current_scope.as_deref()))
            .unwrap_or_default();

        // Get binding path for two-way binding
        let binding_path = text_field.text.as_path().map(|p| {
            if let Some(scope) = &self.current_scope {
                format!("{}/{}", scope, p.trim_start_matches('/'))
            } else {
                p.to_string()
            }
        });

        // Layout
        let walk = Walk {
            width: Size::Fixed(200.0),
            height: Size::Fixed(36.0),
            ..Walk::default()
        };
        let layout = Layout {
            padding: Padding {
                left: 12.0,
                right: 12.0,
                top: 8.0,
                bottom: 8.0,
            },
            align: Align { x: 0.0, y: 0.5 },
            ..Layout::default()
        };

        // Record start position
        let start_pos = cx.turtle().pos();

        // Set focus state
        self.draw_text_field.focus = if is_focused { 1.0 } else { 0.0 };

        // Draw background
        self.draw_text_field.begin(cx, walk, layout);

        // Draw text or placeholder
        if current_value.is_empty() && !is_focused {
            self.draw_text_field_placeholder
                .draw_walk(cx, Walk::fit(), Align::default(), &placeholder);
        } else {
            // Draw text with cursor if focused
            if is_focused {
                // Draw text before cursor
                let (before, after) = current_value.split_at(self.cursor_pos.min(current_value.len()));
                self.draw_text_field_text
                    .draw_walk(cx, Walk::fit(), Align::default(), before);
                // Draw cursor (simple vertical line approximation using |)
                self.draw_text_field_text
                    .draw_walk(cx, Walk::fit(), Align::default(), "|");
                self.draw_text_field_text
                    .draw_walk(cx, Walk::fit(), Align::default(), after);
            } else {
                self.draw_text_field_text
                    .draw_walk(cx, Walk::fit(), Align::default(), &current_value);
            }
        }

        self.draw_text_field.end(cx);

        // Calculate rect for hit testing (using fixed size)
        let rect = Rect {
            pos: start_pos,
            size: dvec2(200.0, 36.0),
        };

        // Update or create area
        if text_field_idx < self.text_field_areas.len() {
            cx.add_rect_area(&mut self.text_field_areas[text_field_idx], rect);
        } else {
            let mut area = Area::Empty;
            cx.add_rect_area(&mut area, rect);
            self.text_field_areas.push(area);
        }

        // Store metadata
        self.text_field_data.push((
            component_id.to_string(),
            binding_path,
            current_value,
        ));
    }

    // ============================================================================
    // CheckBox Rendering
    // ============================================================================

    fn render_checkbox(
        &mut self,
        cx: &mut Cx2d,
        checkbox: &CheckBoxComponent,
        data_model: &DataModel,
        component_id: &str,
    ) {
        let checkbox_idx = self.checkbox_data.len();
        let is_hovered = self.hovered_checkbox_idx == Some(checkbox_idx);

        // Get current checked state
        let is_checked =
            resolve_boolean_value_scoped(&checkbox.value, data_model, self.current_scope.as_deref());

        // Get label text
        let label = checkbox
            .label
            .as_ref()
            .map(|l| resolve_string_value_scoped(l, data_model, self.current_scope.as_deref()))
            .unwrap_or_default();

        // Get binding path
        let binding_path = checkbox.value.as_path().map(|p| {
            if let Some(scope) = &self.current_scope {
                format!("{}/{}", scope, p.trim_start_matches('/'))
            } else {
                p.to_string()
            }
        });

        // Record start position
        let start_pos = cx.turtle().pos();

        // Draw checkbox row
        let row_walk = Walk::fit();
        let row_layout = Layout {
            flow: Flow::right(),
            spacing: 8.0,
            align: Align { x: 0.0, y: 0.5 },
            ..Layout::default()
        };

        cx.begin_turtle(row_walk, row_layout);

        // Draw checkbox box
        let checkbox_walk = Walk {
            width: Size::Fixed(20.0),
            height: Size::Fixed(20.0),
            ..Walk::default()
        };

        self.draw_checkbox.checked = if is_checked { 1.0 } else { 0.0 };
        self.draw_checkbox.hover = if is_hovered { 1.0 } else { 0.0 };
        self.draw_checkbox.draw_walk(cx, checkbox_walk);

        // Draw label
        if !label.is_empty() {
            if self.inside_card {
                self.draw_card_text
                    .draw_walk(cx, Walk::fit(), Align::default(), &label);
            } else {
                self.draw_checkbox_label
                    .draw_walk(cx, Walk::fit(), Align::default(), &label);
            }
        }

        // Get the used rect before ending turtle
        let used = cx.turtle().used();
        cx.end_turtle();

        // Calculate rect for hit testing using the actual used space
        // Ensure minimum clickable area: 200px wide, 28px high
        let rect = Rect {
            pos: start_pos,
            size: dvec2(used.x.max(200.0), used.y.max(28.0)),
        };

        // Update or create area
        if checkbox_idx < self.checkbox_areas.len() {
            cx.add_rect_area(&mut self.checkbox_areas[checkbox_idx], rect);
        } else {
            let mut area = Area::Empty;
            cx.add_rect_area(&mut area, rect);
            self.checkbox_areas.push(area);
        }

        // Store metadata
        self.checkbox_data
            .push((component_id.to_string(), binding_path, is_checked));
    }

    // ============================================================================
    // Slider Rendering
    // ============================================================================

    fn render_slider(
        &mut self,
        cx: &mut Cx2d,
        slider: &SliderComponent,
        data_model: &DataModel,
        component_id: &str,
    ) {
        let slider_idx = self.slider_data.len();
        let _is_hovered = self.hovered_slider_idx == Some(slider_idx);
        let _is_dragging = self.dragging_slider_idx == Some(slider_idx);

        // Get values
        let current_value =
            resolve_number_value_scoped(&slider.value, data_model, self.current_scope.as_deref());
        let min = slider.min.unwrap_or(0.0);
        let max = slider.max.unwrap_or(100.0);

        // Calculate progress (0.0 to 1.0)
        let progress = if max > min {
            ((current_value - min) / (max - min)).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // Get binding path
        let binding_path = slider.value.as_path().map(|p| {
            if let Some(scope) = &self.current_scope {
                format!("{}/{}", scope, p.trim_start_matches('/'))
            } else {
                p.to_string()
            }
        });

        // Record start position
        let start_pos = cx.turtle().pos();

        // Slider dimensions
        let slider_width = 200.0;
        let track_height = 6.0;
        let thumb_size = 18.0;

        // Draw slider container
        let container_walk = Walk {
            width: Size::Fixed(slider_width),
            height: Size::Fixed(thumb_size),
            ..Walk::default()
        };
        let container_layout = Layout {
            align: Align { x: 0.0, y: 0.5 },
            ..Layout::default()
        };

        cx.begin_turtle(container_walk, container_layout);

        // Draw track
        let track_walk = Walk {
            width: Size::Fixed(slider_width),
            height: Size::Fixed(track_height),
            margin: Margin {
                top: (thumb_size - track_height) / 2.0,
                ..Margin::default()
            },
            ..Walk::default()
        };

        self.draw_slider_track.progress = progress as f32;
        self.draw_slider_track.draw_walk(cx, track_walk);

        cx.end_turtle();

        // Draw thumb (overlay at correct position)
        // Note: For proper overlay we'd need absolute positioning
        // For now, we'll use a simpler approach

        // Calculate rect for hit testing (the entire slider area)
        let rect = Rect {
            pos: start_pos,
            size: dvec2(slider_width, thumb_size),
        };

        // Update or create area
        if slider_idx < self.slider_areas.len() {
            cx.add_rect_area(&mut self.slider_areas[slider_idx], rect);
        } else {
            let mut area = Area::Empty;
            cx.add_rect_area(&mut area, rect);
            self.slider_areas.push(area);
        }

        // Store metadata
        self.slider_data.push((
            component_id.to_string(),
            binding_path,
            min,
            max,
            current_value,
        ));
    }

    // ============================================================================
    // List Rendering
    // ============================================================================

    fn render_list(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        surface: &super::processor::Surface,
        data_model: &DataModel,
        list: &ListComponent,
    ) {
        // For now, render List similar to Column
        // TODO: Implement PortalList for virtualized scrolling
        let walk = Walk::fill_fit();
        let layout = Layout {
            flow: Flow::Down,
            spacing: 8.0,
            ..Layout::default()
        };

        cx.begin_turtle(walk, layout);

        // Render children (supports template binding)
        let children = list.children.clone();
        self.render_children(cx, scope, surface, data_model, &children);

        cx.end_turtle();
    }

    // ============================================================================
    // Chart Rendering
    // ============================================================================

    /// Default color palette for charts
    fn chart_palette(index: usize) -> Vec4 {
        const COLORS: &[(f32, f32, f32)] = &[
            (0.231, 0.510, 0.965),  // #3B82F6 blue
            (0.161, 0.714, 0.467),  // #28B677 green
            (0.937, 0.333, 0.314),  // #EF5550 red
            (0.969, 0.643, 0.176),  // #F7A42D orange
            (0.545, 0.361, 0.886),  // #8B5CE2 purple
            (0.071, 0.741, 0.812),  // #12BDD0 teal
            (0.957, 0.486, 0.667),  // #F47CAA pink
            (0.400, 0.553, 0.200),  // #668D33 olive
        ];
        let (r, g, b) = COLORS[index % COLORS.len()];
        Vec4 { x: r, y: g, z: b, w: 1.0 }
    }

    /// Parse a hex color string to Vec4
    fn parse_hex_color(hex: &str) -> Option<Vec4> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 { return None; }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
        Some(Vec4 { x: r, y: g, z: b, w: 1.0 })
    }

    fn get_chart_color(&self, chart: &ChartComponent, index: usize) -> Vec4 {
        if index < chart.colors.len() {
            if let Some(color) = Self::parse_hex_color(&chart.colors[index]) {
                return color;
            }
        }
        Self::chart_palette(index)
    }

    /// Estimate text width in pixels for chart layout
    fn estimate_text_width(text: &str, font_size: f64) -> f64 {
        let avg_char_width = font_size * 0.55;
        text.len() as f64 * avg_char_width
    }

    fn render_chart(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        chart: &ChartComponent,
        data_model: &DataModel,
        component_id: &str,
    ) {
        let current_scope = self.current_scope.clone();
        let cs = current_scope.as_deref();
        match chart.chart_type {
            ChartType::Line => chart_bridge::render_line(&mut self.plot_line, cx, scope, chart, data_model, cs),
            ChartType::Bar => chart_bridge::render_bar(&mut self.plot_bar, cx, scope, chart, data_model, cs),
            ChartType::Scatter => chart_bridge::render_scatter(&mut self.plot_scatter, cx, scope, chart, data_model, cs),
            ChartType::Pie => chart_bridge::render_pie(&mut self.plot_pie, cx, scope, chart, data_model, cs),
            ChartType::Area => chart_bridge::render_area(&mut self.plot_area, cx, scope, chart, data_model, cs),
            ChartType::Radar => chart_bridge::render_radar(&mut self.plot_radar, cx, scope, chart, data_model, cs),
            ChartType::Gauge => chart_bridge::render_gauge(&mut self.plot_gauge, cx, scope, chart, data_model, cs),
            ChartType::Bubble => chart_bridge::render_bubble(&mut self.plot_bubble, cx, scope, chart, data_model, cs),
            ChartType::Candlestick => chart_bridge::render_candlestick(&mut self.plot_candlestick, cx, scope, chart, data_model, cs),
            ChartType::Heatmap => chart_bridge::render_heatmap(&mut self.plot_heatmap, cx, scope, chart, data_model, cs),
            ChartType::Treemap => chart_bridge::render_treemap(&mut self.plot_treemap, cx, scope, chart, data_model, cs),
            ChartType::Sankey => chart_bridge::render_sankey(&mut self.plot_sankey, cx, scope, chart, data_model, cs),
            ChartType::Chord => self.render_chord_chart(cx, chart, data_model),
            // New chart types from makepad-plot
            ChartType::Histogram => chart_bridge::render_histogram(&mut self.plot_histogram, cx, scope, chart, data_model, cs),
            ChartType::BoxPlot => chart_bridge::render_boxplot(&mut self.plot_boxplot, cx, scope, chart, data_model, cs),
            ChartType::Donut => chart_bridge::render_donut(&mut self.plot_donut, cx, scope, chart, data_model, cs),
            ChartType::Stem => chart_bridge::render_stem(&mut self.plot_stem, cx, scope, chart, data_model, cs),
            ChartType::Violin => chart_bridge::render_violin(&mut self.plot_violin, cx, scope, chart, data_model, cs),
            ChartType::Polar => chart_bridge::render_polar(&mut self.plot_polar, cx, scope, chart, data_model, cs),
            ChartType::Contour => chart_bridge::render_contour(&mut self.plot_contour, cx, scope, chart, data_model, cs),
            ChartType::Waterfall => chart_bridge::render_waterfall(&mut self.plot_waterfall, cx, scope, chart, data_model, cs),
            ChartType::Funnel => chart_bridge::render_funnel(&mut self.plot_funnel, cx, scope, chart, data_model, cs),
            ChartType::Step => chart_bridge::render_step(&mut self.plot_step, cx, scope, chart, data_model, cs),
            ChartType::Stackplot => chart_bridge::render_stackplot(&mut self.plot_stackplot, cx, scope, chart, data_model, cs),
            ChartType::Hexbin => chart_bridge::render_hexbin(&mut self.plot_hexbin, cx, scope, chart, data_model, cs),
            ChartType::Streamgraph => chart_bridge::render_streamgraph(&mut self.plot_streamgraph, cx, scope, chart, data_model, cs),
            // 3D chart types - pass component_id for per-chart state tracking
            ChartType::Surface3d => chart_bridge::render_surface3d(&mut self.plot_surface3d, cx, scope, chart, data_model, cs, component_id),
            ChartType::Scatter3d => chart_bridge::render_scatter3d(&mut self.plot_scatter3d, cx, scope, chart, data_model, cs),
            ChartType::Line3d => chart_bridge::render_line3d(&mut self.plot_line3d, cx, scope, chart, data_model, cs),
        }
    }


    // ─── Chord diagram ────────────────────────────────────────────
    // Data convention:
    //   labels: entity names (["A", "B", "C", "D"])
    //   series: flow matrix rows — series[i].values[j] = flow from i to j
    fn render_chord_chart(
        &mut self,
        cx: &mut Cx2d,
        chart: &ChartComponent,
        data_model: &DataModel,
    ) {
        let chart_width = chart.width;
        let chart_height = chart.height;

        let walk = Walk::new(Size::Fixed(chart_width), Size::Fixed(chart_height));
        cx.begin_turtle(walk, Layout::default());
        let origin = cx.turtle().pos();

        let mut title_height = 0.0;
        if let Some(ref title_val) = chart.title {
            let title = resolve_string_value_scoped(title_val, data_model, self.current_scope.as_deref());
            if !title.is_empty() {
                self.draw_chart_text.text_style.font_size = 14.0;
                self.draw_chart_text.draw_walk(cx, Walk {
                    abs_pos: Some(dvec2(origin.x + (chart_width - Self::estimate_text_width(&title, 14.0)) / 2.0, origin.y + 4.0)),
                    ..Walk::fit()
                }, Align::default(), &title);
                title_height = 24.0;
            }
        }

        let n = chart.labels.len().min(chart.series.len());
        if n < 2 { cx.end_turtle(); return; }

        // Compute row sums for arc sizing
        let mut row_sums: Vec<f64> = vec![0.0; n];
        for i in 0..n {
            let vals = &chart.series[i].values;
            for j in 0..n.min(vals.len()) {
                row_sums[i] += vals[j];
            }
        }
        let grand_total: f64 = row_sums.iter().sum();
        if grand_total <= 0.0 { cx.end_turtle(); return; }

        let center_x = origin.x + chart_width / 2.0;
        let center_y = origin.y + title_height + (chart_height - title_height) / 2.0;
        let radius = ((chart_width.min(chart_height - title_height)) / 2.0 - 40.0).max(30.0);
        let inner_frac = 0.85; // inner_radius as fraction of outer
        let gap_angle = 0.04_f64; // radians gap between arcs
        let total_gap = gap_angle * n as f64;
        let available = std::f64::consts::TAU - total_gap;
        let pi = std::f64::consts::PI;

        // Compute arc start/end angles (starting at -PI/2 = 12 o'clock)
        let mut arc_starts: Vec<f64> = vec![0.0; n];
        let mut arc_ends: Vec<f64> = vec![0.0; n];
        let mut angle = -pi / 2.0;
        for i in 0..n {
            arc_starts[i] = angle;
            let span = (row_sums[i] / grand_total) * available;
            arc_ends[i] = angle + span;
            angle += span + gap_angle;
        }

        // Draw outer arcs using DrawA2uiArc (proper GPU arc shader)
        let arc_size = radius * 2.0 + 4.0;
        for i in 0..n {
            self.draw_chart_arc.color = self.get_chart_color(chart, i);
            self.draw_chart_arc.start_angle = arc_starts[i] as f32;
            self.draw_chart_arc.end_angle = arc_ends[i] as f32;
            self.draw_chart_arc.inner_radius = inner_frac as f32;
            self.draw_chart_arc.draw_walk(cx, Walk {
                abs_pos: Some(dvec2(center_x - radius, center_y - radius)),
                width: Size::Fixed(arc_size),
                height: Size::Fixed(arc_size),
                ..Walk::default()
            });
        }

        // Draw chord ribbons using DrawA2uiQuad (proper quadrilateral shader)
        let inner_r = radius * inner_frac;
        let mut dst_angle_cursors: Vec<f64> = arc_starts.clone();
        for i in 0..n {
            let vals = &chart.series[i].values;
            let mut src_angle_cursor = arc_starts[i];
            for j in 0..n.min(vals.len()) {
                let flow = vals[j];
                if flow <= 0.0 || i == j {
                    let span = (flow / grand_total) * available;
                    src_angle_cursor += span;
                    continue;
                }
                let src_span = (flow / grand_total) * available;
                let src_a0 = src_angle_cursor;
                let src_a1 = src_angle_cursor + src_span;
                src_angle_cursor += src_span;

                // Target position on target arc — consume from destination cursor
                let dst_a0 = dst_angle_cursors[j];
                let dst_a1 = dst_a0 + src_span;
                dst_angle_cursors[j] += src_span;

                let color = self.get_chart_color(chart, i);
                let mut fill_color = color;
                fill_color.w = 0.35;

                // Boundary curves: top (src_a0→dst_a0) and bottom (src_a1→dst_a1)
                // Both are quadratic beziers with control point at center
                let s0x = center_x + inner_r * src_a0.cos();
                let s0y = center_y + inner_r * src_a0.sin();
                let s1x = center_x + inner_r * src_a1.cos();
                let s1y = center_y + inner_r * src_a1.sin();
                let d0x = center_x + inner_r * dst_a0.cos();
                let d0y = center_y + inner_r * dst_a0.sin();
                let d1x = center_x + inner_r * dst_a1.cos();
                let d1y = center_y + inner_r * dst_a1.sin();

                // Helper: evaluate quadratic bezier at t
                #[inline]
                fn qbez(t: f64, p0: f64, p1: f64, p2: f64) -> f64 {
                    let u = 1.0 - t;
                    u * u * p0 + 2.0 * u * t * p1 + t * t * p2
                }

                // Render ribbon fill as a strip of proper quadrilaterals
                let t_steps = 64;
                let pad = 2.0;
                for step in 0..t_steps {
                    let t = step as f64 / t_steps as f64;
                    let t_next = (step + 1) as f64 / t_steps as f64;

                    let tx0 = qbez(t, s0x, center_x, d0x);
                    let ty0 = qbez(t, s0y, center_y, d0y);
                    let tx1 = qbez(t_next, s0x, center_x, d0x);
                    let ty1 = qbez(t_next, s0y, center_y, d0y);

                    let bx0 = qbez(t, s1x, center_x, d1x);
                    let by0 = qbez(t, s1y, center_y, d1y);
                    let bx1 = qbez(t_next, s1x, center_x, d1x);
                    let by1 = qbez(t_next, s1y, center_y, d1y);

                    let corners = [(tx0, ty0), (tx1, ty1), (bx1, by1), (bx0, by0)];
                    let min_x = corners.iter().map(|c| c.0).fold(f64::MAX, f64::min) - pad;
                    let max_x = corners.iter().map(|c| c.0).fold(f64::MIN, f64::max) + pad;
                    let min_y = corners.iter().map(|c| c.1).fold(f64::MAX, f64::min) - pad;
                    let max_y = corners.iter().map(|c| c.1).fold(f64::MIN, f64::max) + pad;
                    let w = (max_x - min_x).max(1.0);
                    let h = (max_y - min_y).max(1.0);

                    self.draw_chart_quad.color = fill_color;
                    self.draw_chart_quad.opacity = 0.4;
                    self.draw_chart_quad.p0x = (tx0 - min_x) as f32;
                    self.draw_chart_quad.p0y = (ty0 - min_y) as f32;
                    self.draw_chart_quad.p1x = (tx1 - min_x) as f32;
                    self.draw_chart_quad.p1y = (ty1 - min_y) as f32;
                    self.draw_chart_quad.p2x = (bx1 - min_x) as f32;
                    self.draw_chart_quad.p2y = (by1 - min_y) as f32;
                    self.draw_chart_quad.p3x = (bx0 - min_x) as f32;
                    self.draw_chart_quad.p3y = (by0 - min_y) as f32;
                    self.draw_chart_quad.draw_walk(cx, Walk {
                        abs_pos: Some(dvec2(min_x, min_y)),
                        width: Size::Fixed(w),
                        height: Size::Fixed(h),
                        ..Walk::default()
                    });
                }

                // Draw smooth anti-aliased boundary curves on top
                let border_color = Vec4 { x: color.x, y: color.y, z: color.z, w: 0.6 };
                let edge_steps = 48;
                for step in 0..edge_steps {
                    let t = step as f64 / edge_steps as f64;
                    let t_next = (step + 1) as f64 / edge_steps as f64;

                    // Top boundary line segments
                    let ax0 = qbez(t, s0x, center_x, d0x);
                    let ay0 = qbez(t, s0y, center_y, d0y);
                    let ax1 = qbez(t_next, s0x, center_x, d0x);
                    let ay1 = qbez(t_next, s0y, center_y, d0y);

                    let seg_w = ((ax1 - ax0).abs() + 2.0).max(3.0);
                    let seg_h = ((ay1 - ay0).abs() + 2.0).max(3.0);
                    let seg_x = ax0.min(ax1) - 1.0;
                    let seg_y = ay0.min(ay1) - 1.0;

                    self.draw_chart_line.color = border_color;
                    self.draw_chart_line.line_width = 0.08;
                    self.draw_chart_line.x1 = ((ax0 - seg_x) / seg_w) as f32;
                    self.draw_chart_line.y1 = ((ay0 - seg_y) / seg_h) as f32;
                    self.draw_chart_line.x2 = ((ax1 - seg_x) / seg_w) as f32;
                    self.draw_chart_line.y2 = ((ay1 - seg_y) / seg_h) as f32;
                    self.draw_chart_line.draw_walk(cx, Walk {
                        abs_pos: Some(dvec2(seg_x, seg_y)),
                        width: Size::Fixed(seg_w),
                        height: Size::Fixed(seg_h),
                        ..Walk::default()
                    });

                    // Bottom boundary line segments
                    let bx0l = qbez(t, s1x, center_x, d1x);
                    let by0l = qbez(t, s1y, center_y, d1y);
                    let bx1l = qbez(t_next, s1x, center_x, d1x);
                    let by1l = qbez(t_next, s1y, center_y, d1y);

                    let seg_w2 = ((bx1l - bx0l).abs() + 2.0).max(3.0);
                    let seg_h2 = ((by1l - by0l).abs() + 2.0).max(3.0);
                    let seg_x2 = bx0l.min(bx1l) - 1.0;
                    let seg_y2 = by0l.min(by1l) - 1.0;

                    self.draw_chart_line.x1 = ((bx0l - seg_x2) / seg_w2) as f32;
                    self.draw_chart_line.y1 = ((by0l - seg_y2) / seg_h2) as f32;
                    self.draw_chart_line.x2 = ((bx1l - seg_x2) / seg_w2) as f32;
                    self.draw_chart_line.y2 = ((by1l - seg_y2) / seg_h2) as f32;
                    self.draw_chart_line.draw_walk(cx, Walk {
                        abs_pos: Some(dvec2(seg_x2, seg_y2)),
                        width: Size::Fixed(seg_w2),
                        height: Size::Fixed(seg_h2),
                        ..Walk::default()
                    });
                }
            }
        }

        // Draw entity labels around the outside
        for i in 0..n {
            let mid_angle = (arc_starts[i] + arc_ends[i]) / 2.0;
            let label_r = radius + 18.0;
            let lx = center_x + label_r * mid_angle.cos();
            let ly = center_y + label_r * mid_angle.sin();
            let label = &chart.labels[i];
            self.draw_chart_text.text_style.font_size = 10.0;
            self.draw_chart_text.draw_walk(cx, Walk {
                abs_pos: Some(dvec2(lx - Self::estimate_text_width(label, 10.0) / 2.0, ly - 5.0)),
                ..Walk::fit()
            }, Align::default(), label);
        }

        cx.end_turtle();
    }

    // AudioPlayer Rendering
    // ============================================================================

    fn render_audio_player(
        &mut self,
        cx: &mut Cx2d,
        audio_player: &AudioPlayerComponent,
        data_model: &DataModel,
        component_id: &str,
    ) {
        let audio_player_idx = self.audio_player_data.len();
        let is_hovered = self.hovered_audio_player_idx == Some(audio_player_idx);

        // Resolve URL and title
        let url = resolve_string_value_scoped(
            &audio_player.url,
            data_model,
            self.current_scope.as_deref(),
        );

        // Check if this audio component is currently playing
        let is_playing = self.playing_component_id.as_ref().map(|s| s.as_str()) == Some(component_id);

        let title = audio_player
            .title
            .as_ref()
            .map(|t| resolve_string_value_scoped(t, data_model, self.current_scope.as_deref()))
            .unwrap_or_else(|| "Audio".to_string());

        let artist = audio_player
            .artist
            .as_ref()
            .map(|a| resolve_string_value_scoped(a, data_model, self.current_scope.as_deref()));

        // Check if we're already inside a Card - avoid nested card backgrounds
        let already_in_card = self.inside_card;

        // Only create card background if not already inside a card
        if !already_in_card {
            let walk = Walk {
                width: Size::fill(),
                height: Size::fit(),
                margin: Margin { top: 8.0, bottom: 8.0, left: 0.0, right: 0.0 },
                ..Walk::default()
            };
            let layout = Layout {
                flow: Flow::Down,
                padding: Padding {
                    left: 16.0,
                    right: 16.0,
                    top: 12.0,
                    bottom: 12.0,
                },
                spacing: 8.0,
                ..Layout::default()
            };
            self.draw_card.begin(cx, walk, layout);
            self.inside_card = true;
        } else {
            // When inside a card, create a container for audio player content
            let walk = Walk::fill_fit();
            let layout = Layout {
                flow: Flow::Down,
                spacing: 8.0,
                ..Layout::default()
            };
            cx.begin_turtle(walk, layout);
        }

        // Title row with audio bars
        let title_walk = Walk::fill_fit();
        let title_layout = Layout {
            flow: Flow::right(),
            spacing: 8.0,
            align: Align { x: 0.0, y: 0.5 },
            ..Layout::default()
        };
        cx.begin_turtle(title_walk, title_layout);

        // Music icon (🎵)
        self.draw_card_text.text_style.font_size = 20.0;
        self.draw_card_text.draw_walk(cx, Walk::fit(), Align::default(), "🎵");

        // Title and artist column (flexible width, not fill)
        let info_walk = Walk::fit();
        let info_layout = Layout {
            flow: Flow::Down,
            spacing: 2.0,
            ..Layout::default()
        };
        cx.begin_turtle(info_walk, info_layout);

        // Title
        self.draw_card_text.text_style.font_size = 16.0;
        self.draw_card_text.draw_walk(cx, Walk::fit(), Align::default(), &title);

        // Artist (if present)
        if let Some(artist_name) = &artist {
            self.draw_card_text.text_style.font_size = 12.0;
            self.draw_card_text.color = vec4(0.6, 0.6, 0.6, 1.0);
            self.draw_card_text.draw_walk(cx, Walk::fit(), Align::default(), artist_name);
            self.draw_card_text.color = vec4(1.0, 1.0, 1.0, 1.0); // Reset color
        }

        cx.end_turtle();

        // Audio bars visualization (placed right after title/artist)
        let bars_walk = Walk {
            width: Size::Fixed(50.0),
            height: Size::Fixed(35.0),
            margin: Margin { left: 16.0, ..Margin::default() },
            ..Walk::default()
        };
        self.draw_audio_bars.is_playing = if is_playing { 1.0 } else { 0.0 };
        self.draw_audio_bars.draw_walk(cx, bars_walk);

        // Request next frame for continuous animation when playing
        if is_playing {
            cx.new_next_frame();
        }

        cx.end_turtle();

        // Play button
        let button_walk = Walk::fit();
        let button_layout = Layout {
            padding: Padding {
                left: 20.0,
                right: 20.0,
                top: 10.0,
                bottom: 10.0,
            },
            align: Align { x: 0.5, y: 0.5 },
            ..Layout::default()
        };

        // Button colors - different for play vs stop
        let (base_color, hover_color, button_text) = if is_playing {
            (
                vec4(0.9, 0.3, 0.3, 1.0),    // Red for stop
                vec4(0.8, 0.2, 0.2, 1.0),    // Darker red
                "⏹ Stop"
            )
        } else {
            (
                vec4(0.231, 0.51, 0.965, 1.0),   // Blue for play
                vec4(0.145, 0.388, 0.922, 1.0), // Darker blue
                "▶ Play"
            )
        };
        let color = if is_hovered { hover_color } else { base_color };

        self.draw_button.color = color;
        self.draw_button.begin(cx, button_walk, button_layout);

        // Play/Stop button text
        self.draw_button_text.text_style.font_size = 14.0;
        self.draw_button_text.draw_walk(cx, Walk::fit(), Align::default(), button_text);

        self.draw_button.end(cx);

        // Use draw_button's area directly for hit testing
        let button_area = self.draw_button.area();

        // Update or create Area for this audio player button
        if audio_player_idx < self.audio_player_areas.len() {
            self.audio_player_areas[audio_player_idx] = button_area;
        } else {
            self.audio_player_areas.push(button_area);
        }

        // Only end card if we started it
        if !already_in_card {
            self.inside_card = false;
            self.draw_card.end(cx);
        } else {
            // End the container turtle we created
            cx.end_turtle();
        }

        // Store metadata
        self.audio_player_data.push((
            component_id.to_string(),
            url.clone(),
            title.clone(),
        ));

        // Debug: check area rect
        let rect = button_area.rect(cx);
        log!("[render_audio_player] idx={}, id={}, rect=({:.0},{:.0} {:.0}x{:.0})",
             audio_player_idx, component_id, rect.pos.x, rect.pos.y, rect.size.x, rect.size.y);
    }
}

impl A2uiSurfaceRef {
    /// Process A2UI JSON messages
    pub fn process_json(&self, json: &str) -> Result<Vec<ProcessorEvent>, serde_json::Error> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.process_json(json)
        } else {
            Ok(vec![])
        }
    }

    /// Process a single A2UI message
    pub fn process_message(&self, message: A2uiMessage) -> Vec<ProcessorEvent> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.process_message(message)
        } else {
            vec![]
        }
    }

    /// Check if any user action was triggered
    /// Returns the UserAction if one was triggered
    pub fn user_action(&self, actions: &Actions) -> Option<UserAction> {
        if let Some(inner) = self.borrow() {
            if let Some(action) = actions.find_widget_action(inner.widget_uid()) {
                if let A2uiSurfaceAction::UserAction(user_action) =
                    action.cast::<A2uiSurfaceAction>()
                {
                    return Some(user_action);
                }
            }
        }
        None
    }

    /// Check if a specific action was triggered by name
    /// Returns the context HashMap if the action matches
    pub fn action_by_name(
        &self,
        actions: &Actions,
        action_name: &str,
    ) -> Option<std::collections::HashMap<String, serde_json::Value>> {
        if let Some(user_action) = self.user_action(actions) {
            if user_action.action.name == action_name {
                return Some(user_action.action.context);
            }
        }
        None
    }

    /// Check if an audio play action was triggered
    /// Returns (component_id, url, title) if PlayAudio was triggered
    pub fn play_audio(&self, actions: &Actions) -> Option<(String, String, String)> {
        if let Some(inner) = self.borrow() {
            if let Some(action) = actions.find_widget_action(inner.widget_uid()) {
                if let A2uiSurfaceAction::PlayAudio { component_id, url, title } =
                    action.cast::<A2uiSurfaceAction>()
                {
                    return Some((component_id, url, title));
                }
            }
        }
        None
    }

    /// Set the currently playing audio component ID (for Play/Stop toggle display)
    pub fn set_playing_component(&self, component_id: Option<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_playing_component(component_id);
        }
    }
}

// ============================================================================
// A2UI Text Widget
// ============================================================================

/// A2UI Text component
#[derive(Live, LiveHook, Widget)]
pub struct A2uiText {
    #[redraw]
    #[live]
    draw_text: DrawText,

    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    #[live]
    text: ArcStringMut,

    #[rust]
    area: Area,
}

impl Widget for A2uiText {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        self.draw_text
            .draw_walk(cx, Walk::fit(), Align::default(), self.text.as_ref());
        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

impl A2uiText {
    pub fn set_text(&mut self, text: &str) {
        self.text.as_mut_empty().push_str(text);
    }
}

// ============================================================================
// A2UI Column Widget
// ============================================================================

/// A2UI Column layout component
#[derive(Live, LiveHook, Widget)]
pub struct A2uiColumn {
    #[deref]
    view: View,
}

impl Widget for A2uiColumn {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// ============================================================================
// A2UI Row Widget
// ============================================================================

/// A2UI Row layout component
#[derive(Live, LiveHook, Widget)]
pub struct A2uiRow {
    #[deref]
    view: View,
}

impl Widget for A2uiRow {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// ============================================================================
// A2UI Card Widget
// ============================================================================

/// A2UI Card container component
#[derive(Live, LiveHook, Widget)]
pub struct A2uiCard {
    #[deref]
    view: View,

    #[live]
    draw_bg: DrawQuad,
}

impl Widget for A2uiCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

// ============================================================================
// A2UI Button Widget
// ============================================================================

/// A2UI Button component with action support
#[derive(Live, LiveHook, Widget)]
pub struct A2uiButton {
    #[redraw]
    #[live]
    draw_bg: DrawQuad,

    #[live]
    draw_text: DrawText,

    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    #[live]
    text: ArcStringMut,

    #[animator]
    animator: Animator,

    /// The action definition from A2UI
    #[rust]
    action_def: Option<ActionDefinition>,

    #[rust]
    area: Area,
}

/// Actions emitted by A2uiButton
#[derive(Clone, Debug, DefaultNone)]
pub enum A2uiButtonAction {
    Clicked {
        action_name: String,
        component_id: String,
    },
    None,
}

impl Widget for A2uiButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Hand);
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Default);
                self.animator_play(cx, ids!(hover.off));
            }
            Hit::FingerDown(_) => {
                self.animator_play(cx, ids!(pressed.on));
            }
            Hit::FingerUp(fe) => {
                self.animator_play(cx, ids!(pressed.off));
                if fe.is_over {
                    // Emit action
                    if let Some(action_def) = &self.action_def {
                        cx.widget_action(
                            uid,
                            &scope.path,
                            A2uiButtonAction::Clicked {
                                action_name: action_def.name.clone(),
                                component_id: String::new(), // TODO: get from context
                            },
                        );
                    }
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        self.draw_text
            .draw_walk(cx, Walk::fit(), Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        self.area = self.draw_bg.area();
        DrawStep::done()
    }
}

impl A2uiButton {
    pub fn set_text(&mut self, text: &str) {
        self.text.as_mut_empty().push_str(text);
    }

    pub fn set_action(&mut self, action_def: ActionDefinition) {
        self.action_def = Some(action_def);
    }

    pub fn clicked(&self, actions: &Actions) -> Option<(String, String)> {
        if let Some(action) = actions.find_widget_action(self.widget_uid()) {
            if let A2uiButtonAction::Clicked {
                action_name,
                component_id,
            } = action.cast::<A2uiButtonAction>()
            {
                return Some((action_name, component_id));
            }
        }
        None
    }
}

impl A2uiButtonRef {
    pub fn set_text(&self, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_text(text);
        }
    }

    pub fn clicked(&self, actions: &Actions) -> Option<(String, String)> {
        if let Some(inner) = self.borrow() {
            inner.clicked(actions)
        } else {
            None
        }
    }
}
