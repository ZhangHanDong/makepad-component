use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub HistogramChart = {{HistogramChart}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    pub BoxPlotChart = {{BoxPlotChart}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

/// Histogram bin
#[derive(Clone, Debug)]
pub struct HistogramBin {
    pub left: f64,
    pub right: f64,
    pub count: usize,
}

#[derive(Live, LiveHook, Widget)]
pub struct HistogramChart {
    #[deref]
    #[live]
    view: View,

    #[live]
    draw_bar: DrawPlotBar,

    #[live]
    draw_line: DrawPlotLine,

    #[live]
    label: PlotLabel,

    #[rust]
    values: Vec<f64>,

    #[rust]
    bins: Vec<HistogramBin>,

    #[rust]
    num_bins: Option<usize>,

    #[rust]
    plot_area: PlotArea,

    #[rust]
    title: String,

    #[rust]
    x_label: String,

    #[rust]
    y_label: String,

    #[rust(true)]
    show_grid: bool,

    #[rust]
    bar_color: Option<Vec4>,

    #[rust(50.0)]
    left_margin: f64,

    #[rust(30.0)]
    bottom_margin: f64,

    #[rust(20.0)]
    right_margin: f64,

    #[rust(30.0)]
    top_margin: f64,
}

impl Widget for HistogramChart {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.bins.is_empty() {
            self.update_plot_area(rect);
            self.draw_grid(cx);
            self.draw_axes(cx);
            self.draw_bars(cx);
            self.draw_labels(cx);
        }

        DrawStep::done()
    }
}

impl HistogramChart {
    pub fn set_values(&mut self, values: Vec<f64>) {
        self.values = values;
        self.compute_bins();
    }

    pub fn set_num_bins(&mut self, num_bins: usize) {
        self.num_bins = Some(num_bins);
        if !self.values.is_empty() {
            self.compute_bins();
        }
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

    pub fn set_color(&mut self, color: Vec4) {
        self.bar_color = Some(color);
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.bins.clear();
    }

    fn compute_bins(&mut self) {
        if self.values.is_empty() {
            self.bins.clear();
            return;
        }

        let min = self.values.iter().cloned().fold(f64::MAX, f64::min);
        let max = self.values.iter().cloned().fold(f64::MIN, f64::max);

        let num_bins = self.num_bins.unwrap_or_else(|| {
            let n = self.values.len() as f64;
            (1.0 + 3.322 * n.log10()).ceil() as usize
        }).max(1);

        let bin_width = (max - min) / num_bins as f64;

        self.bins = (0..num_bins).map(|i| {
            let left = min + i as f64 * bin_width;
            let right = if i == num_bins - 1 { max } else { min + (i + 1) as f64 * bin_width };
            HistogramBin { left, right, count: 0 }
        }).collect();

        for &value in &self.values {
            let bin_idx = ((value - min) / bin_width).floor() as usize;
            let bin_idx = bin_idx.min(num_bins - 1);
            self.bins[bin_idx].count += 1;
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

    fn get_ranges(&self) -> ((f64, f64), (f64, f64)) {
        if self.bins.is_empty() {
            return ((0.0, 1.0), (0.0, 1.0));
        }

        let x_min = self.bins.first().map(|b| b.left).unwrap_or(0.0);
        let x_max = self.bins.last().map(|b| b.right).unwrap_or(1.0);
        let y_max = self.bins.iter().map(|b| b.count).max().unwrap_or(1) as f64 * 1.1;

        ((x_min, x_max), (0.0, y_max))
    }

    fn data_to_pixel(&self, x: f64, y: f64) -> DVec2 {
        let ((x_min, x_max), (y_min, y_max)) = self.get_ranges();
        let px = self.plot_area.left + (x - x_min) / (x_max - x_min) * self.plot_area.width();
        let py = self.plot_area.bottom - (y - y_min) / (y_max - y_min) * self.plot_area.height();
        dvec2(px, py)
    }

    fn draw_grid(&mut self, cx: &mut Cx2d) {
        if !self.show_grid {
            return;
        }

        self.draw_line.color = vec4(0.9, 0.9, 0.9, 1.0);
        let (_, (y_min, y_max)) = self.get_ranges();

        let y_ticks = self.generate_ticks(y_min, y_max, 5);
        for y in &y_ticks {
            let p1 = self.data_to_pixel(self.bins.first().map(|b| b.left).unwrap_or(0.0), *y);
            let p2 = self.data_to_pixel(self.bins.last().map(|b| b.right).unwrap_or(1.0), *y);
            self.draw_line.draw_line(cx, p1, p2, 0.5);
        }
    }

    fn draw_axes(&mut self, cx: &mut Cx2d) {
        self.draw_line.color = vec4(0.3, 0.3, 0.3, 1.0);

        let x1 = dvec2(self.plot_area.left, self.plot_area.bottom);
        let x2 = dvec2(self.plot_area.right, self.plot_area.bottom);
        self.draw_line.draw_line(cx, x1, x2, 1.0);

        let y1 = dvec2(self.plot_area.left, self.plot_area.bottom);
        let y2 = dvec2(self.plot_area.left, self.plot_area.top);
        self.draw_line.draw_line(cx, y1, y2, 1.0);
    }

    fn draw_bars(&mut self, cx: &mut Cx2d) {
        let color = self.bar_color.unwrap_or_else(|| get_color(0));
        self.draw_bar.color = color;

        for bin in &self.bins {
            let p1 = self.data_to_pixel(bin.left, 0.0);
            let p2 = self.data_to_pixel(bin.right, bin.count as f64);

            let rect = Rect {
                pos: dvec2(p1.x, p2.y),
                size: dvec2(p2.x - p1.x - 1.0, p1.y - p2.y),
            };
            self.draw_bar.draw_bar(cx, rect);
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));

        let ((x_min, x_max), (y_min, y_max)) = self.get_ranges();

        let x_ticks = self.generate_ticks(x_min, x_max, 5);
        for x in &x_ticks {
            let p = self.data_to_pixel(*x, y_min);
            let label = format!("{:.1}", x);
            self.label.draw_at(cx, dvec2(p.x, p.y + 5.0), &label, TextAnchor::TopCenter);
        }

        let y_ticks = self.generate_ticks(y_min, y_max, 5);
        for y in &y_ticks {
            let p = self.data_to_pixel(x_min, *y);
            let label = format!("{:.0}", y);
            self.label.draw_at(cx, dvec2(p.x - 5.0, p.y), &label, TextAnchor::MiddleRight);
        }

        if !self.title.is_empty() {
            let center_x = (self.plot_area.left + self.plot_area.right) / 2.0;
            self.label.draw_at(cx, dvec2(center_x, self.plot_area.top - 10.0), &self.title, TextAnchor::BottomCenter);
        }
    }

    fn generate_ticks(&self, min: f64, max: f64, count: usize) -> Vec<f64> {
        let step = (max - min) / count as f64;
        (0..=count).map(|i| min + i as f64 * step).collect()
    }
}

impl HistogramChartRef {
    pub fn set_values(&self, values: Vec<f64>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_values(values);
        }
    }

    pub fn set_num_bins(&self, num_bins: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_num_bins(num_bins);
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

    pub fn set_color(&self, color: Vec4) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_color(color);
        }
    }

    pub fn clear(&self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear();
        }
    }

    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.redraw(cx);
        }
    }
}

// =============================================================================
// BoxPlotChart Widget
// =============================================================================

/// Box plot statistics
#[derive(Clone, Debug, Default)]
pub struct BoxPlotStats {
    pub min: f64,
    pub q1: f64,
    pub median: f64,
    pub q3: f64,
    pub max: f64,
    pub outliers: Vec<f64>,
}

impl BoxPlotStats {
    pub fn from_values(values: &[f64]) -> Option<Self> {
        if values.is_empty() {
            return None;
        }

        let mut sorted: Vec<f64> = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let n = sorted.len();
        let median = if n % 2 == 0 {
            (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
        } else {
            sorted[n / 2]
        };

        let q1_idx = n / 4;
        let q3_idx = 3 * n / 4;
        let q1 = sorted[q1_idx];
        let q3 = sorted[q3_idx];

        let iqr = q3 - q1;
        let lower_fence = q1 - 1.5 * iqr;
        let upper_fence = q3 + 1.5 * iqr;

        let outliers: Vec<f64> = sorted.iter()
            .filter(|&&v| v < lower_fence || v > upper_fence)
            .cloned()
            .collect();

        let whisker_min = sorted.iter().find(|&&v| v >= lower_fence).cloned().unwrap_or(q1);
        let whisker_max = sorted.iter().rev().find(|&&v| v <= upper_fence).cloned().unwrap_or(q3);

        Some(BoxPlotStats {
            min: whisker_min,
            q1,
            median,
            q3,
            max: whisker_max,
            outliers,
        })
    }
}

/// Box plot data item
#[derive(Clone, Debug)]
pub struct BoxPlotItem {
    pub label: String,
    pub stats: BoxPlotStats,
    pub color: Option<Vec4>,
}

impl BoxPlotItem {
    pub fn new(label: impl Into<String>, values: &[f64]) -> Option<Self> {
        BoxPlotStats::from_values(values).map(|stats| BoxPlotItem {
            label: label.into(),
            stats,
            color: None,
        })
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BoxPlotChart {
    #[deref]
    #[live]
    view: View,

    #[live]
    draw_bar: DrawPlotBar,

    #[live]
    draw_line: DrawPlotLine,

    #[live]
    draw_point: DrawPlotPoint,

    #[live]
    label: PlotLabel,

    #[rust]
    items: Vec<BoxPlotItem>,

    #[rust]
    plot_area: PlotArea,

    #[rust]
    title: String,

    #[rust(true)]
    show_grid: bool,

    #[rust(true)]
    show_outliers: bool,

    #[rust(50.0)]
    left_margin: f64,

    #[rust(40.0)]
    bottom_margin: f64,

    #[rust(20.0)]
    right_margin: f64,

    #[rust(30.0)]
    top_margin: f64,

    #[rust(0.6)]
    box_width_ratio: f64,
}

impl Widget for BoxPlotChart {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.items.is_empty() {
            self.update_plot_area(rect);
            self.draw_grid(cx);
            self.draw_axes(cx);
            self.draw_boxes(cx);
            self.draw_labels(cx);
        }

        DrawStep::done()
    }
}

impl BoxPlotChart {
    pub fn add_item(&mut self, item: BoxPlotItem) {
        self.items.push(item);
    }

    pub fn add_from_values(&mut self, label: impl Into<String>, values: &[f64]) {
        if let Some(item) = BoxPlotItem::new(label, values) {
            self.items.push(item);
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_show_outliers(&mut self, show: bool) {
        self.show_outliers = show;
    }

    fn update_plot_area(&mut self, rect: Rect) {
        self.plot_area = PlotArea::new(
            rect.pos.x + self.left_margin,
            rect.pos.y + self.top_margin,
            rect.pos.x + rect.size.x - self.right_margin,
            rect.pos.y + rect.size.y - self.bottom_margin,
        );
    }

    fn get_y_range(&self) -> (f64, f64) {
        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for item in &self.items {
            min = min.min(item.stats.min);
            max = max.max(item.stats.max);
            if self.show_outliers {
                for &outlier in &item.stats.outliers {
                    min = min.min(outlier);
                    max = max.max(outlier);
                }
            }
        }

        let padding = (max - min) * 0.1;
        (min - padding, max + padding)
    }

    fn draw_grid(&mut self, cx: &mut Cx2d) {
        if !self.show_grid {
            return;
        }

        self.draw_line.color = vec4(0.9, 0.9, 0.9, 1.0);
        let (y_min, y_max) = self.get_y_range();

        let y_ticks = self.generate_ticks(y_min, y_max, 5);
        for y in &y_ticks {
            let y_pixel = self.plot_area.bottom - (*y - y_min) / (y_max - y_min) * self.plot_area.height();
            let p1 = dvec2(self.plot_area.left, y_pixel);
            let p2 = dvec2(self.plot_area.right, y_pixel);
            self.draw_line.draw_line(cx, p1, p2, 0.5);
        }
    }

    fn draw_axes(&mut self, cx: &mut Cx2d) {
        self.draw_line.color = vec4(0.3, 0.3, 0.3, 1.0);

        let x1 = dvec2(self.plot_area.left, self.plot_area.bottom);
        let x2 = dvec2(self.plot_area.right, self.plot_area.bottom);
        self.draw_line.draw_line(cx, x1, x2, 1.0);

        let y1 = dvec2(self.plot_area.left, self.plot_area.bottom);
        let y2 = dvec2(self.plot_area.left, self.plot_area.top);
        self.draw_line.draw_line(cx, y1, y2, 1.0);
    }

    fn draw_boxes(&mut self, cx: &mut Cx2d) {
        let n = self.items.len();
        if n == 0 {
            return;
        }

        let (y_min, y_max) = self.get_y_range();
        let band_width = self.plot_area.width() / n as f64;
        let box_width = band_width * self.box_width_ratio;

        for (i, item) in self.items.iter().enumerate() {
            let color = item.color.unwrap_or_else(|| get_color(i));
            let x_center = self.plot_area.left + (i as f64 + 0.5) * band_width;

            let y_to_pixel = |y: f64| -> f64 {
                self.plot_area.bottom - (y - y_min) / (y_max - y_min) * self.plot_area.height()
            };

            let q1_y = y_to_pixel(item.stats.q1);
            let q3_y = y_to_pixel(item.stats.q3);
            let median_y = y_to_pixel(item.stats.median);
            let min_y = y_to_pixel(item.stats.min);
            let max_y = y_to_pixel(item.stats.max);

            // Draw box (Q1 to Q3)
            self.draw_bar.color = color;
            let box_rect = Rect {
                pos: dvec2(x_center - box_width / 2.0, q3_y),
                size: dvec2(box_width, q1_y - q3_y),
            };
            self.draw_bar.draw_bar(cx, box_rect);

            // Draw median line
            self.draw_line.color = vec4(1.0, 1.0, 1.0, 1.0);
            self.draw_line.draw_line(
                cx,
                dvec2(x_center - box_width / 2.0, median_y),
                dvec2(x_center + box_width / 2.0, median_y),
                2.0
            );

            // Draw whiskers
            self.draw_line.color = vec4(0.3, 0.3, 0.3, 1.0);

            // Lower whisker
            self.draw_line.draw_line(
                cx,
                dvec2(x_center, q1_y),
                dvec2(x_center, min_y),
                1.0
            );
            // Lower whisker cap
            self.draw_line.draw_line(
                cx,
                dvec2(x_center - box_width / 4.0, min_y),
                dvec2(x_center + box_width / 4.0, min_y),
                1.0
            );

            // Upper whisker
            self.draw_line.draw_line(
                cx,
                dvec2(x_center, q3_y),
                dvec2(x_center, max_y),
                1.0
            );
            // Upper whisker cap
            self.draw_line.draw_line(
                cx,
                dvec2(x_center - box_width / 4.0, max_y),
                dvec2(x_center + box_width / 4.0, max_y),
                1.0
            );

            // Draw outliers
            if self.show_outliers {
                self.draw_point.color = color;
                for &outlier in &item.stats.outliers {
                    let outlier_y = y_to_pixel(outlier);
                    self.draw_point.draw_point(cx, dvec2(x_center, outlier_y), 3.0);
                }
            }
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));

        let n = self.items.len();
        let band_width = self.plot_area.width() / n as f64;

        // Category labels
        for (i, item) in self.items.iter().enumerate() {
            let x = self.plot_area.left + (i as f64 + 0.5) * band_width;
            let y = self.plot_area.bottom + 5.0;
            self.label.draw_at(cx, dvec2(x, y), &item.label, TextAnchor::TopCenter);
        }

        // Y axis tick labels
        let (y_min, y_max) = self.get_y_range();
        let y_ticks = self.generate_ticks(y_min, y_max, 5);
        for y in &y_ticks {
            let y_pixel = self.plot_area.bottom - (*y - y_min) / (y_max - y_min) * self.plot_area.height();
            let label = format!("{:.0}", y);
            self.label.draw_at(cx, dvec2(self.plot_area.left - 5.0, y_pixel), &label, TextAnchor::MiddleRight);
        }

        // Title
        if !self.title.is_empty() {
            let center_x = (self.plot_area.left + self.plot_area.right) / 2.0;
            self.label.draw_at(cx, dvec2(center_x, self.plot_area.top - 10.0), &self.title, TextAnchor::BottomCenter);
        }
    }

    fn generate_ticks(&self, min: f64, max: f64, count: usize) -> Vec<f64> {
        let step = (max - min) / count as f64;
        (0..=count).map(|i| min + i as f64 * step).collect()
    }
}

impl BoxPlotChartRef {
    pub fn add_item(&self, item: BoxPlotItem) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_item(item);
        }
    }

    pub fn add_from_values(&self, label: impl Into<String>, values: &[f64]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_from_values(label, values);
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

    pub fn set_show_outliers(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_show_outliers(show);
        }
    }

    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.redraw(cx);
        }
    }
}

