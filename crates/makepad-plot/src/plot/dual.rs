use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub SubplotGrid = {{SubplotGrid}} {
        width: Fill,
        height: Fill,
        flow: Down,
        spacing: 10.0,
    }

    pub LinePlotDual = {{LinePlotDual}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SubplotGrid {
    #[deref] #[live] view: View,
    #[live] draw_bg: DrawColor,
    #[rust] rows: usize,
    #[rust] cols: usize,
    #[rust] h_spacing: f64,
    #[rust] v_spacing: f64,
}

impl SubplotGrid {
    pub fn set_grid(&mut self, rows: usize, cols: usize) {
        self.rows = rows;
        self.cols = cols;
    }

    pub fn set_spacing(&mut self, h: f64, v: f64) {
        self.h_spacing = h;
        self.v_spacing = v;
    }

    pub fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }
}

impl Widget for SubplotGrid {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Initialize defaults
        if self.rows == 0 { self.rows = 2; }
        if self.cols == 0 { self.cols = 2; }
        if self.h_spacing == 0.0 { self.h_spacing = 10.0; }
        if self.v_spacing == 0.0 { self.v_spacing = 10.0; }

        self.view.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl SubplotGridRef {
    pub fn set_grid(&self, rows: usize, cols: usize) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_grid(rows, cols); }
    }
    pub fn set_spacing(&self, h: f64, v: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_spacing(h, v); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}

// =============================================================================
// LinePlotDual - Line plot with dual y-axes (twinx support)
// =============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct LinePlotDual {
    #[deref] #[live] view: View,
    #[live] draw_line: DrawPlotLine,
    #[live] draw_fill: DrawPlotFill,
    #[live] draw_point: DrawPlotPoint,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] x_label: String,
    #[rust] y_label: String,
    #[rust] y2_label: String,
    #[rust] series_left: Vec<Series>,
    #[rust] series_right: Vec<Series>,
    #[rust] x_range: (f64, f64),
    #[rust] y_range: (f64, f64),
    #[rust] y2_range: (f64, f64),
    #[rust] show_grid: bool,
    #[rust] show_legend: bool,
    #[rust] legend_position: LegendPosition,
}

impl LinePlotDual {
    pub fn set_title(&mut self, title: impl Into<String>) { self.title = title.into(); }
    pub fn set_xlabel(&mut self, label: impl Into<String>) { self.x_label = label.into(); }
    pub fn set_ylabel(&mut self, label: impl Into<String>) { self.y_label = label.into(); }
    pub fn set_y2label(&mut self, label: impl Into<String>) { self.y2_label = label.into(); }

    pub fn add_series_left(&mut self, series: Series) {
        self.series_left.push(series);
        self.auto_range_left();
    }

    pub fn add_series_right(&mut self, series: Series) {
        self.series_right.push(series);
        self.auto_range_right();
    }

    fn auto_range_left(&mut self) {
        let mut x_min = f64::MAX; let mut x_max = f64::MIN;
        let mut y_min = f64::MAX; let mut y_max = f64::MIN;

        for s in &self.series_left {
            for &v in &s.x { if v < x_min { x_min = v; } if v > x_max { x_max = v; } }
            for &v in &s.y { if v < y_min { y_min = v; } if v > y_max { y_max = v; } }
        }

        if x_min != f64::MAX {
            let pad_x = (x_max - x_min).max(0.1) * 0.05;
            let pad_y = (y_max - y_min).max(0.1) * 0.1;
            self.x_range = (x_min - pad_x, x_max + pad_x);
            self.y_range = (y_min - pad_y, y_max + pad_y);
        }
    }

    fn auto_range_right(&mut self) {
        let mut y_min = f64::MAX; let mut y_max = f64::MIN;

        for s in &self.series_right {
            for &v in &s.y { if v < y_min { y_min = v; } if v > y_max { y_max = v; } }
        }

        if y_min != f64::MAX {
            let pad = (y_max - y_min).max(0.1) * 0.1;
            self.y2_range = (y_min - pad, y_max + pad);
        }
    }

    pub fn set_xlim(&mut self, min: f64, max: f64) { self.x_range = (min, max); }
    pub fn set_ylim(&mut self, min: f64, max: f64) { self.y_range = (min, max); }
    pub fn set_y2lim(&mut self, min: f64, max: f64) { self.y2_range = (min, max); }
    pub fn set_grid(&mut self, show: bool) { self.show_grid = show; }
    pub fn set_legend(&mut self, pos: LegendPosition) { self.show_legend = true; self.legend_position = pos; }
    pub fn clear(&mut self) { self.series_left.clear(); self.series_right.clear(); }
    pub fn redraw(&mut self, cx: &mut Cx) { self.view.redraw(cx); }
}

impl Widget for LinePlotDual {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let margin_left = 60.0;
            let margin_right = 60.0;
            let margin_top = 40.0;
            let margin_bottom = 50.0;

            let plot_rect = Rect {
                pos: dvec2(rect.pos.x + margin_left, rect.pos.y + margin_top),
                size: dvec2(rect.size.x - margin_left - margin_right, rect.size.y - margin_top - margin_bottom),
            };

            if plot_rect.size.x > 0.0 && plot_rect.size.y > 0.0 {
                // Initialize ranges if needed
                if self.x_range.0 >= self.x_range.1 { self.x_range = (0.0, 1.0); }
                if self.y_range.0 >= self.y_range.1 { self.y_range = (0.0, 1.0); }
                if self.y2_range.0 >= self.y2_range.1 { self.y2_range = (0.0, 1.0); }

                // Draw grid
                if self.show_grid {
                    self.draw_line.color = vec4(0.9, 0.9, 0.9, 1.0);
                    for i in 0..=5 {
                        let t = i as f64 / 5.0;
                        let x = plot_rect.pos.x + t * plot_rect.size.x;
                        let y = plot_rect.pos.y + t * plot_rect.size.y;
                        self.draw_line.draw_line(cx, dvec2(x, plot_rect.pos.y), dvec2(x, plot_rect.pos.y + plot_rect.size.y), 1.0);
                        self.draw_line.draw_line(cx, dvec2(plot_rect.pos.x, y), dvec2(plot_rect.pos.x + plot_rect.size.x, y), 1.0);
                    }
                }

                // Draw axes
                self.draw_line.color = vec4(0.2, 0.2, 0.2, 1.0);
                self.draw_line.draw_line(cx, dvec2(plot_rect.pos.x, plot_rect.pos.y + plot_rect.size.y),
                    dvec2(plot_rect.pos.x + plot_rect.size.x, plot_rect.pos.y + plot_rect.size.y), 1.5);
                self.draw_line.draw_line(cx, dvec2(plot_rect.pos.x, plot_rect.pos.y),
                    dvec2(plot_rect.pos.x, plot_rect.pos.y + plot_rect.size.y), 1.5);
                // Right y-axis
                self.draw_line.draw_line(cx, dvec2(plot_rect.pos.x + plot_rect.size.x, plot_rect.pos.y),
                    dvec2(plot_rect.pos.x + plot_rect.size.x, plot_rect.pos.y + plot_rect.size.y), 1.5);

                // Draw left series
                for (idx, s) in self.series_left.iter().enumerate() {
                    let color = s.color.unwrap_or_else(|| get_color(idx));
                    let line_width = s.line_width.unwrap_or(1.5);
                    self.draw_line.color = color;
                    let n = s.x.len().min(s.y.len());
                    for i in 1..n {
                        let x0 = plot_rect.pos.x + (s.x[i-1] - self.x_range.0) / (self.x_range.1 - self.x_range.0) * plot_rect.size.x;
                        let y0 = plot_rect.pos.y + plot_rect.size.y - (s.y[i-1] - self.y_range.0) / (self.y_range.1 - self.y_range.0) * plot_rect.size.y;
                        let x1 = plot_rect.pos.x + (s.x[i] - self.x_range.0) / (self.x_range.1 - self.x_range.0) * plot_rect.size.x;
                        let y1 = plot_rect.pos.y + plot_rect.size.y - (s.y[i] - self.y_range.0) / (self.y_range.1 - self.y_range.0) * plot_rect.size.y;
                        self.draw_line.draw_line(cx, dvec2(x0, y0), dvec2(x1, y1), line_width);
                    }
                }

                // Draw right series (uses y2_range)
                for (idx, s) in self.series_right.iter().enumerate() {
                    let color = s.color.unwrap_or_else(|| get_color(idx + self.series_left.len()));
                    let line_width = s.line_width.unwrap_or(1.5);
                    self.draw_line.color = color;
                    let n = s.x.len().min(s.y.len());
                    for i in 1..n {
                        let x0 = plot_rect.pos.x + (s.x[i-1] - self.x_range.0) / (self.x_range.1 - self.x_range.0) * plot_rect.size.x;
                        let y0 = plot_rect.pos.y + plot_rect.size.y - (s.y[i-1] - self.y2_range.0) / (self.y2_range.1 - self.y2_range.0) * plot_rect.size.y;
                        let x1 = plot_rect.pos.x + (s.x[i] - self.x_range.0) / (self.x_range.1 - self.x_range.0) * plot_rect.size.x;
                        let y1 = plot_rect.pos.y + plot_rect.size.y - (s.y[i] - self.y2_range.0) / (self.y2_range.1 - self.y2_range.0) * plot_rect.size.y;
                        self.draw_line.draw_line(cx, dvec2(x0, y0), dvec2(x1, y1), line_width);
                    }
                }

                // Draw axis labels
                // Left Y-axis labels
                for i in 0..=5 {
                    let t = i as f64 / 5.0;
                    let val = self.y_range.0 + t * (self.y_range.1 - self.y_range.0);
                    let y = plot_rect.pos.y + plot_rect.size.y - t * plot_rect.size.y;
                    self.label.draw_at(cx, dvec2(plot_rect.pos.x - 5.0, y), &format!("{:.1}", val), TextAnchor::MiddleRight);
                }

                // Right Y-axis labels
                for i in 0..=5 {
                    let t = i as f64 / 5.0;
                    let val = self.y2_range.0 + t * (self.y2_range.1 - self.y2_range.0);
                    let y = plot_rect.pos.y + plot_rect.size.y - t * plot_rect.size.y;
                    self.label.draw_at(cx, dvec2(plot_rect.pos.x + plot_rect.size.x + 5.0, y), &format!("{:.1}", val), TextAnchor::MiddleLeft);
                }

                // X-axis labels
                for i in 0..=5 {
                    let t = i as f64 / 5.0;
                    let val = self.x_range.0 + t * (self.x_range.1 - self.x_range.0);
                    let x = plot_rect.pos.x + t * plot_rect.size.x;
                    self.label.draw_at(cx, dvec2(x, plot_rect.pos.y + plot_rect.size.y + 15.0), &format!("{:.1}", val), TextAnchor::TopCenter);
                }

                // Title
                if !self.title.is_empty() {
                    self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x * 0.5, rect.pos.y + 10.0), &self.title, TextAnchor::TopCenter);
                }

                // Y-axis label (left)
                if !self.y_label.is_empty() {
                    self.label.draw_at(cx, dvec2(rect.pos.x + 15.0, plot_rect.pos.y + plot_rect.size.y * 0.5), &self.y_label, TextAnchor::MiddleLeft);
                }

                // Y2-axis label (right)
                if !self.y2_label.is_empty() {
                    self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x - 15.0, plot_rect.pos.y + plot_rect.size.y * 0.5), &self.y2_label, TextAnchor::MiddleRight);
                }

                // Legend
                if self.show_legend {
                    let all_series: Vec<&Series> = self.series_left.iter().chain(self.series_right.iter()).collect();
                    if !all_series.is_empty() {
                        let legend_x = match self.legend_position {
                            LegendPosition::TopLeft | LegendPosition::BottomLeft => plot_rect.pos.x + 10.0,
                            _ => plot_rect.pos.x + plot_rect.size.x - 80.0,
                        };
                        let legend_y = match self.legend_position {
                            LegendPosition::TopLeft | LegendPosition::TopRight => plot_rect.pos.y + 10.0,
                            _ => plot_rect.pos.y + plot_rect.size.y - 10.0 - all_series.len() as f64 * 18.0,
                        };

                        for (i, s) in all_series.iter().enumerate() {
                            let y = legend_y + i as f64 * 18.0;
                            self.draw_line.color = s.color.unwrap_or_else(|| get_color(i));
                            self.draw_line.draw_line(cx, dvec2(legend_x, y + 6.0), dvec2(legend_x + 20.0, y + 6.0), 2.0);
                            self.label.draw_at(cx, dvec2(legend_x + 25.0, y), &s.label, TextAnchor::TopLeft);
                        }
                    }
                }
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl LinePlotDualRef {
    pub fn set_title(&self, title: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); } }
    pub fn set_xlabel(&self, label: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_xlabel(label); } }
    pub fn set_ylabel(&self, label: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_ylabel(label); } }
    pub fn set_y2label(&self, label: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_y2label(label); } }
    pub fn add_series_left(&self, s: Series) { if let Some(mut inner) = self.borrow_mut() { inner.add_series_left(s); } }
    pub fn add_series_right(&self, s: Series) { if let Some(mut inner) = self.borrow_mut() { inner.add_series_right(s); } }
    pub fn set_xlim(&self, min: f64, max: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_xlim(min, max); } }
    pub fn set_ylim(&self, min: f64, max: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_ylim(min, max); } }
    pub fn set_y2lim(&self, min: f64, max: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_y2lim(min, max); } }
    pub fn set_grid(&self, show: bool) { if let Some(mut inner) = self.borrow_mut() { inner.set_grid(show); } }
    pub fn set_legend(&self, pos: LegendPosition) { if let Some(mut inner) = self.borrow_mut() { inner.set_legend(pos); } }
    pub fn clear(&self) { if let Some(mut inner) = self.borrow_mut() { inner.clear(); } }
    pub fn redraw(&self, cx: &mut Cx) { if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); } }
}

