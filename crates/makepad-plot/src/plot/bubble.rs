use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub BubbleChart = {{BubbleChart}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

#[derive(Clone)]
pub struct BubblePoint {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Option<Vec4>,
    pub label: Option<String>,
}

impl BubblePoint {
    pub fn new(x: f64, y: f64, size: f64) -> Self {
        Self { x, y, size, color: None, label: None }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

#[derive(Clone)]
pub struct BubbleSeries {
    pub name: String,
    pub points: Vec<BubblePoint>,
    pub color: Vec4,
}

impl BubbleSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            points: Vec::new(),
            color: get_color(0),
        }
    }

    pub fn with_points(mut self, points: Vec<BubblePoint>) -> Self {
        self.points = points;
        self
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BubbleChart {
    #[deref] #[live] view: View,
    #[live] draw_line: DrawPlotLine,
    #[live] draw_bubble: DrawPlotPointGradient,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] series: Vec<BubbleSeries>,
    #[rust] x_label: String,
    #[rust] y_label: String,
    #[rust] show_grid: bool,
    #[rust] max_bubble_radius: f64,
    #[rust] min_bubble_radius: f64,
    #[rust] use_gradient: bool,
}

impl BubbleChart {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn add_series(&mut self, series: BubbleSeries) {
        self.series.push(series);
    }

    pub fn set_x_label(&mut self, label: impl Into<String>) {
        self.x_label = label.into();
    }

    pub fn set_y_label(&mut self, label: impl Into<String>) {
        self.y_label = label.into();
    }

    pub fn set_show_grid(&mut self, show: bool) {
        self.show_grid = show;
    }

    pub fn set_bubble_radius_range(&mut self, min: f64, max: f64) {
        self.min_bubble_radius = min;
        self.max_bubble_radius = max;
    }

    pub fn set_use_gradient(&mut self, use_gradient: bool) {
        self.use_gradient = use_gradient;
    }

    pub fn clear(&mut self) {
        self.series.clear();
        self.use_gradient = false;
    }

    fn get_data_bounds(&self) -> (f64, f64, f64, f64, f64, f64) {
        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;
        let mut size_min = f64::MAX;
        let mut size_max = f64::MIN;

        for series in &self.series {
            for point in &series.points {
                if point.x < x_min { x_min = point.x; }
                if point.x > x_max { x_max = point.x; }
                if point.y < y_min { y_min = point.y; }
                if point.y > y_max { y_max = point.y; }
                if point.size < size_min { size_min = point.size; }
                if point.size > size_max { size_max = point.size; }
            }
        }

        if x_min == f64::MAX { x_min = 0.0; x_max = 1.0; }
        if y_min == f64::MAX { y_min = 0.0; y_max = 1.0; }
        if size_min == f64::MAX { size_min = 1.0; size_max = 1.0; }

        let x_range = (x_max - x_min).max(0.001);
        let y_range = (y_max - y_min).max(0.001);
        x_min -= x_range * 0.1;
        x_max += x_range * 0.1;
        y_min -= y_range * 0.1;
        y_max += y_range * 0.1;

        (x_min, x_max, y_min, y_max, size_min, size_max)
    }
}

impl Widget for BubbleChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            // Set defaults
            if self.max_bubble_radius == 0.0 { self.max_bubble_radius = 40.0; }
            if self.min_bubble_radius == 0.0 { self.min_bubble_radius = 5.0; }

            let padding_left = 60.0;
            let padding_right = 40.0;
            let padding_top = 40.0;
            let padding_bottom = 50.0;

            let plot_left = rect.pos.x + padding_left;
            let plot_top = rect.pos.y + padding_top;
            let plot_right = rect.pos.x + rect.size.x - padding_right;
            let plot_bottom = rect.pos.y + rect.size.y - padding_bottom;
            let plot_width = plot_right - plot_left;
            let plot_height = plot_bottom - plot_top;

            let (x_min, x_max, y_min, y_max, size_min, size_max) = self.get_data_bounds();
            let x_range = (x_max - x_min).max(0.001);
            let y_range = (y_max - y_min).max(0.001);
            let size_range = (size_max - size_min).max(0.001);

            // Draw grid
            if self.show_grid {
                self.draw_line.color = vec4(0.9, 0.9, 0.9, 1.0);
                for i in 0..=5 {
                    let t = i as f64 / 5.0;
                    let x = plot_left + t * plot_width;
                    let y = plot_top + t * plot_height;
                    self.draw_line.draw_line(cx, dvec2(x, plot_top), dvec2(x, plot_bottom), 1.0);
                    self.draw_line.draw_line(cx, dvec2(plot_left, y), dvec2(plot_right, y), 1.0);
                }
            }

            // Draw axes
            self.draw_line.color = vec4(0.3, 0.3, 0.3, 1.0);
            self.draw_line.draw_line(cx, dvec2(plot_left, plot_bottom), dvec2(plot_right, plot_bottom), 1.5);
            self.draw_line.draw_line(cx, dvec2(plot_left, plot_top), dvec2(plot_left, plot_bottom), 1.5);

            // Draw axis tick labels
            self.label.draw_text.color = vec4(0.3, 0.3, 0.3, 1.0);
            for i in 0..=5 {
                let t = i as f64 / 5.0;
                let x_val = x_min + t * x_range;
                let y_val = y_min + (1.0 - t) * y_range;
                let x = plot_left + t * plot_width;
                let y = plot_top + t * plot_height;

                self.label.draw_at(cx, dvec2(x, plot_bottom + 15.0), &format!("{:.1}", x_val), TextAnchor::TopCenter);
                self.label.draw_at(cx, dvec2(plot_left - 10.0, y), &format!("{:.1}", y_val), TextAnchor::MiddleRight);
            }

            // Draw bubbles
            for series in &self.series {
                let base_color = series.color;

                for point in &series.points {
                    let px = plot_left + ((point.x - x_min) / x_range) * plot_width;
                    let py = plot_bottom - ((point.y - y_min) / y_range) * plot_height;

                    let size_norm = (point.size - size_min) / size_range;
                    let radius = self.min_bubble_radius + size_norm * (self.max_bubble_radius - self.min_bubble_radius);

                    let color = point.color.unwrap_or(base_color);

                    if self.use_gradient {
                        // Draw bubble with radial gradient using same-hue lighter/darker colors
                        let (center, outer) = gradient_pair(color);
                        let center_color = vec4(center.x, center.y, center.z, 0.9);
                        let edge_color = vec4(outer.x, outer.y, outer.z, 0.85);
                        self.draw_bubble.color = color;
                        self.draw_bubble.draw_point_gradient(cx, dvec2(px, py), radius, center_color, edge_color);
                    } else {
                        // Draw bubble with solid color and slight transparency
                        let fill_color = vec4(color.x, color.y, color.z, 0.6);
                        self.draw_bubble.color = fill_color;
                        self.draw_bubble.draw_point(cx, dvec2(px, py), radius);
                    }

                    // Draw circle outline
                    self.draw_line.color = color;
                    let segments = 32;
                    for i in 0..segments {
                        let angle1 = (i as f64 / segments as f64) * 2.0 * std::f64::consts::PI;
                        let angle2 = ((i + 1) as f64 / segments as f64) * 2.0 * std::f64::consts::PI;
                        let x1 = px + radius * angle1.cos();
                        let y1 = py + radius * angle1.sin();
                        let x2 = px + radius * angle2.cos();
                        let y2 = py + radius * angle2.sin();
                        self.draw_line.draw_line(cx, dvec2(x1, y1), dvec2(x2, y2), 1.5);
                    }

                    // Draw label if present
                    if let Some(label) = &point.label {
                        self.label.draw_text.color = vec4(0.2, 0.2, 0.2, 1.0);
                        self.label.draw_at(cx, dvec2(px, py - radius - 5.0), label, TextAnchor::BottomCenter);
                    }
                }
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_text.color = vec4(0.2, 0.2, 0.2, 1.0);
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 15.0), &self.title, TextAnchor::TopCenter);
            }

            // Draw x-axis label
            if !self.x_label.is_empty() {
                self.label.draw_at(cx, dvec2((plot_left + plot_right) / 2.0, rect.pos.y + rect.size.y - 10.0), &self.x_label, TextAnchor::BottomCenter);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl BubbleChart {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }
}

impl BubbleChartRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn add_series(&self, series: BubbleSeries) {
        if let Some(mut inner) = self.borrow_mut() { inner.add_series(series); }
    }
    pub fn set_x_label(&self, label: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_x_label(label); }
    }
    pub fn set_y_label(&self, label: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_y_label(label); }
    }
    pub fn set_show_grid(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_grid(show); }
    }
    pub fn set_bubble_radius_range(&self, min: f64, max: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_bubble_radius_range(min, max); }
    }
    pub fn set_use_gradient(&self, use_gradient: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_use_gradient(use_gradient); }
    }
    pub fn clear(&self) {
        if let Some(mut inner) = self.borrow_mut() { inner.clear(); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}

