use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub PolarPlot = {{PolarPlot}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    pub RadarChart = {{RadarChart}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

pub struct PolarSeries {
    pub label: String, pub theta: Vec<f64>, pub r: Vec<f64>,
    pub color: Option<Vec4>, pub marker_style: MarkerStyle, pub fill: bool,
}

impl PolarSeries {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), theta: Vec::new(), r: Vec::new(), color: None, marker_style: MarkerStyle::None, fill: false }
    }
    pub fn with_data(mut self, theta: Vec<f64>, r: Vec<f64>) -> Self { self.theta = theta; self.r = r; self }
    pub fn with_color(mut self, color: Vec4) -> Self { self.color = Some(color); self }
    pub fn with_fill(mut self, fill: bool) -> Self { self.fill = fill; self }
    pub fn with_marker(mut self, style: MarkerStyle) -> Self { self.marker_style = style; self }
}

#[derive(Live, LiveHook, Widget)]
pub struct PolarPlot {
    #[deref] #[live] view: View,
    #[live] draw_line: DrawPlotLine,
    #[live] draw_point: DrawPlotPoint,
    #[live] draw_fill: DrawPlotFill,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] series: Vec<PolarSeries>,
    #[rust] r_max: Option<f64>,
    #[rust] plot_center: DVec2,
    #[rust] plot_radius: f64,
    #[live(20.0)] margin: f64,
}

impl Widget for PolarPlot {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) { self.view.handle_event(cx, event, scope); }
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.series.is_empty() {
            let size = rect.size.x.min(rect.size.y) - self.margin * 2.0;
            self.plot_radius = size / 2.0;
            self.plot_center = dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + rect.size.y / 2.0);
            self.draw_grid(cx);
            self.draw_data(cx);
            self.draw_labels(cx);
        }
        DrawStep::done()
    }
}

impl PolarPlot {
    pub fn set_title(&mut self, title: impl Into<String>) { self.title = title.into(); }
    pub fn add_series(&mut self, series: PolarSeries) { self.series.push(series); }
    pub fn set_r_max(&mut self, r_max: f64) { self.r_max = Some(r_max); }
    pub fn clear(&mut self) { self.series.clear(); }

    fn get_r_max(&self) -> f64 {
        self.r_max.unwrap_or_else(|| self.series.iter().flat_map(|s| s.r.iter()).cloned().fold(0.0f64, f64::max) * 1.1)
    }

    fn polar_to_cart(&self, theta: f64, r: f64, r_max: f64) -> DVec2 {
        let nr = r / r_max * self.plot_radius;
        dvec2(self.plot_center.x + nr * theta.cos(), self.plot_center.y - nr * theta.sin())
    }

    fn draw_grid(&mut self, cx: &mut Cx2d) {
        self.draw_line.color = vec4(0.85, 0.85, 0.85, 1.0);
        for i in 1..=5 {
            let r = i as f64 / 5.0 * self.plot_radius;
            for j in 0..64 {
                let t1 = j as f64 / 64.0 * 2.0 * std::f64::consts::PI;
                let t2 = (j + 1) as f64 / 64.0 * 2.0 * std::f64::consts::PI;
                self.draw_line.draw_line(cx, dvec2(self.plot_center.x + r * t1.cos(), self.plot_center.y - r * t1.sin()),
                    dvec2(self.plot_center.x + r * t2.cos(), self.plot_center.y - r * t2.sin()), 1.0);
            }
        }
        for i in 0..12 {
            let t = i as f64 * std::f64::consts::PI / 6.0;
            self.draw_line.draw_line(cx, self.plot_center, dvec2(self.plot_center.x + self.plot_radius * t.cos(), self.plot_center.y - self.plot_radius * t.sin()), 1.0);
        }
        self.draw_line.color = vec4(0.3, 0.3, 0.3, 1.0);
        for j in 0..64 {
            let t1 = j as f64 / 64.0 * 2.0 * std::f64::consts::PI;
            let t2 = (j + 1) as f64 / 64.0 * 2.0 * std::f64::consts::PI;
            self.draw_line.draw_line(cx, dvec2(self.plot_center.x + self.plot_radius * t1.cos(), self.plot_center.y - self.plot_radius * t1.sin()),
                dvec2(self.plot_center.x + self.plot_radius * t2.cos(), self.plot_center.y - self.plot_radius * t2.sin()), 1.5);
        }
    }

    fn draw_data(&mut self, cx: &mut Cx2d) {
        let r_max = self.get_r_max();
        for (idx, s) in self.series.iter().enumerate() {
            if s.theta.len() != s.r.len() || s.theta.is_empty() { continue; }
            let color = s.color.unwrap_or_else(|| get_color(idx));
            self.draw_line.color = color;
            for i in 0..s.theta.len() {
                let next = (i + 1) % s.theta.len();
                self.draw_line.draw_line(cx, self.polar_to_cart(s.theta[i], s.r[i], r_max), self.polar_to_cart(s.theta[next], s.r[next], r_max), 2.0);
            }
            if s.marker_style != MarkerStyle::None {
                self.draw_point.color = color;
                for i in 0..s.theta.len() {
                    self.draw_point.draw_marker(cx, self.polar_to_cart(s.theta[i], s.r[i], r_max), 5.0, s.marker_style);
                }
            }
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
        if !self.title.is_empty() {
            self.label.draw_at(cx, dvec2(self.plot_center.x, self.plot_center.y - self.plot_radius - 20.0), &self.title, TextAnchor::Center);
        }
        for &deg in &[0, 90, 180, 270] {
            let t = deg as f64 * std::f64::consts::PI / 180.0;
            let r = self.plot_radius + 15.0;
            self.label.draw_at(cx, dvec2(self.plot_center.x + r * t.cos(), self.plot_center.y - r * t.sin()), &format!("{}°", deg), TextAnchor::Center);
        }
    }
}

impl PolarPlotRef {
    pub fn set_title(&self, title: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); } }
    pub fn add_series(&self, series: PolarSeries) { if let Some(mut inner) = self.borrow_mut() { inner.add_series(series); } }
    pub fn set_r_max(&self, r_max: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_r_max(r_max); } }
    pub fn clear(&self) { if let Some(mut inner) = self.borrow_mut() { inner.clear(); } }
    pub fn redraw(&self, cx: &mut Cx) { if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); } }
}


// =============================================================================
// RadarChart Widget
// =============================================================================

pub struct RadarSeries {
    pub label: String,
    pub values: Vec<f64>,
    pub color: Vec4,
    pub fill_alpha: f64,
}

impl RadarSeries {
    pub fn new(label: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            label: label.into(),
            values,
            color: vec4(0.12, 0.47, 0.71, 1.0),
            fill_alpha: 0.3,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn with_fill_alpha(mut self, alpha: f64) -> Self {
        self.fill_alpha = alpha;
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct RadarChart {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] draw_triangle: DrawTriangle,
    #[live] draw_point: DrawPlotPointGradient,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] axes: Vec<String>,
    #[rust] series: Vec<RadarSeries>,
    #[rust] max_value: f64,
    #[rust] show_grid: bool,
    #[rust] grid_levels: usize,
    #[rust] use_gradient: bool,
}

impl RadarChart {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_axes(&mut self, axes: Vec<String>) {
        self.axes = axes;
    }

    pub fn add_series(&mut self, series: RadarSeries) {
        self.series.push(series);
    }

    pub fn set_max_value(&mut self, max: f64) {
        self.max_value = max;
    }

    pub fn set_show_grid(&mut self, show: bool) {
        self.show_grid = show;
    }

    pub fn set_grid_levels(&mut self, levels: usize) {
        self.grid_levels = levels;
    }

    pub fn set_use_gradient(&mut self, use_gradient: bool) {
        self.use_gradient = use_gradient;
    }

    pub fn clear(&mut self) {
        self.series.clear();
        self.use_gradient = false;
    }

    fn compute_max(&self) -> f64 {
        if self.max_value > 0.0 {
            return self.max_value;
        }
        let max = self.series.iter()
            .flat_map(|s| s.values.iter())
            .cloned()
            .fold(0.0f64, f64::max);
        if max > 0.0 { max * 1.1 } else { 1.0 }
    }
}

impl Widget for RadarChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let num_axes = self.axes.len();
            if num_axes < 3 {
                // Need at least 3 axes for radar chart
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + rect.size.y / 2.0),
                    "Need at least 3 axes", TextAnchor::Center);
                return DrawStep::done();
            }

            // Calculate center and radius
            let margin = 60.0;
            let center = dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + rect.size.y / 2.0 + 10.0);
            let radius = (rect.size.x.min(rect.size.y) / 2.0 - margin).max(50.0);

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 5.0),
                    &self.title, TextAnchor::TopCenter);
            }

            let max_val = self.compute_max();
            let angle_step = std::f64::consts::TAU / num_axes as f64;

            // Initialize grid settings
            let grid_levels = if self.grid_levels > 0 { self.grid_levels } else { 5 };
            let show_grid = self.show_grid || self.grid_levels == 0; // Default to true

            // Draw grid circles
            if show_grid {
                self.draw_line.color = vec4(0.8, 0.8, 0.8, 0.5);
                for level in 1..=grid_levels {
                    let r = radius * level as f64 / grid_levels as f64;
                    // Draw polygon for this level
                    for i in 0..num_axes {
                        let angle1 = -std::f64::consts::FRAC_PI_2 + i as f64 * angle_step;
                        let angle2 = -std::f64::consts::FRAC_PI_2 + ((i + 1) % num_axes) as f64 * angle_step;
                        let p1 = dvec2(center.x + r * angle1.cos(), center.y + r * angle1.sin());
                        let p2 = dvec2(center.x + r * angle2.cos(), center.y + r * angle2.sin());
                        self.draw_line.draw_line(cx, p1, p2, 0.5);
                    }
                }
            }

            // Draw axis lines and labels
            self.draw_line.color = vec4(0.5, 0.5, 0.5, 1.0);
            for (i, axis_name) in self.axes.iter().enumerate() {
                let angle = -std::f64::consts::FRAC_PI_2 + i as f64 * angle_step;
                let end = dvec2(center.x + radius * angle.cos(), center.y + radius * angle.sin());

                // Draw axis line
                self.draw_line.draw_line(cx, center, end, 1.0);

                // Draw axis label
                let label_pos = dvec2(
                    center.x + (radius + 15.0) * angle.cos(),
                    center.y + (radius + 15.0) * angle.sin()
                );
                let anchor = if angle.cos().abs() < 0.1 {
                    if angle.sin() < 0.0 { TextAnchor::BottomCenter } else { TextAnchor::TopCenter }
                } else if angle.cos() > 0.0 {
                    TextAnchor::MiddleLeft
                } else {
                    TextAnchor::MiddleRight
                };
                self.label.draw_at(cx, label_pos, axis_name, anchor);
            }

            // Draw series
            for series in &self.series {
                if series.values.len() != num_axes {
                    continue;
                }

                // Collect points
                let points: Vec<DVec2> = series.values.iter().enumerate().map(|(i, &val)| {
                    let angle = -std::f64::consts::FRAC_PI_2 + i as f64 * angle_step;
                    let r = (val / max_val).min(1.0) * radius;
                    dvec2(center.x + r * angle.cos(), center.y + r * angle.sin())
                }).collect();

                // Draw filled polygon using proper triangles
                if series.fill_alpha > 0.0 {
                    let fill_color = vec4(series.color.x, series.color.y, series.color.z, series.fill_alpha as f32);
                    // Draw triangles from center to each edge
                    for i in 0..points.len() {
                        let p1 = points[i];
                        let p2 = points[(i + 1) % points.len()];

                        if self.use_gradient {
                            // Gradient from center (lighter) to edge (darker) - same hue, different brightness
                            let center_color = lighten(fill_color, 0.5);
                            let outer_color = darken(fill_color, 0.1);
                            self.draw_triangle.color = fill_color;
                            self.draw_triangle.draw_triangle_gradient(cx, center, p1, p2, center_color, outer_color);
                        } else {
                            self.draw_triangle.color = fill_color;
                            self.draw_triangle.draw_triangle(cx, center, p1, p2);
                        }
                    }
                }

                // Draw outline
                self.draw_line.color = series.color;
                for i in 0..points.len() {
                    let p1 = points[i];
                    let p2 = points[(i + 1) % points.len()];
                    self.draw_line.draw_line(cx, p1, p2, 2.0);
                }

                // Draw points with gradient
                for p in &points {
                    let point_size = 5.0;
                    if self.use_gradient {
                        // Draw with radial gradient using same-hue lighter/darker colors
                        let (center_color, outer_color) = gradient_pair(series.color);
                        self.draw_point.color = series.color;
                        self.draw_point.draw_point_gradient(cx, *p, point_size, center_color, outer_color);
                    } else {
                        self.draw_point.color = series.color;
                        self.draw_point.draw_point(cx, *p, point_size);
                    }
                }
            }

            // Draw legend
            if !self.series.is_empty() {
                let legend_x = rect.pos.x + rect.size.x - 100.0;
                let legend_y = rect.pos.y + 25.0;
                for (i, s) in self.series.iter().enumerate() {
                    let y = legend_y + i as f64 * 18.0;
                    self.draw_line.color = s.color;
                    self.draw_line.draw_line(cx, dvec2(legend_x, y + 6.0), dvec2(legend_x + 15.0, y + 6.0), 2.0);
                    self.label.draw_at(cx, dvec2(legend_x + 20.0, y), &s.label, TextAnchor::TopLeft);
                }
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl RadarChartRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn set_axes(&self, axes: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_axes(axes); }
    }
    pub fn add_series(&self, series: RadarSeries) {
        if let Some(mut inner) = self.borrow_mut() { inner.add_series(series); }
    }
    pub fn set_max_value(&self, max: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_max_value(max); }
    }
    pub fn set_show_grid(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_grid(show); }
    }
    pub fn set_grid_levels(&self, levels: usize) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_grid_levels(levels); }
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

