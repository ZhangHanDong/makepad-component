use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub ScatterPlot = {{ScatterPlot}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ScatterPlot {
    #[deref]
    #[live]
    view: View,

    #[live]
    draw_point: DrawPlotPoint,

    #[live]
    draw_point_gradient: DrawPlotPointGradient,

    #[live]
    draw_line: DrawPlotLine,

    #[live]
    label: PlotLabel,

    #[rust]
    series: Vec<Series>,

    #[rust]
    use_gradient: bool,

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

    #[rust(5.0)]
    point_radius: f64,

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

    // Pan/zoom state
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
}

impl Widget for ScatterPlot {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if !self.interactive {
            return;
        }

        match event.hits(cx, self.view.area()) {
            Hit::FingerDown(fe) => {
                self.is_dragging = true;
                self.drag_start = fe.abs;
                self.initial_x_range = self.x_range;
                self.initial_y_range = self.y_range;
            }
            Hit::FingerMove(fe) => {
                if self.is_dragging && self.plot_area.width() > 0.0 && self.plot_area.height() > 0.0 {
                    let dx_pixels = fe.abs.x - self.drag_start.x;
                    let dy_pixels = fe.abs.y - self.drag_start.y;

                    let x_range_size = self.initial_x_range.1 - self.initial_x_range.0;
                    let y_range_size = self.initial_y_range.1 - self.initial_y_range.0;

                    let dx_data = -dx_pixels * x_range_size / self.plot_area.width();
                    let dy_data = dy_pixels * y_range_size / self.plot_area.height();

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
                let zoom_factor = if fe.scroll.y > 0.0 { 0.9 } else { 1.1 };

                let mouse_x = fe.abs.x;
                let mouse_y = fe.abs.y;

                if mouse_x >= self.plot_area.left && mouse_x <= self.plot_area.right
                    && mouse_y >= self.plot_area.top && mouse_y <= self.plot_area.bottom
                {
                    let rel_x = (mouse_x - self.plot_area.left) / self.plot_area.width();
                    let rel_y = (self.plot_area.bottom - mouse_y) / self.plot_area.height();

                    let data_x = self.x_range.0 + rel_x * (self.x_range.1 - self.x_range.0);
                    let data_y = self.y_range.0 + rel_y * (self.y_range.1 - self.y_range.0);

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
            self.draw_points(cx);
            self.draw_labels(cx);
            self.draw_legend(cx);
        }

        DrawStep::done()
    }
}

impl ScatterPlot {
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

    /// Set point radius
    pub fn set_point_radius(&mut self, radius: f64) {
        self.point_radius = radius;
    }

    /// Set legend position
    pub fn set_legend(&mut self, position: LegendPosition) {
        self.legend_position = position;
    }

    /// Enable gradient points
    pub fn set_use_gradient(&mut self, use_gradient: bool) {
        self.use_gradient = use_gradient;
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

        // Add 10% padding
        let x_pad = (x_max - x_min) * 0.1;
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
        let px = self.plot_area.left + (x - self.x_range.0) / (self.x_range.1 - self.x_range.0) * self.plot_area.width();
        let py = self.plot_area.bottom - (y - self.y_range.0) / (self.y_range.1 - self.y_range.0) * self.plot_area.height();
        dvec2(px, py)
    }

    fn draw_grid(&mut self, cx: &mut Cx2d) {
        if !self.show_grid {
            return;
        }

        self.draw_line.color = vec4(0.9, 0.9, 0.9, 1.0);

        // Horizontal grid lines
        let y_ticks = self.generate_ticks(self.y_range.0, self.y_range.1, 5);
        for y in &y_ticks {
            let p1 = self.data_to_pixel(self.x_range.0, *y);
            let p2 = self.data_to_pixel(self.x_range.1, *y);
            self.draw_line.draw_line(cx, p1, p2, 0.5);
        }

        // Vertical grid lines
        let x_ticks = self.generate_ticks(self.x_range.0, self.x_range.1, 5);
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

    fn draw_points(&mut self, cx: &mut Cx2d) {
        for (idx, series) in self.series.iter().enumerate() {
            let color = series.color.unwrap_or_else(|| get_color(idx));

            for i in 0..series.x.len() {
                let p = self.data_to_pixel(series.x[i], series.y[i]);

                if self.use_gradient {
                    // Radial gradient using same-hue lighter/darker colors
                    let (center_color, outer_color) = gradient_pair(color);
                    self.draw_point_gradient.color = color;
                    self.draw_point_gradient.draw_point_gradient(cx, p, self.point_radius, center_color, outer_color);
                } else {
                    self.draw_point.color = color;
                    self.draw_point.draw_point(cx, p, self.point_radius);
                }
            }
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));

        // X axis tick labels
        let x_ticks = self.generate_ticks(self.x_range.0, self.x_range.1, 5);
        for x in &x_ticks {
            let p = self.data_to_pixel(*x, self.y_range.0);
            let label = format!("{:.1}", x);
            self.label.draw_at(cx, dvec2(p.x, p.y + 5.0), &label, TextAnchor::TopCenter);
        }

        // Y axis tick labels
        let y_ticks = self.generate_ticks(self.y_range.0, self.y_range.1, 5);
        for y in &y_ticks {
            let p = self.data_to_pixel(self.x_range.0, *y);
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
        if self.legend_position == LegendPosition::None || self.series.is_empty() {
            return;
        }

        let padding = 8.0;
        let line_height = 16.0;
        let marker_size = 10.0;
        let marker_text_gap = 6.0;
        let legend_height = self.series.len() as f64 * line_height + padding * 2.0;
        let legend_width = 100.0;

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
        self.draw_line.draw_line(cx, dvec2(legend_x, legend_y), dvec2(legend_x + legend_width, legend_y), 1.0);
        self.draw_line.draw_line(cx, dvec2(legend_x, legend_y + legend_height), dvec2(legend_x + legend_width, legend_y + legend_height), 1.0);
        self.draw_line.draw_line(cx, dvec2(legend_x, legend_y), dvec2(legend_x, legend_y + legend_height), 1.0);
        self.draw_line.draw_line(cx, dvec2(legend_x + legend_width, legend_y), dvec2(legend_x + legend_width, legend_y + legend_height), 1.0);

        // Draw legend entries
        for (idx, series) in self.series.iter().enumerate() {
            let color = series.color.unwrap_or_else(|| get_color(idx));
            let entry_y = legend_y + padding + idx as f64 * line_height + line_height / 2.0;

            self.draw_point.color = color;
            self.draw_point.draw_point(cx, dvec2(legend_x + padding + marker_size / 2.0, entry_y), marker_size / 2.0);

            self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
            self.label.draw_at(
                cx,
                dvec2(legend_x + padding + marker_size + marker_text_gap, entry_y),
                &series.label,
                TextAnchor::MiddleLeft,
            );
        }
    }

    fn generate_ticks(&self, min: f64, max: f64, count: usize) -> Vec<f64> {
        let step = (max - min) / count as f64;
        (0..=count).map(|i| min + i as f64 * step).collect()
    }

    /// Enable or disable interactive pan/zoom
    pub fn set_interactive(&mut self, interactive: bool) {
        self.interactive = interactive;
    }

    /// Reset view to auto-fit all data
    pub fn reset_view(&mut self) {
        self.auto_range();
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }
}

impl ScatterPlotRef {
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

    pub fn set_point_radius(&self, radius: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_point_radius(radius);
        }
    }

    pub fn set_legend(&self, position: LegendPosition) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_legend(position);
        }
    }

    pub fn set_interactive(&self, interactive: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_interactive(interactive);
        }
    }

    pub fn set_use_gradient(&self, use_gradient: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_use_gradient(use_gradient);
        }
    }

    pub fn reset_view(&self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.reset_view();
        }
    }

    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.redraw(cx);
        }
    }
}

