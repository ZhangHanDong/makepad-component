use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub Stackplot = {{Stackplot}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    pub Streamgraph = {{Streamgraph}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

/// Stack ordering method
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackOrder {
    /// No reordering, maintain original series order
    #[default]
    None,
    /// Sort by sum of values ascending
    Ascending,
    /// Sort by sum of values descending
    Descending,
    /// Sort so smallest series are in the middle
    InsideOut,
    /// Reverse the current order
    Reverse,
}

/// Stack offset method
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackOffset {
    /// No offset, stack from zero
    #[default]
    None,
    /// Normalize to fill [0, 1] range
    Expand,
    /// Center around zero (diverging stacks)
    Diverging,
    /// Center the baseline (silhouette)
    Silhouette,
    /// Streamgraph wiggle minimization
    Wiggle,
}

/// A single series for the stackplot
#[derive(Clone, Debug)]
pub struct StackSeries {
    pub label: String,
    pub values: Vec<f64>,
    pub color: Option<Vec4>,
}

impl StackSeries {
    pub fn new(label: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            label: label.into(),
            values,
            color: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

/// Stacked point with y0 (bottom) and y1 (top) bounds
#[derive(Clone, Debug)]
pub struct StackedPoint {
    pub y0: f64,
    pub y1: f64,
}

impl StackedPoint {
    pub fn new(y0: f64, y1: f64) -> Self {
        Self { y0, y1 }
    }

    pub fn height(&self) -> f64 {
        self.y1 - self.y0
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Stackplot {
    #[redraw] #[live] draw_bg: DrawQuad,
    #[redraw] #[live] draw_triangle: DrawTriangle,
    #[redraw] #[live] draw_line: DrawPlotLine,
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    #[live] label: PlotLabel,

    #[rust] series: Vec<StackSeries>,
    #[rust] x_labels: Vec<String>,
    #[rust] title: String,
    #[rust] order: StackOrder,
    #[rust] offset: StackOffset,
    #[rust] show_lines: bool,
    #[rust] area: Area,
}

impl Stackplot {
    pub fn set_data(&mut self, series: Vec<StackSeries>, x_labels: Vec<String>) {
        self.series = series;
        self.x_labels = x_labels;
    }

    pub fn add_series(&mut self, series: StackSeries) {
        self.series.push(series);
    }

    pub fn set_x_labels(&mut self, labels: Vec<String>) {
        self.x_labels = labels;
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_order(&mut self, order: StackOrder) {
        self.order = order;
    }

    pub fn set_offset(&mut self, offset: StackOffset) {
        self.offset = offset;
    }

    pub fn set_show_lines(&mut self, show: bool) {
        self.show_lines = show;
    }

    fn compute_stacked(&self) -> Vec<Vec<StackedPoint>> {
        let n_series = self.series.len();
        if n_series == 0 { return vec![]; }

        let n_points = self.series.iter().map(|s| s.values.len()).max().unwrap_or(0);
        if n_points == 0 { return vec![]; }

        // Initialize result
        let mut result: Vec<Vec<StackedPoint>> = self.series.iter()
            .map(|_| vec![StackedPoint::new(0.0, 0.0); n_points])
            .collect();

        // Compute order
        let order = self.compute_order();

        // Stack values
        for i in 0..n_points {
            let mut y0 = 0.0;
            for &series_idx in &order {
                let y = self.series[series_idx].values.get(i).copied().unwrap_or(0.0);
                result[series_idx][i] = StackedPoint::new(y0, y0 + y);
                y0 += y;
            }
        }

        // Apply offset
        self.apply_offset(&mut result, n_points);

        result
    }

    fn compute_order(&self) -> Vec<usize> {
        let n = self.series.len();
        let mut indices: Vec<usize> = (0..n).collect();

        match self.order {
            StackOrder::None => {}
            StackOrder::Ascending => {
                let sums: Vec<f64> = self.series.iter().map(|s| s.values.iter().sum()).collect();
                indices.sort_by(|&a, &b| sums[a].partial_cmp(&sums[b]).unwrap_or(std::cmp::Ordering::Equal));
            }
            StackOrder::Descending => {
                let sums: Vec<f64> = self.series.iter().map(|s| s.values.iter().sum()).collect();
                indices.sort_by(|&a, &b| sums[b].partial_cmp(&sums[a]).unwrap_or(std::cmp::Ordering::Equal));
            }
            StackOrder::InsideOut => {
                let sums: Vec<f64> = self.series.iter().map(|s| s.values.iter().sum()).collect();
                indices.sort_by(|&a, &b| sums[b].partial_cmp(&sums[a]).unwrap_or(std::cmp::Ordering::Equal));

                let mut new_order = Vec::with_capacity(n);
                let mut top = true;
                for idx in indices {
                    if top { new_order.push(idx); } else { new_order.insert(0, idx); }
                    top = !top;
                }
                indices = new_order;
            }
            StackOrder::Reverse => {
                indices.reverse();
            }
        }

        indices
    }

    fn apply_offset(&self, result: &mut Vec<Vec<StackedPoint>>, n_points: usize) {
        match self.offset {
            StackOffset::None => {}
            StackOffset::Expand => {
                for i in 0..n_points {
                    let total: f64 = result.iter().map(|s| s[i].height()).sum();
                    if total > 0.0 {
                        for s in result.iter_mut() {
                            s[i].y0 /= total;
                            s[i].y1 /= total;
                        }
                    }
                }
            }
            StackOffset::Diverging | StackOffset::Silhouette => {
                for i in 0..n_points {
                    let max_y1 = result.iter().map(|s| s[i].y1).fold(0.0_f64, f64::max);
                    let offset = -max_y1 / 2.0;
                    for s in result.iter_mut() {
                        s[i].y0 += offset;
                        s[i].y1 += offset;
                    }
                }
            }
            StackOffset::Wiggle => {
                if result.is_empty() || n_points == 0 { return; }
                let n = result.len();
                for i in 0..n_points {
                    let mut sum = 0.0;
                    let mut total_weight = 0.0;
                    for (j, s) in result.iter().enumerate() {
                        let height = s[i].height();
                        let weight = (n - j) as f64;
                        sum += weight * height;
                        total_weight += weight;
                    }
                    let total: f64 = result.iter().map(|s| s[i].height()).sum();
                    let offset = if total_weight > 0.0 && total > 0.0 {
                        -sum / (total_weight * 2.0)
                    } else { 0.0 };
                    for s in result.iter_mut() {
                        s[i].y0 += offset;
                        s[i].y1 += offset;
                    }
                }
            }
        }
    }
}

impl Widget for Stackplot {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 10.0 && rect.size.y > 10.0 {
            let padding = 30.0;
            let chart_x = rect.pos.x + padding;
            let chart_y = rect.pos.y + padding;
            let chart_w = rect.size.x - padding * 2.0;
            let chart_h = rect.size.y - padding * 2.0;

            if self.series.is_empty() || chart_w < 10.0 || chart_h < 10.0 {
                return DrawStep::done();
            }

            // Compute stacked data
            let stacked = self.compute_stacked();
            let n_points = stacked[0].len();
            if n_points == 0 { return DrawStep::done(); }

            // Find y range
            let mut y_min = f64::MAX;
            let mut y_max = f64::MIN;
            for series in &stacked {
                for pt in series {
                    y_min = y_min.min(pt.y0);
                    y_max = y_max.max(pt.y1);
                }
            }
            if (y_max - y_min).abs() < 0.001 { y_max = y_min + 1.0; }

            // Color palette
            let colors = [
                vec4(0.40, 0.76, 0.65, 0.85),
                vec4(0.99, 0.55, 0.38, 0.85),
                vec4(0.55, 0.63, 0.80, 0.85),
                vec4(0.91, 0.84, 0.42, 0.85),
                vec4(0.65, 0.85, 0.33, 0.85),
                vec4(0.90, 0.45, 0.77, 0.85),
                vec4(0.45, 0.85, 0.90, 0.85),
                vec4(0.85, 0.65, 0.45, 0.85),
            ];

            // Draw stacked areas
            for (series_idx, series_data) in stacked.iter().enumerate() {
                let color = self.series[series_idx].color.unwrap_or(colors[series_idx % colors.len()]);
                self.draw_triangle.color = color;

                for i in 0..n_points.saturating_sub(1) {
                    let x1 = chart_x + (i as f64 / (n_points - 1).max(1) as f64) * chart_w;
                    let x2 = chart_x + ((i + 1) as f64 / (n_points - 1).max(1) as f64) * chart_w;

                    let y1_bottom = chart_y + chart_h - ((series_data[i].y0 - y_min) / (y_max - y_min)) * chart_h;
                    let y1_top = chart_y + chart_h - ((series_data[i].y1 - y_min) / (y_max - y_min)) * chart_h;
                    let y2_bottom = chart_y + chart_h - ((series_data[i + 1].y0 - y_min) / (y_max - y_min)) * chart_h;
                    let y2_top = chart_y + chart_h - ((series_data[i + 1].y1 - y_min) / (y_max - y_min)) * chart_h;

                    // Draw two triangles per segment
                    self.draw_triangle.draw_triangle(cx, dvec2(x1, y1_bottom), dvec2(x2, y2_bottom), dvec2(x1, y1_top));
                    self.draw_triangle.draw_triangle(cx, dvec2(x1, y1_top), dvec2(x2, y2_bottom), dvec2(x2, y2_top));
                }

                // Draw top line
                if self.show_lines {
                    self.draw_line.color = darken(color, 0.3);
                    for i in 0..n_points.saturating_sub(1) {
                        let x1 = chart_x + (i as f64 / (n_points - 1).max(1) as f64) * chart_w;
                        let x2 = chart_x + ((i + 1) as f64 / (n_points - 1).max(1) as f64) * chart_w;
                        let y1 = chart_y + chart_h - ((series_data[i].y1 - y_min) / (y_max - y_min)) * chart_h;
                        let y2 = chart_y + chart_h - ((series_data[i + 1].y1 - y_min) / (y_max - y_min)) * chart_h;
                        self.draw_line.draw_line(cx, dvec2(x1, y1), dvec2(x2, y2), 1.5);
                    }
                }
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 15.0), &self.title, TextAnchor::TopCenter);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
}

impl StackplotRef {
    pub fn set_data(&self, series: Vec<StackSeries>, x_labels: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(series, x_labels); }
    }
    pub fn add_series(&self, series: StackSeries) {
        if let Some(mut inner) = self.borrow_mut() { inner.add_series(series); }
    }
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn set_order(&self, order: StackOrder) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_order(order); }
    }
    pub fn set_offset(&self, offset: StackOffset) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_offset(offset); }
    }
}


// =============================================================================
// Streamgraph Widget
// =============================================================================

pub struct StreamSeries {
    pub name: String,
    pub values: Vec<f64>,
    pub color: Option<Vec4>,
}

impl StreamSeries {
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            values,
            color: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Streamgraph {
    #[redraw] #[live] draw_bg: DrawQuad,
    #[redraw] #[live] draw_triangle: DrawTriangle,
    #[redraw] #[live] draw_line: DrawPlotLine,
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    #[live] label: PlotLabel,

    #[rust] series: Vec<StreamSeries>,
    #[rust] labels: Vec<String>,
    #[rust] title: String,
    #[rust] area: Area,
}

impl Streamgraph {
    pub fn set_data(&mut self, series: Vec<StreamSeries>, labels: Vec<String>) {
        self.series = series;
        self.labels = labels;
    }

    pub fn add_series(&mut self, series: StreamSeries) {
        self.series.push(series);
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }
}

impl Widget for Streamgraph {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 10.0 && rect.size.y > 10.0 && !self.series.is_empty() {
            let padding = 30.0;
            let chart_x = rect.pos.x + padding;
            let chart_y = rect.pos.y + padding;
            let chart_w = rect.size.x - padding * 2.0;
            let chart_h = rect.size.y - padding * 2.0;

            let n_points = self.series.iter().map(|s| s.values.len()).max().unwrap_or(0);
            if n_points == 0 { return DrawStep::done(); }

            // Calculate totals
            let mut totals: Vec<f64> = vec![0.0; n_points];
            for s in &self.series {
                for (i, &val) in s.values.iter().enumerate() {
                    if i < totals.len() { totals[i] += val; }
                }
            }

            let max_total = totals.iter().cloned().fold(0.0_f64, f64::max);
            if max_total == 0.0 { return DrawStep::done(); }

            // Calculate baselines for centering (silhouette offset)
            let baselines: Vec<f64> = totals.iter().map(|&t| (max_total - t) / 2.0).collect();

            let colors = [
                vec4(0.40, 0.76, 0.65, 0.85),
                vec4(0.99, 0.55, 0.38, 0.85),
                vec4(0.55, 0.63, 0.80, 0.85),
                vec4(0.91, 0.84, 0.42, 0.85),
                vec4(0.65, 0.85, 0.33, 0.85),
                vec4(0.90, 0.45, 0.77, 0.85),
            ];

            let mut cumulative = baselines.clone();

            for (series_idx, s) in self.series.iter().enumerate() {
                let color = s.color.unwrap_or(colors[series_idx % colors.len()]);

                let mut bottom_points: Vec<DVec2> = Vec::new();
                let mut top_points: Vec<DVec2> = Vec::new();

                for i in 0..n_points {
                    let x = chart_x + (i as f64 / (n_points - 1).max(1) as f64) * chart_w;
                    let val = s.values.get(i).copied().unwrap_or(0.0);

                    let bottom_y = chart_y + chart_h - (cumulative[i] / max_total) * chart_h;
                    let top_y = chart_y + chart_h - ((cumulative[i] + val) / max_total) * chart_h;

                    bottom_points.push(DVec2 { x, y: bottom_y });
                    top_points.push(DVec2 { x, y: top_y });

                    cumulative[i] += val;
                }

                // Draw stream area
                self.draw_triangle.color = color;
                for i in 0..n_points.saturating_sub(1) {
                    let b1 = bottom_points[i];
                    let b2 = bottom_points[i + 1];
                    let t1 = top_points[i];
                    let t2 = top_points[i + 1];

                    self.draw_triangle.draw_triangle(cx, b1, b2, t1);
                    self.draw_triangle.draw_triangle(cx, t1, b2, t2);
                }
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 15.0), &self.title, TextAnchor::TopCenter);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
}

impl StreamgraphRef {
    pub fn set_data(&self, series: Vec<StreamSeries>, labels: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(series, labels); }
    }
    pub fn add_series(&self, series: StreamSeries) {
        if let Some(mut inner) = self.borrow_mut() { inner.add_series(series); }
    }
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
}

