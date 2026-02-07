use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub LinePlot = {{LinePlot}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
        math_label: <PlotLabel> {}
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct LinePlot {
    #[deref]
    #[live]
    view: View,

    #[live]
    draw_line: DrawPlotLine,

    #[live]
    draw_point: DrawPlotPoint,

    #[live]
    draw_fill: DrawPlotFill,

    #[live]
    label: PlotLabel,

    #[live]
    math_label: PlotLabel,

    #[rust]
    series: Vec<Series>,

    #[rust]
    fill_regions: Vec<FillRegion>,

    #[rust]
    annotations: Vec<TextAnnotation>,

    #[rust]
    arrow_annotations: Vec<ArrowAnnotation>,

    #[rust]
    plot_area: PlotArea,

    #[rust]
    x_range: (f64, f64),

    #[rust]
    y_range: (f64, f64),

    #[rust]
    title: String,

    #[rust]
    x_label: String,

    #[rust]
    y_label: String,

    #[rust(true)]
    show_grid: bool,

    #[rust(true)]
    show_points: bool,

    #[rust(4.0)]
    point_radius: f64,

    #[rust(2.0)]
    line_width: f64,

    #[rust(50.0)]
    left_margin: f64,

    #[rust(30.0)]
    bottom_margin: f64,

    #[rust(20.0)]
    right_margin: f64,

    #[rust(30.0)]
    top_margin: f64,

    #[rust]
    legend_position: LegendPosition,

    #[rust]
    x_scale: ScaleType,

    #[rust]
    y_scale: ScaleType,

    // Pan/zoom state (disabled by default - enable with set_interactive(true))
    #[rust]
    interactive: bool,

    #[rust]
    is_dragging: bool,

    #[rust]
    drag_start: DVec2,

    #[rust]
    initial_x_range: (f64, f64),

    #[rust]
    initial_y_range: (f64, f64),

    // Reference lines and spans
    #[rust]
    vlines: Vec<VLine>,

    #[rust]
    hlines: Vec<HLine>,

    #[rust]
    vspans: Vec<VSpan>,

    #[rust]
    hspans: Vec<HSpan>,
}

impl Widget for LinePlot {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if !self.interactive {
            return;
        }

        // Handle pan/zoom events
        match event.hits(cx, self.view.area()) {
            Hit::FingerDown(fe) => {
                self.is_dragging = true;
                self.drag_start = fe.abs;
                self.initial_x_range = self.x_range;
                self.initial_y_range = self.y_range;
            }
            Hit::FingerMove(fe) => {
                if self.is_dragging && self.plot_area.width() > 0.0 && self.plot_area.height() > 0.0 {
                    // Calculate the delta in data coordinates
                    let dx_pixels = fe.abs.x - self.drag_start.x;
                    let dy_pixels = fe.abs.y - self.drag_start.y;

                    // Convert pixel delta to data delta
                    let x_range_size = self.initial_x_range.1 - self.initial_x_range.0;
                    let y_range_size = self.initial_y_range.1 - self.initial_y_range.0;

                    let dx_data = -dx_pixels * x_range_size / self.plot_area.width();
                    let dy_data = dy_pixels * y_range_size / self.plot_area.height();

                    // Update ranges (pan)
                    self.x_range = (
                        self.initial_x_range.0 + dx_data,
                        self.initial_x_range.1 + dx_data,
                    );
                    self.y_range = (
                        self.initial_y_range.0 + dy_data,
                        self.initial_y_range.1 + dy_data,
                    );

                    self.redraw(cx);
                }
            }
            Hit::FingerUp(_) => {
                self.is_dragging = false;
            }
            Hit::FingerScroll(fe) => {
                // Zoom with scroll wheel
                let zoom_factor = if fe.scroll.y > 0.0 { 0.9 } else { 1.1 };

                // Get mouse position in plot coordinates
                let mouse_x = fe.abs.x;
                let mouse_y = fe.abs.y;

                // Check if mouse is in plot area
                if mouse_x >= self.plot_area.left && mouse_x <= self.plot_area.right
                    && mouse_y >= self.plot_area.top && mouse_y <= self.plot_area.bottom
                {
                    // Calculate the data point under the mouse
                    let rel_x = (mouse_x - self.plot_area.left) / self.plot_area.width();
                    let rel_y = (self.plot_area.bottom - mouse_y) / self.plot_area.height();

                    let data_x = self.x_range.0 + rel_x * (self.x_range.1 - self.x_range.0);
                    let data_y = self.y_range.0 + rel_y * (self.y_range.1 - self.y_range.0);

                    // Zoom around the mouse position
                    let new_x_range = (self.x_range.1 - self.x_range.0) * zoom_factor;
                    let new_y_range = (self.y_range.1 - self.y_range.0) * zoom_factor;

                    self.x_range = (
                        data_x - rel_x * new_x_range,
                        data_x + (1.0 - rel_x) * new_x_range,
                    );
                    self.y_range = (
                        data_y - rel_y * new_y_range,
                        data_y + (1.0 - rel_y) * new_y_range,
                    );

                    self.redraw(cx);
                }
            }
            Hit::FingerHoverIn(_) => {
                // Change cursor to indicate interactive mode
                cx.set_cursor(MouseCursor::Move);
            }
            Hit::FingerHoverOut(_) => {
                cx.set_cursor(MouseCursor::Default);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            self.update_plot_area(rect);
            self.draw_grid(cx);
            self.draw_axes(cx);
            self.draw_series(cx);
            self.draw_annotations(cx);
            self.draw_labels(cx);
            self.draw_legend(cx);
        }

        DrawStep::done()
    }
}

impl LinePlot {
    /// Add a data series to the plot
    pub fn add_series(&mut self, series: Series) {
        self.series.push(series);
        self.auto_range();
    }

    /// Clear all series
    pub fn clear(&mut self) {
        self.series.clear();
    }

    /// Set plot title
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    /// Set X axis label
    pub fn set_xlabel(&mut self, label: impl Into<String>) {
        self.x_label = label.into();
    }

    /// Set Y axis label
    pub fn set_ylabel(&mut self, label: impl Into<String>) {
        self.y_label = label.into();
    }

    /// Set X range manually
    pub fn set_xlim(&mut self, min: f64, max: f64) {
        self.x_range = (min, max);
    }

    /// Set Y range manually
    pub fn set_ylim(&mut self, min: f64, max: f64) {
        self.y_range = (min, max);
    }

    /// Show or hide data points
    pub fn set_show_points(&mut self, show: bool) {
        self.show_points = show;
    }

    /// Set line width
    pub fn set_line_width(&mut self, width: f64) {
        self.line_width = width;
    }

    /// Add a filled region between y1 and y2 values at each x
    /// Similar to matplotlib's fill_between
    pub fn fill_between(&mut self, x: Vec<f64>, y1: Vec<f64>, y2: Vec<f64>, color: Vec4) {
        self.fill_regions.push(FillRegion { x, y1, y2, color });
        self.auto_range();
    }

    /// Add a filled region between a curve and a constant baseline
    pub fn fill_between_baseline(&mut self, x: Vec<f64>, y: Vec<f64>, baseline: f64, color: Vec4) {
        let y2 = vec![baseline; x.len()];
        self.fill_regions.push(FillRegion { x, y1: y, y2, color });
        self.auto_range();
    }

    /// Add a text annotation at a specific data coordinate
    /// Add a plain text annotation at a specific data coordinate
    pub fn annotate(&mut self, text: impl Into<String>, x: f64, y: f64, color: Vec4, font_size: f64) {
        self.annotations.push(TextAnnotation {
            text: text.into(),
            x,
            y,
            color,
            font_size,
            is_math: false,
        });
    }

    /// Add a LaTeX math annotation at a specific data coordinate
    pub fn annotate_math(&mut self, latex: impl Into<String>, x: f64, y: f64, color: Vec4, font_size: f64) {
        self.annotations.push(TextAnnotation {
            text: latex.into(),
            x,
            y,
            color,
            font_size,
            is_math: true,
        });
    }

    /// Add a vertical line at x position (like matplotlib axvline)
    pub fn axvline(&mut self, x: f64, color: Vec4, line_width: f64, line_style: LineStyle) {
        self.vlines.push(VLine { x, color, line_width, line_style });
    }

    /// Add a horizontal line at y position (like matplotlib axhline)
    pub fn axhline(&mut self, y: f64, color: Vec4, line_width: f64, line_style: LineStyle) {
        self.hlines.push(HLine { y, color, line_width, line_style });
    }

    /// Add a vertical shaded span between x1 and x2 (like matplotlib axvspan)
    pub fn axvspan(&mut self, x1: f64, x2: f64, color: Vec4) {
        self.vspans.push(VSpan { x1, x2, color });
    }

    /// Add a horizontal shaded span between y1 and y2 (like matplotlib axhspan)
    pub fn axhspan(&mut self, y1: f64, y2: f64, color: Vec4) {
        self.hspans.push(HSpan { y1, y2, color });
    }

    /// Add an arrow annotation (like matplotlib annotate with arrow)
    pub fn add_arrow(&mut self, arrow: ArrowAnnotation) {
        self.arrow_annotations.push(arrow);
    }

    /// Add an arrow from text position to a data point
    pub fn annotate_with_arrow(&mut self, text: impl Into<String>, text_x: f64, text_y: f64,
                                point_x: f64, point_y: f64, color: Vec4) {
        // Add text annotation
        self.annotations.push(TextAnnotation {
            text: text.into(),
            x: text_x,
            y: text_y,
            color,
            font_size: 12.0,
            is_math: false,
        });
        // Add arrow from text to point
        self.arrow_annotations.push(ArrowAnnotation {
            start_x: text_x,
            start_y: text_y,
            end_x: point_x,
            end_y: point_y,
            color,
            line_width: 1.5,
            head_size: 8.0,
            text: None,
        });
    }

    /// Clear all reference lines and spans
    pub fn clear_annotations(&mut self) {
        self.annotations.clear();
        self.arrow_annotations.clear();
        self.vlines.clear();
        self.hlines.clear();
        self.vspans.clear();
        self.hspans.clear();
    }

    fn auto_range(&mut self) {
        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;

        for s in &self.series {
            for &x in &s.x {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }
            for &y in &s.y {
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }

        // Include fill_regions in auto_range
        for fr in &self.fill_regions {
            for &x in &fr.x {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }
            for &y in &fr.y1 {
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
            for &y in &fr.y2 {
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }

        // Apply scale-aware padding
        match self.x_scale {
            ScaleType::Log => {
                // For log scale, use multiplicative padding
                if x_min > 0.0 && x_max > 0.0 {
                    self.x_range = (x_min / 1.5, x_max * 1.5);
                } else {
                    self.x_range = (x_min, x_max);
                }
            }
            _ => {
                let x_pad = (x_max - x_min) * 0.05;
                self.x_range = (x_min - x_pad, x_max + x_pad);
            }
        }

        match self.y_scale {
            ScaleType::Log => {
                // For log scale, use multiplicative padding
                if y_min > 0.0 && y_max > 0.0 {
                    self.y_range = (y_min / 1.5, y_max * 1.5);
                } else {
                    self.y_range = (y_min, y_max);
                }
            }
            _ => {
                let y_pad = (y_max - y_min) * 0.05;
                self.y_range = (y_min - y_pad, y_max + y_pad);
            }
        }
    }

    fn update_plot_area(&mut self, rect: Rect) {
        self.plot_area = PlotArea::new(
            rect.pos.x + self.left_margin,
            rect.pos.y + self.top_margin,
            rect.pos.x + rect.size.x - self.right_margin,
            rect.pos.y + rect.size.y - self.bottom_margin,
        );
    }

    fn data_to_pixel(&self, x: f64, y: f64) -> DVec2 {
        // Apply scale transformations
        let tx = self.x_scale.transform(x);
        let ty = self.y_scale.transform(y);
        let tx_min = self.x_scale.transform(self.x_range.0);
        let tx_max = self.x_scale.transform(self.x_range.1);
        let ty_min = self.y_scale.transform(self.y_range.0);
        let ty_max = self.y_scale.transform(self.y_range.1);

        let px = self.plot_area.left + (tx - tx_min) / (tx_max - tx_min) * self.plot_area.width();
        let py = self.plot_area.bottom - (ty - ty_min) / (ty_max - ty_min) * self.plot_area.height();
        dvec2(px, py)
    }

    fn draw_grid(&mut self, cx: &mut Cx2d) {
        if !self.show_grid {
            return;
        }

        self.draw_line.color = vec4(0.9, 0.9, 0.9, 1.0);

        // Horizontal grid lines - use scale-aware tick generation
        let y_ticks = self.y_scale.generate_ticks(self.y_range.0, self.y_range.1, 5);
        for y in &y_ticks {
            let p1 = self.data_to_pixel(self.x_range.0, *y);
            let p2 = self.data_to_pixel(self.x_range.1, *y);
            self.draw_line.draw_line(cx, p1, p2, 0.5);
        }

        // Vertical grid lines - use scale-aware tick generation
        let x_ticks = self.x_scale.generate_ticks(self.x_range.0, self.x_range.1, 5);
        for x in &x_ticks {
            let p1 = self.data_to_pixel(*x, self.y_range.0);
            let p2 = self.data_to_pixel(*x, self.y_range.1);
            self.draw_line.draw_line(cx, p1, p2, 0.5);
        }
    }

    fn draw_axes(&mut self, cx: &mut Cx2d) {
        self.draw_line.color = vec4(0.3, 0.3, 0.3, 1.0);

        // X axis
        let x1 = dvec2(self.plot_area.left, self.plot_area.bottom);
        let x2 = dvec2(self.plot_area.right, self.plot_area.bottom);
        self.draw_line.draw_line(cx, x1, x2, 1.0);

        // Y axis
        let y1 = dvec2(self.plot_area.left, self.plot_area.bottom);
        let y2 = dvec2(self.plot_area.left, self.plot_area.top);
        self.draw_line.draw_line(cx, y1, y2, 1.0);
    }

    fn draw_series(&mut self, cx: &mut Cx2d) {
        // 1. Draw horizontal spans (hspans) - background layer
        for hs in &self.hspans {
            self.draw_fill.color = hs.color;
            let p1 = self.data_to_pixel(self.x_range.0, hs.y1);
            let p2 = self.data_to_pixel(self.x_range.1, hs.y2);
            let top = p1.y.min(p2.y);
            let bottom = p1.y.max(p2.y);
            self.draw_fill.draw_fill_strip(cx, self.plot_area.left, self.plot_area.width(), top, bottom);
        }

        // 2. Draw vertical spans (vspans) - background layer
        for vs in &self.vspans {
            self.draw_fill.color = vs.color;
            let p1 = self.data_to_pixel(vs.x1, self.y_range.0);
            let p2 = self.data_to_pixel(vs.x2, self.y_range.1);
            let left = p1.x.min(p2.x);
            let right = p1.x.max(p2.x);
            self.draw_fill.draw_fill_strip(cx, left, right - left, self.plot_area.top, self.plot_area.bottom);
        }

        // 3. Draw fill regions (fill_between)
        for fr in &self.fill_regions {
            self.draw_fill.color = fr.color;
            if fr.x.len() >= 2 {
                for i in 0..fr.x.len() - 1 {
                    let x1 = fr.x[i];
                    let x2 = fr.x[i + 1];
                    let y1_a = fr.y1[i];
                    let y1_b = fr.y1[i + 1];
                    let y2_a = fr.y2[i];
                    let y2_b = fr.y2[i + 1];

                    // Draw a series of thin vertical strips to approximate the fill
                    let steps = 4;
                    for s in 0..steps {
                        let t1 = s as f64 / steps as f64;
                        let t2 = (s + 1) as f64 / steps as f64;
                        let x_left = x1 + (x2 - x1) * t1;
                        let x_right = x1 + (x2 - x1) * t2;
                        let y1_left = y1_a + (y1_b - y1_a) * t1;
                        let y2_left = y2_a + (y2_b - y2_a) * t1;
                        let y1_right = y1_a + (y1_b - y1_a) * t2;
                        let y2_right = y2_a + (y2_b - y2_a) * t2;

                        let p_tl = self.data_to_pixel(x_left, y1_left);
                        let p_bl = self.data_to_pixel(x_left, y2_left);
                        let p_tr = self.data_to_pixel(x_right, y1_right);
                        let p_br = self.data_to_pixel(x_right, y2_right);

                        let left = p_tl.x.min(p_bl.x);
                        let right = p_tr.x.max(p_br.x);
                        let top = p_tl.y.min(p_tr.y).min(p_bl.y).min(p_br.y);
                        let bottom = p_tl.y.max(p_tr.y).max(p_bl.y).max(p_br.y);

                        self.draw_fill.draw_fill_strip(cx, left, right - left, top, bottom);
                    }
                }
            }
        }

        // 4. Draw horizontal reference lines (hlines)
        for hl in &self.hlines {
            self.draw_line.color = hl.color;
            let p = self.data_to_pixel(self.x_range.0, hl.y);
            self.draw_line.draw_line_styled(cx,
                dvec2(self.plot_area.left, p.y),
                dvec2(self.plot_area.right, p.y),
                hl.line_width, hl.line_style, 0.0);
        }

        // 5. Draw vertical reference lines (vlines)
        for vl in &self.vlines {
            self.draw_line.color = vl.color;
            let p = self.data_to_pixel(vl.x, self.y_range.0);
            self.draw_line.draw_line_styled(cx,
                dvec2(p.x, self.plot_area.top),
                dvec2(p.x, self.plot_area.bottom),
                vl.line_width, vl.line_style, 0.0);
        }

        // 6. Draw data series
        for (idx, series) in self.series.iter().enumerate() {
            let color = series.color.unwrap_or_else(|| get_color(idx));
            let line_width = series.line_width.unwrap_or(self.line_width);
            let marker_size = series.marker_size.unwrap_or(self.point_radius);

            self.draw_line.color = color;
            self.draw_point.color = color;

            // Draw error bars first (behind the line)
            if series.yerr_minus.is_some() || series.yerr_plus.is_some() || series.xerr_minus.is_some() || series.xerr_plus.is_some() {
                let cap_width = 4.0;
                for i in 0..series.x.len() {
                    let x = series.x[i];
                    let y = series.y[i];

                    // Y error bars
                    if let (Some(ref err_minus), Some(ref err_plus)) = (&series.yerr_minus, &series.yerr_plus) {
                        if i < err_minus.len() && i < err_plus.len() {
                            let y_low = y - err_minus[i];
                            let y_high = y + err_plus[i];
                            let p_low = self.data_to_pixel(x, y_low);
                            let p_high = self.data_to_pixel(x, y_high);

                            // Vertical line
                            self.draw_line.draw_line_styled(cx, p_low, p_high, 1.0, LineStyle::Solid, 0.0);
                            // Bottom cap
                            self.draw_line.draw_line_styled(cx,
                                dvec2(p_low.x - cap_width, p_low.y),
                                dvec2(p_low.x + cap_width, p_low.y),
                                1.0, LineStyle::Solid, 0.0);
                            // Top cap
                            self.draw_line.draw_line_styled(cx,
                                dvec2(p_high.x - cap_width, p_high.y),
                                dvec2(p_high.x + cap_width, p_high.y),
                                1.0, LineStyle::Solid, 0.0);
                        }
                    }

                    // X error bars
                    if let (Some(ref err_minus), Some(ref err_plus)) = (&series.xerr_minus, &series.xerr_plus) {
                        if i < err_minus.len() && i < err_plus.len() {
                            let x_low = x - err_minus[i];
                            let x_high = x + err_plus[i];
                            let p_low = self.data_to_pixel(x_low, y);
                            let p_high = self.data_to_pixel(x_high, y);

                            // Horizontal line
                            self.draw_line.draw_line_styled(cx, p_low, p_high, 1.0, LineStyle::Solid, 0.0);
                            // Left cap
                            self.draw_line.draw_line_styled(cx,
                                dvec2(p_low.x, p_low.y - cap_width),
                                dvec2(p_low.x, p_low.y + cap_width),
                                1.0, LineStyle::Solid, 0.0);
                            // Right cap
                            self.draw_line.draw_line_styled(cx,
                                dvec2(p_high.x, p_high.y - cap_width),
                                dvec2(p_high.x, p_high.y + cap_width),
                                1.0, LineStyle::Solid, 0.0);
                        }
                    }
                }
            }

            // Draw lines with proper style
            if series.x.len() >= 2 {
                let mut dash_offset = 0.0;

                for i in 0..series.x.len() - 1 {
                    let (x1, y1, x2, y2) = match series.step_style {
                        StepStyle::None => {
                            // Normal line
                            (series.x[i], series.y[i], series.x[i + 1], series.y[i + 1])
                        }
                        StepStyle::Pre => {
                            // Step before: vertical then horizontal
                            // First draw vertical segment
                            let p1 = self.data_to_pixel(series.x[i], series.y[i]);
                            let p2 = self.data_to_pixel(series.x[i], series.y[i + 1]);
                            self.draw_line.draw_line_styled(cx, p1, p2, line_width, series.line_style, dash_offset);
                            let seg_len = ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt();
                            dash_offset += seg_len;
                            // Then horizontal
                            (series.x[i], series.y[i + 1], series.x[i + 1], series.y[i + 1])
                        }
                        StepStyle::Post => {
                            // Step after: horizontal then vertical
                            // First draw horizontal segment
                            let p1 = self.data_to_pixel(series.x[i], series.y[i]);
                            let p2 = self.data_to_pixel(series.x[i + 1], series.y[i]);
                            self.draw_line.draw_line_styled(cx, p1, p2, line_width, series.line_style, dash_offset);
                            let seg_len = ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt();
                            dash_offset += seg_len;
                            // Then vertical
                            (series.x[i + 1], series.y[i], series.x[i + 1], series.y[i + 1])
                        }
                        StepStyle::Mid => {
                            // Step in middle: half horizontal, vertical, half horizontal
                            let mid_x = (series.x[i] + series.x[i + 1]) / 2.0;
                            // First half horizontal
                            let p1 = self.data_to_pixel(series.x[i], series.y[i]);
                            let p2 = self.data_to_pixel(mid_x, series.y[i]);
                            self.draw_line.draw_line_styled(cx, p1, p2, line_width, series.line_style, dash_offset);
                            dash_offset += ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt();
                            // Vertical
                            let p3 = self.data_to_pixel(mid_x, series.y[i + 1]);
                            self.draw_line.draw_line_styled(cx, p2, p3, line_width, series.line_style, dash_offset);
                            dash_offset += ((p3.x - p2.x).powi(2) + (p3.y - p2.y).powi(2)).sqrt();
                            // Second half horizontal
                            (mid_x, series.y[i + 1], series.x[i + 1], series.y[i + 1])
                        }
                    };

                    let p1 = self.data_to_pixel(x1, y1);
                    let p2 = self.data_to_pixel(x2, y2);
                    self.draw_line.draw_line_styled(cx, p1, p2, line_width, series.line_style, dash_offset);

                    // Update dash offset for continuous pattern
                    let seg_len = ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt();
                    dash_offset += seg_len;
                }
            }

            // Draw markers
            let should_draw_markers = series.marker_style != MarkerStyle::None ||
                (self.show_points && series.marker_style == MarkerStyle::None);

            if should_draw_markers {
                let marker = if series.marker_style != MarkerStyle::None {
                    series.marker_style
                } else {
                    MarkerStyle::Circle
                };

                for i in 0..series.x.len() {
                    let p = self.data_to_pixel(series.x[i], series.y[i]);
                    self.draw_point.draw_marker(cx, p, marker_size, marker);
                }
            }
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));

        // X axis tick labels - use scale-aware tick generation and formatting
        let x_ticks = self.x_scale.generate_ticks(self.x_range.0, self.x_range.1, 5);
        for x in &x_ticks {
            let p = self.data_to_pixel(*x, self.y_range.0);
            let label = self.x_scale.format_tick(*x);
            self.label.draw_at(cx, dvec2(p.x, p.y + 5.0), &label, TextAnchor::TopCenter);
        }

        // Y axis tick labels - use scale-aware tick generation and formatting
        let y_ticks = self.y_scale.generate_ticks(self.y_range.0, self.y_range.1, 5);
        for y in &y_ticks {
            let p = self.data_to_pixel(self.x_range.0, *y);
            let label = self.y_scale.format_tick(*y);
            self.label.draw_at(cx, dvec2(p.x - 5.0, p.y), &label, TextAnchor::MiddleRight);
        }

        // Title
        if !self.title.is_empty() {
            let center_x = (self.plot_area.left + self.plot_area.right) / 2.0;
            self.label.draw_at(cx, dvec2(center_x, self.plot_area.top - 10.0), &self.title, TextAnchor::BottomCenter);
        }
    }

    /// Set legend position
    pub fn set_legend(&mut self, position: LegendPosition) {
        self.legend_position = position;
    }

    /// Set X axis scale type
    pub fn set_x_scale(&mut self, scale: ScaleType) {
        self.x_scale = scale;
        // Recalculate range with scale-aware padding
        if !self.series.is_empty() {
            self.auto_range();
        }
    }

    /// Set Y axis scale type
    pub fn set_y_scale(&mut self, scale: ScaleType) {
        self.y_scale = scale;
        // Recalculate range with scale-aware padding
        if !self.series.is_empty() {
            self.auto_range();
        }
    }

    /// Enable or disable pan/zoom interactivity
    pub fn set_interactive(&mut self, interactive: bool) {
        self.interactive = interactive;
    }

    /// Reset view to auto-fit all data
    pub fn reset_view(&mut self) {
        self.auto_range();
    }

    fn draw_annotations(&mut self, cx: &mut Cx2d) {
        // Draw arrow annotations first (so text appears on top)
        let arrows = self.arrow_annotations.clone();
        for arrow in &arrows {
            let start = self.data_to_pixel(arrow.start_x, arrow.start_y);
            let end = self.data_to_pixel(arrow.end_x, arrow.end_y);

            // Draw arrow line
            self.draw_line.color = arrow.color;
            self.draw_line.draw_line(cx, start, end, arrow.line_width);

            // Draw arrowhead at the end point
            let dx = end.x - start.x;
            let dy = end.y - start.y;
            let len = (dx * dx + dy * dy).sqrt();
            if len > 0.0 {
                let ux = dx / len;
                let uy = dy / len;
                let head_size = arrow.head_size;

                // Arrowhead points
                let head_base = dvec2(end.x - ux * head_size, end.y - uy * head_size);
                let perp_x = -uy * head_size * 0.5;
                let perp_y = ux * head_size * 0.5;

                let left = dvec2(head_base.x + perp_x, head_base.y + perp_y);
                let right = dvec2(head_base.x - perp_x, head_base.y - perp_y);

                // Draw arrowhead as two lines forming a V
                self.draw_line.draw_line(cx, end, left, arrow.line_width);
                self.draw_line.draw_line(cx, end, right, arrow.line_width);
            }

            // Draw optional text near the start
            if let Some(ref text) = arrow.text {
                self.label.set_color(arrow.color);
                self.label.set_font_size(11.0);
                self.label.draw_at(cx, start, text, TextAnchor::BottomCenter);
            }
        }

        // Draw text annotations
        let annotations = self.annotations.clone();
        for ann in &annotations {
            let p = self.data_to_pixel(ann.x, ann.y);
            if ann.is_math {
                // Render math annotations as plain text (LaTeX not available without math_widget)
                self.math_label.set_color(ann.color);
                self.math_label.set_font_size(ann.font_size);
                self.math_label.draw_at(cx, p, &ann.text, TextAnchor::Center);
            } else {
                // Use plain text label
                self.label.set_color(ann.color);
                self.label.set_font_size(ann.font_size);
                self.label.draw_at(cx, p, &ann.text, TextAnchor::Center);
            }
        }
    }

    fn draw_legend(&mut self, cx: &mut Cx2d) {
        if self.legend_position == LegendPosition::None || self.series.is_empty() {
            return;
        }

        // Calculate legend dimensions
        let padding = 8.0;
        let line_height = 16.0;
        let marker_size = 10.0;
        let marker_text_gap = 6.0;
        let legend_height = self.series.len() as f64 * line_height + padding * 2.0;
        let legend_width = 100.0; // Fixed width for simplicity

        // Position legend based on setting
        let (legend_x, legend_y) = match self.legend_position {
            LegendPosition::TopRight => (
                self.plot_area.right - legend_width - 10.0,
                self.plot_area.top + 10.0,
            ),
            LegendPosition::TopLeft => (
                self.plot_area.left + 10.0,
                self.plot_area.top + 10.0,
            ),
            LegendPosition::BottomRight => (
                self.plot_area.right - legend_width - 10.0,
                self.plot_area.bottom - legend_height - 10.0,
            ),
            LegendPosition::BottomLeft => (
                self.plot_area.left + 10.0,
                self.plot_area.bottom - legend_height - 10.0,
            ),
            LegendPosition::None => return,
        };

        // Draw legend background
        self.draw_line.color = vec4(0.95, 0.95, 0.95, 0.9);
        let bg_rect = Rect {
            pos: dvec2(legend_x, legend_y),
            size: dvec2(legend_width, legend_height),
        };
        self.draw_line.draw_abs(cx, bg_rect);

        // Draw legend border
        self.draw_line.color = vec4(0.8, 0.8, 0.8, 1.0);
        // Top border
        self.draw_line.draw_line(cx, dvec2(legend_x, legend_y), dvec2(legend_x + legend_width, legend_y), 1.0);
        // Bottom border
        self.draw_line.draw_line(cx, dvec2(legend_x, legend_y + legend_height), dvec2(legend_x + legend_width, legend_y + legend_height), 1.0);
        // Left border
        self.draw_line.draw_line(cx, dvec2(legend_x, legend_y), dvec2(legend_x, legend_y + legend_height), 1.0);
        // Right border
        self.draw_line.draw_line(cx, dvec2(legend_x + legend_width, legend_y), dvec2(legend_x + legend_width, legend_y + legend_height), 1.0);

        // Draw legend entries
        for (idx, series) in self.series.iter().enumerate() {
            let color = series.color.unwrap_or_else(|| get_color(idx));
            let entry_y = legend_y + padding + idx as f64 * line_height + line_height / 2.0;

            // Draw color marker (small rectangle)
            self.draw_point.color = color;
            self.draw_point.draw_point(cx, dvec2(legend_x + padding + marker_size / 2.0, entry_y), marker_size / 2.0);

            // Draw label
            self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
            self.label.draw_at(
                cx,
                dvec2(legend_x + padding + marker_size + marker_text_gap, entry_y),
                &series.label,
                TextAnchor::MiddleLeft,
            );
        }
    }
}

impl LinePlotRef {
    pub fn add_series(&self, series: Series) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_series(series);
        }
    }

    pub fn clear(&self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear();
        }
    }

    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_title(title);
        }
    }

    pub fn set_xlabel(&self, label: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_xlabel(label);
        }
    }

    pub fn set_ylabel(&self, label: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_ylabel(label);
        }
    }

    pub fn set_legend(&self, position: LegendPosition) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_legend(position);
        }
    }

    pub fn set_x_scale(&self, scale: ScaleType) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_x_scale(scale);
        }
    }

    pub fn set_y_scale(&self, scale: ScaleType) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_y_scale(scale);
        }
    }

    pub fn set_interactive(&self, interactive: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_interactive(interactive);
        }
    }

    pub fn reset_view(&self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.reset_view();
        }
    }

    pub fn set_show_points(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_show_points(show);
        }
    }

    pub fn set_line_width(&self, width: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_line_width(width);
        }
    }

    pub fn set_xlim(&self, min: f64, max: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_xlim(min, max);
        }
    }

    pub fn set_ylim(&self, min: f64, max: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_ylim(min, max);
        }
    }

    pub fn fill_between(&self, x: Vec<f64>, y1: Vec<f64>, y2: Vec<f64>, color: Vec4) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.fill_between(x, y1, y2, color);
        }
    }

    pub fn fill_between_baseline(&self, x: Vec<f64>, y: Vec<f64>, baseline: f64, color: Vec4) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.fill_between_baseline(x, y, baseline, color);
        }
    }

    pub fn annotate(&self, text: impl Into<String>, x: f64, y: f64, color: Vec4, font_size: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.annotate(text, x, y, color, font_size);
        }
    }

    /// Add a LaTeX math annotation
    pub fn annotate_math(&self, latex: impl Into<String>, x: f64, y: f64, color: Vec4, font_size: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.annotate_math(latex, x, y, color, font_size);
        }
    }

    /// Add a vertical line at x position
    pub fn axvline(&self, x: f64, color: Vec4, line_width: f64, line_style: LineStyle) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.axvline(x, color, line_width, line_style);
        }
    }

    /// Add a horizontal line at y position
    pub fn axhline(&self, y: f64, color: Vec4, line_width: f64, line_style: LineStyle) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.axhline(y, color, line_width, line_style);
        }
    }

    /// Add a vertical shaded span
    pub fn axvspan(&self, x1: f64, x2: f64, color: Vec4) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.axvspan(x1, x2, color);
        }
    }

    /// Add a horizontal shaded span
    pub fn axhspan(&self, y1: f64, y2: f64, color: Vec4) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.axhspan(y1, y2, color);
        }
    }

    /// Add an arrow annotation
    pub fn add_arrow(&self, arrow: ArrowAnnotation) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_arrow(arrow);
        }
    }

    /// Add an arrow from text position to a data point
    pub fn annotate_with_arrow(&self, text: impl Into<String>, text_x: f64, text_y: f64,
                                point_x: f64, point_y: f64, color: Vec4) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.annotate_with_arrow(text, text_x, text_y, point_x, point_y, color);
        }
    }

    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.redraw(cx);
        }
    }
}
