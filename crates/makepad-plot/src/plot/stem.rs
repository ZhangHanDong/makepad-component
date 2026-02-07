use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub StemPlot = {{StemPlot}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    pub ViolinPlot = {{ViolinPlot}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct StemPlot {
    #[deref]
    #[live]
    view: View,

    #[live]
    draw_line: DrawPlotLine,

    #[live]
    draw_point: DrawPlotPoint,

    #[live]
    label: PlotLabel,

    #[rust]
    series: Vec<Series>,

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

    #[rust(0.0)]
    baseline: f64,

    #[rust(true)]
    show_grid: bool,

    #[rust(6.0)]
    marker_size: f64,

    #[rust(1.5)]
    stem_width: f64,

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
}

impl Widget for StemPlot {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.series.is_empty() {
            self.update_plot_area(rect);
            self.draw_grid(cx);
            self.draw_axes(cx);
            self.draw_stems(cx);
            self.draw_labels(cx);
            self.draw_legend(cx);
        }

        DrawStep::done()
    }
}

impl StemPlot {
    pub fn add_series(&mut self, series: Series) {
        self.series.push(series);
        self.auto_range();
    }

    pub fn clear(&mut self) {
        self.series.clear();
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_xlabel(&mut self, label: impl Into<String>) {
        self.x_label = label.into();
    }

    pub fn set_ylabel(&mut self, label: impl Into<String>) {
        self.y_label = label.into();
    }

    pub fn set_xlim(&mut self, min: f64, max: f64) {
        self.x_range = (min, max);
    }

    pub fn set_ylim(&mut self, min: f64, max: f64) {
        self.y_range = (min, max);
    }

    pub fn set_baseline(&mut self, baseline: f64) {
        self.baseline = baseline;
    }

    pub fn set_marker_size(&mut self, size: f64) {
        self.marker_size = size;
    }

    pub fn set_stem_width(&mut self, width: f64) {
        self.stem_width = width;
    }

    pub fn set_legend(&mut self, position: LegendPosition) {
        self.legend_position = position;
    }

    fn auto_range(&mut self) {
        if self.series.is_empty() {
            return;
        }

        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_min = self.baseline;
        let mut y_max = self.baseline;

        for series in &self.series {
            for &x in &series.x {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }
            for &y in &series.y {
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }

        // Add padding
        let x_pad = (x_max - x_min) * 0.05;
        let y_pad = (y_max - y_min) * 0.1;

        self.x_range = (x_min - x_pad, x_max + x_pad);
        self.y_range = (y_min - y_pad, y_max + y_pad);
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
        let x_norm = (x - self.x_range.0) / (self.x_range.1 - self.x_range.0);
        let y_norm = (y - self.y_range.0) / (self.y_range.1 - self.y_range.0);

        dvec2(
            self.plot_area.left + x_norm * self.plot_area.width(),
            self.plot_area.bottom - y_norm * self.plot_area.height(),
        )
    }

    fn draw_grid(&mut self, cx: &mut Cx2d) {
        if !self.show_grid {
            return;
        }

        self.draw_line.color = vec4(0.9, 0.9, 0.9, 1.0);

        // Horizontal grid lines
        let y_step = (self.y_range.1 - self.y_range.0) / 5.0;
        for i in 0..=5 {
            let y = self.y_range.0 + i as f64 * y_step;
            let p1 = self.data_to_pixel(self.x_range.0, y);
            let p2 = self.data_to_pixel(self.x_range.1, y);
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

        // Baseline (if different from y_range.0)
        if self.baseline > self.y_range.0 && self.baseline < self.y_range.1 {
            self.draw_line.color = vec4(0.5, 0.5, 0.5, 0.5);
            let p1 = self.data_to_pixel(self.x_range.0, self.baseline);
            let p2 = self.data_to_pixel(self.x_range.1, self.baseline);
            self.draw_line.draw_line_styled(cx, p1, p2, 1.0, LineStyle::Dashed, 0.0);
        }
    }

    fn draw_stems(&mut self, cx: &mut Cx2d) {
        for (idx, series) in self.series.iter().enumerate() {
            let color = series.color.unwrap_or_else(|| get_color(idx));
            let marker_style = if series.marker_style != MarkerStyle::None {
                series.marker_style
            } else {
                MarkerStyle::Circle
            };
            let marker_size = series.marker_size.unwrap_or(self.marker_size);
            let stem_width = series.line_width.unwrap_or(self.stem_width);

            self.draw_line.color = color;
            self.draw_point.color = color;

            for i in 0..series.x.len() {
                let x = series.x[i];
                let y = series.y[i];

                // Draw stem (vertical line from baseline to point)
                let p_base = self.data_to_pixel(x, self.baseline);
                let p_top = self.data_to_pixel(x, y);
                self.draw_line.draw_line_styled(cx, p_base, p_top, stem_width, series.line_style, 0.0);

                // Draw marker at top
                self.draw_point.draw_marker(cx, p_top, marker_size, marker_style);
            }
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));

        // X axis tick labels
        let x_step = (self.x_range.1 - self.x_range.0) / 5.0;
        for i in 0..=5 {
            let x = self.x_range.0 + i as f64 * x_step;
            let p = self.data_to_pixel(x, self.y_range.0);
            let label = format!("{:.1}", x);
            self.label.draw_at(cx, dvec2(p.x, p.y + 5.0), &label, TextAnchor::TopCenter);
        }

        // Y axis tick labels
        let y_step = (self.y_range.1 - self.y_range.0) / 5.0;
        for i in 0..=5 {
            let y = self.y_range.0 + i as f64 * y_step;
            let p = self.data_to_pixel(self.x_range.0, y);
            let label = format!("{:.1}", y);
            self.label.draw_at(cx, dvec2(p.x - 5.0, p.y), &label, TextAnchor::MiddleRight);
        }

        // Title
        if !self.title.is_empty() {
            let center_x = (self.plot_area.left + self.plot_area.right) / 2.0;
            self.label.draw_at(cx, dvec2(center_x, self.plot_area.top - 10.0), &self.title, TextAnchor::BottomCenter);
        }
    }

    fn draw_legend(&mut self, cx: &mut Cx2d) {
        if self.legend_position == LegendPosition::None || self.series.len() <= 1 {
            return;
        }

        // Calculate legend position
        let (legend_x, legend_y) = match self.legend_position {
            LegendPosition::TopRight => (self.plot_area.right - 100.0, self.plot_area.top + 10.0),
            LegendPosition::TopLeft => (self.plot_area.left + 10.0, self.plot_area.top + 10.0),
            LegendPosition::BottomRight => (self.plot_area.right - 100.0, self.plot_area.bottom - 50.0),
            LegendPosition::BottomLeft => (self.plot_area.left + 10.0, self.plot_area.bottom - 50.0),
            LegendPosition::None => return,
        };

        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));

        for (idx, series) in self.series.iter().enumerate() {
            if series.label.is_empty() {
                continue;
            }

            let y_offset = legend_y + idx as f64 * 18.0;
            let color = series.color.unwrap_or_else(|| get_color(idx));

            // Draw marker
            self.draw_point.color = color;
            self.draw_point.draw_marker(cx, dvec2(legend_x + 8.0, y_offset), 4.0, MarkerStyle::Circle);

            // Draw label
            self.label.draw_at(cx, dvec2(legend_x + 20.0, y_offset), &series.label, TextAnchor::MiddleLeft);
        }
    }
}

impl StemPlotRef {
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

    pub fn set_baseline(&self, baseline: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_baseline(baseline);
        }
    }

    pub fn set_marker_size(&self, size: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_marker_size(size);
        }
    }

    pub fn set_legend(&self, position: LegendPosition) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_legend(position);
        }
    }

    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.redraw(cx);
        }
    }
}


#[derive(Clone, Debug)]
pub struct ViolinItem {
    pub label: String,
    pub values: Vec<f64>,
    pub color: Option<Vec4>,
}

impl ViolinItem {
    pub fn new(label: impl Into<String>, values: Vec<f64>) -> Self {
        Self { label: label.into(), values, color: None }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ViolinPlot {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] draw_line: DrawPlotLine,
    #[live] draw_point: DrawPlotPoint,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] items: Vec<ViolinItem>,
    #[rust] show_box: bool,
    #[rust] show_median: bool,
    #[rust] bandwidth: f64,
    #[rust] plot_area: PlotArea,
    #[live(40.0)] left_margin: f64,
    #[live(30.0)] right_margin: f64,
    #[live(30.0)] top_margin: f64,
    #[live(50.0)] bottom_margin: f64,
}

impl Widget for ViolinPlot {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.items.is_empty() {
            self.update_plot_area(rect);
            self.draw_violins(cx);
            self.draw_labels(cx);
        }
        DrawStep::done()
    }
}

impl ViolinPlot {
    pub fn set_title(&mut self, title: impl Into<String>) { self.title = title.into(); }
    pub fn add_item(&mut self, item: ViolinItem) { self.items.push(item); }
    pub fn add_from_values(&mut self, label: impl Into<String>, values: &[f64]) {
        self.items.push(ViolinItem::new(label, values.to_vec()));
    }
    pub fn set_show_box(&mut self, show: bool) { self.show_box = show; }
    pub fn set_show_median(&mut self, show: bool) { self.show_median = show; }
    pub fn clear(&mut self) { self.items.clear(); }

    fn update_plot_area(&mut self, rect: Rect) {
        self.plot_area = PlotArea::new(rect.pos.x + self.left_margin, rect.pos.y + self.top_margin,
            rect.pos.x + rect.size.x - self.right_margin, rect.pos.y + rect.size.y - self.bottom_margin);
    }

    fn get_value_range(&self) -> (f64, f64) {
        let mut min = f64::MAX; let mut max = f64::MIN;
        for item in &self.items { for &v in &item.values { min = min.min(v); max = max.max(v); } }
        let padding = (max - min) * 0.1;
        (min - padding, max + padding)
    }

    fn compute_kde(&self, values: &[f64], bw: f64, y_min: f64, y_max: f64, n: usize) -> Vec<(f64, f64)> {
        let step = (y_max - y_min) / (n - 1) as f64;
        (0..n).map(|i| {
            let y = y_min + i as f64 * step;
            let density: f64 = values.iter().map(|&v| (-(y - v).powi(2) / (2.0 * bw * bw)).exp()).sum();
            (y, density / (values.len() as f64 * bw * (2.0 * std::f64::consts::PI).sqrt()))
        }).collect()
    }

    fn draw_violins(&mut self, cx: &mut Cx2d) {
        let n = self.items.len();
        if n == 0 { return; }
        let (y_min, y_max) = self.get_value_range();
        let band_w = self.plot_area.width() / n as f64;
        let all: Vec<f64> = self.items.iter().flat_map(|i| i.values.iter().cloned()).collect();
        let mean = all.iter().sum::<f64>() / all.len() as f64;
        let std = (all.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / all.len() as f64).sqrt();
        let bw = if self.bandwidth > 0.0 { self.bandwidth } else { 1.06 * std * (all.len() as f64).powf(-0.2) };

        self.draw_line.color = vec4(0.3, 0.3, 0.3, 1.0);
        self.draw_line.draw_line(cx, dvec2(self.plot_area.left, self.plot_area.bottom), dvec2(self.plot_area.right, self.plot_area.bottom), 1.0);
        self.draw_line.draw_line(cx, dvec2(self.plot_area.left, self.plot_area.bottom), dvec2(self.plot_area.left, self.plot_area.top), 1.0);

        for (i, item) in self.items.iter().enumerate() {
            if item.values.is_empty() { continue; }
            let x_c = self.plot_area.left + (i as f64 + 0.5) * band_w;
            let max_w = band_w * 0.4;
            let kde = self.compute_kde(&item.values, bw, y_min, y_max, 50);
            let max_d = kde.iter().map(|(_, d)| *d).fold(0.0f64, f64::max);
            if max_d <= 0.0 { continue; }
            let color = item.color.unwrap_or_else(|| get_color(i));
            self.draw_fill.color = vec4(color.x, color.y, color.z, 0.6);

            for j in 0..kde.len() - 1 {
                let (y1, d1) = kde[j]; let (y2, d2) = kde[j + 1];
                let py1 = self.plot_area.bottom - (y1 - y_min) / (y_max - y_min) * self.plot_area.height();
                let py2 = self.plot_area.bottom - (y2 - y_min) / (y_max - y_min) * self.plot_area.height();
                let w = ((d1 + d2) / 2.0) / max_d * max_w;
                self.draw_fill.draw_abs(cx, Rect { pos: dvec2(x_c - w, py2.min(py1)), size: dvec2(w * 2.0, (py1 - py2).abs()) });
            }

            self.draw_line.color = color;
            for j in 0..kde.len() - 1 {
                let (y1, d1) = kde[j]; let (y2, d2) = kde[j + 1];
                let py1 = self.plot_area.bottom - (y1 - y_min) / (y_max - y_min) * self.plot_area.height();
                let py2 = self.plot_area.bottom - (y2 - y_min) / (y_max - y_min) * self.plot_area.height();
                let w1 = d1 / max_d * max_w; let w2 = d2 / max_d * max_w;
                self.draw_line.draw_line(cx, dvec2(x_c - w1, py1), dvec2(x_c - w2, py2), 1.5);
                self.draw_line.draw_line(cx, dvec2(x_c + w1, py1), dvec2(x_c + w2, py2), 1.5);
            }

            if self.show_box {
                let mut s = item.values.clone(); s.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let q1 = s[s.len() / 4]; let med = s[s.len() / 2]; let q3 = s[3 * s.len() / 4];
                let py_q1 = self.plot_area.bottom - (q1 - y_min) / (y_max - y_min) * self.plot_area.height();
                let py_m = self.plot_area.bottom - (med - y_min) / (y_max - y_min) * self.plot_area.height();
                let py_q3 = self.plot_area.bottom - (q3 - y_min) / (y_max - y_min) * self.plot_area.height();
                let bw = max_w * 0.15;
                self.draw_fill.color = vec4(0.3, 0.3, 0.3, 0.8);
                self.draw_fill.draw_abs(cx, Rect { pos: dvec2(x_c - bw, py_q3), size: dvec2(bw * 2.0, py_q1 - py_q3) });
                self.draw_line.color = vec4(1.0, 1.0, 1.0, 1.0);
                self.draw_line.draw_line(cx, dvec2(x_c - bw, py_m), dvec2(x_c + bw, py_m), 2.0);
            }
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
        if !self.title.is_empty() {
            self.label.draw_at(cx, dvec2((self.plot_area.left + self.plot_area.right) / 2.0, self.plot_area.top - 15.0), &self.title, TextAnchor::Center);
        }
        let n = self.items.len();
        let band_w = self.plot_area.width() / n.max(1) as f64;
        for (i, item) in self.items.iter().enumerate() {
            let x = self.plot_area.left + (i as f64 + 0.5) * band_w;
            self.label.draw_at(cx, dvec2(x, self.plot_area.bottom + 15.0), &item.label, TextAnchor::Center);
        }
    }
}

impl ViolinPlotRef {
    pub fn set_title(&self, title: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); } }
    pub fn add_from_values(&self, label: impl Into<String>, values: &[f64]) { if let Some(mut inner) = self.borrow_mut() { inner.add_from_values(label, values); } }
    pub fn set_show_box(&self, show: bool) { if let Some(mut inner) = self.borrow_mut() { inner.set_show_box(show); } }
    pub fn clear(&self) { if let Some(mut inner) = self.borrow_mut() { inner.clear(); } }
    pub fn redraw(&self, cx: &mut Cx) { if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); } }
}

