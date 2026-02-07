use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub AreaChart = {{AreaChart}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    pub StepPlot = {{StepPlot}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

pub struct AreaSeries {
    pub name: String,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub color: Vec4,
}

impl AreaSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            x: Vec::new(),
            y: Vec::new(),
            color: get_color(0),
        }
    }

    pub fn with_data(mut self, x: Vec<f64>, y: Vec<f64>) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct AreaChart {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] series: Vec<AreaSeries>,
    #[rust] x_label: String,
    #[rust] y_label: String,
    #[rust] stacked: bool,
    #[rust] show_grid: bool,
}

impl AreaChart {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn add_series(&mut self, series: AreaSeries) {
        self.series.push(series);
    }

    pub fn set_x_label(&mut self, label: impl Into<String>) {
        self.x_label = label.into();
    }

    pub fn set_y_label(&mut self, label: impl Into<String>) {
        self.y_label = label.into();
    }

    pub fn set_stacked(&mut self, stacked: bool) {
        self.stacked = stacked;
    }

    pub fn set_show_grid(&mut self, show: bool) {
        self.show_grid = show;
    }

    pub fn clear(&mut self) {
        self.series.clear();
    }

    fn get_bounds(&self) -> (f64, f64, f64, f64) {
        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_min = 0.0f64;
        let mut y_max = f64::MIN;

        if self.stacked {
            // For stacked, compute cumulative max
            if !self.series.is_empty() && !self.series[0].x.is_empty() {
                let n = self.series[0].x.len();
                for i in 0..n {
                    let mut sum = 0.0;
                    for s in &self.series {
                        if i < s.y.len() { sum += s.y[i]; }
                    }
                    if sum > y_max { y_max = sum; }
                }
            }
        }

        for s in &self.series {
            for &x in &s.x {
                if x < x_min { x_min = x; }
                if x > x_max { x_max = x; }
            }
            if !self.stacked {
                for &y in &s.y {
                    if y > y_max { y_max = y; }
                }
            }
        }

        if x_min == f64::MAX { x_min = 0.0; x_max = 1.0; }
        if y_max == f64::MIN { y_max = 1.0; }

        y_max *= 1.1; // Add 10% padding
        (x_min, x_max, y_min, y_max)
    }
}

impl Widget for AreaChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.series.is_empty() {
            let padding_left = 60.0;
            let padding_right = 20.0;
            let padding_top = 40.0;
            let padding_bottom = 50.0;

            let plot_left = rect.pos.x + padding_left;
            let plot_top = rect.pos.y + padding_top;
            let plot_right = rect.pos.x + rect.size.x - padding_right;
            let plot_bottom = rect.pos.y + rect.size.y - padding_bottom;
            let plot_width = plot_right - plot_left;
            let plot_height = plot_bottom - plot_top;

            let (x_min, x_max, y_min, y_max) = self.get_bounds();
            let x_range = (x_max - x_min).max(0.001);
            let y_range = (y_max - y_min).max(0.001);

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

            // Draw axis labels
            self.label.draw_text.color = vec4(0.3, 0.3, 0.3, 1.0);
            for i in 0..=5 {
                let t = i as f64 / 5.0;
                let x_val = x_min + t * x_range;
                let y_val = y_min + (1.0 - t) * y_range;
                let x = plot_left + t * plot_width;
                let y = plot_top + t * plot_height;
                self.label.draw_at(cx, dvec2(x, plot_bottom + 15.0), &format!("{:.0}", x_val), TextAnchor::TopCenter);
                self.label.draw_at(cx, dvec2(plot_left - 10.0, y), &format!("{:.0}", y_val), TextAnchor::MiddleRight);
            }

            // Draw areas (from back to front for stacked)
            let mut cumulative: Vec<f64> = vec![0.0; self.series.first().map(|s| s.x.len()).unwrap_or(0)];

            for (_idx, series) in self.series.iter().enumerate() {
                if series.x.len() < 2 { continue; }

                // Create gradient colors: lighter at top, base color at bottom
                let top_color = vec4(
                    (series.color.x * 1.2).min(1.0),
                    (series.color.y * 1.2).min(1.0),
                    (series.color.z * 1.2).min(1.0),
                    series.color.w * 0.3
                );
                let bottom_color = vec4(
                    series.color.x,
                    series.color.y,
                    series.color.z,
                    series.color.w * 0.7
                );

                // Draw filled area using vertical strips with gradient
                let n = series.x.len();
                let subdivisions = 4; // Subdivide each segment for smoother curves
                for i in 0..n.saturating_sub(1) {
                    let x1 = series.x[i];
                    let x2 = series.x[i + 1];
                    let y1 = if self.stacked { series.y[i] + cumulative[i] } else { series.y[i] };
                    let y2 = if self.stacked { series.y[i + 1] + cumulative.get(i + 1).copied().unwrap_or(0.0) } else { series.y[i + 1] };
                    let base1 = if self.stacked { cumulative[i] } else { y_min };
                    let base2 = if self.stacked { cumulative.get(i + 1).copied().unwrap_or(0.0) } else { y_min };

                    for s in 0..subdivisions {
                        let t1 = s as f64 / subdivisions as f64;
                        let t2 = (s + 1) as f64 / subdivisions as f64;

                        let sx1 = x1 + (x2 - x1) * t1;
                        let sx2 = x1 + (x2 - x1) * t2;
                        let sy1 = y1 + (y2 - y1) * t1;
                        let sy2 = y1 + (y2 - y1) * t2;
                        let sb1 = base1 + (base2 - base1) * t1;
                        let sb2 = base1 + (base2 - base1) * t2;

                        let px1 = plot_left + ((sx1 - x_min) / x_range) * plot_width;
                        let px2 = plot_left + ((sx2 - x_min) / x_range) * plot_width;
                        let py1 = plot_bottom - ((sy1 - y_min) / y_range) * plot_height;
                        let py2 = plot_bottom - ((sy2 - y_min) / y_range) * plot_height;
                        let pby1 = plot_bottom - ((sb1 - y_min) / y_range) * plot_height;
                        let pby2 = plot_bottom - ((sb2 - y_min) / y_range) * plot_height;

                        // Draw as filled rectangle with gradient
                        let strip_width = (px2 - px1).max(1.0);
                        let top_y = py1.min(py2);
                        let bottom_y = pby1.max(pby2);

                        self.draw_fill.draw_fill_strip_gradient(cx, px1, strip_width, top_y, bottom_y, bottom_color, top_color);
                    }
                }

                // Draw top line with solid color
                self.draw_line.color = series.color;
                for i in 0..n.saturating_sub(1) {
                    let x1 = series.x[i];
                    let x2 = series.x[i + 1];
                    let y1 = if self.stacked { series.y[i] + cumulative[i] } else { series.y[i] };
                    let y2 = if self.stacked { series.y[i + 1] + cumulative.get(i + 1).copied().unwrap_or(0.0) } else { series.y[i + 1] };

                    let px1 = plot_left + ((x1 - x_min) / x_range) * plot_width;
                    let px2 = plot_left + ((x2 - x_min) / x_range) * plot_width;
                    let py1 = plot_bottom - ((y1 - y_min) / y_range) * plot_height;
                    let py2 = plot_bottom - ((y2 - y_min) / y_range) * plot_height;
                    self.draw_line.draw_line(cx, dvec2(px1, py1), dvec2(px2, py2), 2.0);
                }

                // Update cumulative for stacked
                if self.stacked {
                    for i in 0..cumulative.len().min(series.y.len()) {
                        cumulative[i] += series.y[i];
                    }
                }
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_text.color = vec4(0.2, 0.2, 0.2, 1.0);
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 15.0), &self.title, TextAnchor::TopCenter);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl AreaChart {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }
}

impl AreaChartRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn add_series(&self, series: AreaSeries) {
        if let Some(mut inner) = self.borrow_mut() { inner.add_series(series); }
    }
    pub fn set_x_label(&self, label: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_x_label(label); }
    }
    pub fn set_y_label(&self, label: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_y_label(label); }
    }
    pub fn set_stacked(&self, stacked: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_stacked(stacked); }
    }
    pub fn set_show_grid(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_grid(show); }
    }
    pub fn clear(&self) {
        if let Some(mut inner) = self.borrow_mut() { inner.clear(); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}

// ============================================================================
// StepPlot Widget - Discrete step-wise line visualization
// ============================================================================

#[derive(Clone)]
pub struct StepSeries {
    pub name: String,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub color: Vec4,
    pub style: StepStyle,
}

impl StepSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            x: Vec::new(),
            y: Vec::new(),
            color: get_color(0),
            style: StepStyle::Pre,
        }
    }

    pub fn with_data(mut self, x: Vec<f64>, y: Vec<f64>) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn with_style(mut self, style: StepStyle) -> Self {
        self.style = style;
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct StepPlot {
    #[deref] #[live] view: View,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] series: Vec<StepSeries>,
    #[rust] x_label: String,
    #[rust] y_label: String,
    #[rust] show_grid: bool,
    #[rust] show_markers: bool,
}

impl StepPlot {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn add_series(&mut self, series: StepSeries) {
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

    pub fn set_show_markers(&mut self, show: bool) {
        self.show_markers = show;
    }

    pub fn clear(&mut self) {
        self.series.clear();
    }

    fn get_bounds(&self) -> (f64, f64, f64, f64) {
        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;

        for s in &self.series {
            for &x in &s.x {
                if x < x_min { x_min = x; }
                if x > x_max { x_max = x; }
            }
            for &y in &s.y {
                if y < y_min { y_min = y; }
                if y > y_max { y_max = y; }
            }
        }

        if x_min == f64::MAX { x_min = 0.0; x_max = 1.0; }
        if y_min == f64::MAX { y_min = 0.0; y_max = 1.0; }

        let x_range = (x_max - x_min).max(0.001);
        let y_range = (y_max - y_min).max(0.001);
        x_min -= x_range * 0.05;
        x_max += x_range * 0.05;
        y_min -= y_range * 0.1;
        y_max += y_range * 0.1;

        (x_min, x_max, y_min, y_max)
    }
}

impl Widget for StepPlot {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.series.is_empty() {
            let padding_left = 60.0;
            let padding_right = 20.0;
            let padding_top = 40.0;
            let padding_bottom = 50.0;

            let plot_left = rect.pos.x + padding_left;
            let plot_top = rect.pos.y + padding_top;
            let plot_right = rect.pos.x + rect.size.x - padding_right;
            let plot_bottom = rect.pos.y + rect.size.y - padding_bottom;
            let plot_width = plot_right - plot_left;
            let plot_height = plot_bottom - plot_top;

            let (x_min, x_max, y_min, y_max) = self.get_bounds();
            let x_range = (x_max - x_min).max(0.001);
            let y_range = (y_max - y_min).max(0.001);

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

            // Draw axis labels
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

            // Draw step lines
            for series in &self.series {
                if series.x.len() < 2 { continue; }

                self.draw_line.color = series.color;
                let n = series.x.len();

                for i in 0..n.saturating_sub(1) {
                    let x1 = series.x[i];
                    let x2 = series.x[i + 1];
                    let y1 = series.y[i];
                    let y2 = series.y[i + 1];

                    let px1 = plot_left + ((x1 - x_min) / x_range) * plot_width;
                    let px2 = plot_left + ((x2 - x_min) / x_range) * plot_width;
                    let py1 = plot_bottom - ((y1 - y_min) / y_range) * plot_height;
                    let py2 = plot_bottom - ((y2 - y_min) / y_range) * plot_height;

                    match series.style {
                        StepStyle::None => {
                            // Normal direct line
                            self.draw_line.draw_line(cx, dvec2(px1, py1), dvec2(px2, py2), 2.0);
                        }
                        StepStyle::Pre => {
                            // Vertical then horizontal
                            self.draw_line.draw_line(cx, dvec2(px1, py1), dvec2(px1, py2), 2.0);
                            self.draw_line.draw_line(cx, dvec2(px1, py2), dvec2(px2, py2), 2.0);
                        }
                        StepStyle::Post => {
                            // Horizontal then vertical
                            self.draw_line.draw_line(cx, dvec2(px1, py1), dvec2(px2, py1), 2.0);
                            self.draw_line.draw_line(cx, dvec2(px2, py1), dvec2(px2, py2), 2.0);
                        }
                        StepStyle::Mid => {
                            // Horizontal, vertical at midpoint, horizontal
                            let mid_x = (px1 + px2) / 2.0;
                            self.draw_line.draw_line(cx, dvec2(px1, py1), dvec2(mid_x, py1), 2.0);
                            self.draw_line.draw_line(cx, dvec2(mid_x, py1), dvec2(mid_x, py2), 2.0);
                            self.draw_line.draw_line(cx, dvec2(mid_x, py2), dvec2(px2, py2), 2.0);
                        }
                    }
                }

                // Draw markers
                if self.show_markers {
                    for i in 0..n {
                        let px = plot_left + ((series.x[i] - x_min) / x_range) * plot_width;
                        let py = plot_bottom - ((series.y[i] - y_min) / y_range) * plot_height;

                        // Draw small circle
                        let segments = 12;
                        let radius = 4.0;
                        for j in 0..segments {
                            let a1 = (j as f64 / segments as f64) * 2.0 * std::f64::consts::PI;
                            let a2 = ((j + 1) as f64 / segments as f64) * 2.0 * std::f64::consts::PI;
                            let x1 = px + radius * a1.cos();
                            let y1 = py + radius * a1.sin();
                            let x2 = px + radius * a2.cos();
                            let y2 = py + radius * a2.sin();
                            self.draw_line.draw_line(cx, dvec2(x1, y1), dvec2(x2, y2), 2.0);
                        }
                    }
                }
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_text.color = vec4(0.2, 0.2, 0.2, 1.0);
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 15.0), &self.title, TextAnchor::TopCenter);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl StepPlot {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }
}

impl StepPlotRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn add_series(&self, series: StepSeries) {
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
    pub fn set_show_markers(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_markers(show); }
    }
    pub fn clear(&self) {
        if let Some(mut inner) = self.borrow_mut() { inner.clear(); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}

