use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;
use super::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::text::PlotLabel;

    pub CandlestickChart = {{CandlestickChart}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    pub WaterfallChart = {{WaterfallChart}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

pub struct Candle {
    pub timestamp: f64,  // X position (can be index or actual timestamp)
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: Option<f64>,
}

impl Candle {
    pub fn new(timestamp: f64, open: f64, high: f64, low: f64, close: f64) -> Self {
        Self { timestamp, open, high, low, close, volume: None }
    }

    pub fn with_volume(mut self, volume: f64) -> Self {
        self.volume = Some(volume);
        self
    }

    pub fn is_bullish(&self) -> bool {
        self.close >= self.open
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct CandlestickChart {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] candles: Vec<Candle>,
    #[rust] plot_area: PlotArea,
    #[rust] bullish_color: Vec4,
    #[rust] bearish_color: Vec4,
    #[rust] show_volume: bool,
    #[rust] candle_width: f64,
    #[rust(50.0)] left_margin: f64,
    #[rust(30.0)] bottom_margin: f64,
    #[rust(20.0)] right_margin: f64,
    #[rust(30.0)] top_margin: f64,
}

impl CandlestickChart {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_data(&mut self, candles: Vec<Candle>) {
        self.candles = candles;
    }

    pub fn add_candle(&mut self, candle: Candle) {
        self.candles.push(candle);
    }

    pub fn set_colors(&mut self, bullish: Vec4, bearish: Vec4) {
        self.bullish_color = bullish;
        self.bearish_color = bearish;
    }

    pub fn set_show_volume(&mut self, show: bool) {
        self.show_volume = show;
    }

    pub fn set_candle_width(&mut self, width: f64) {
        self.candle_width = width;
    }

    pub fn clear(&mut self) {
        self.candles.clear();
    }

    fn compute_ranges(&self) -> (f64, f64, f64, f64) {
        if self.candles.is_empty() {
            return (0.0, 1.0, 0.0, 1.0);
        }

        let x_min = self.candles.first().map(|c| c.timestamp).unwrap_or(0.0);
        let x_max = self.candles.last().map(|c| c.timestamp).unwrap_or(1.0);

        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;
        for c in &self.candles {
            if c.low < y_min { y_min = c.low; }
            if c.high > y_max { y_max = c.high; }
        }

        // Add padding
        let y_range = y_max - y_min;
        y_min -= y_range * 0.05;
        y_max += y_range * 0.05;

        (x_min, x_max, y_min, y_max)
    }
}

impl Widget for CandlestickChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            let plot_rect = Rect {
                pos: dvec2(rect.pos.x + self.left_margin, rect.pos.y + self.top_margin),
                size: dvec2(
                    rect.size.x - self.left_margin - self.right_margin,
                    rect.size.y - self.top_margin - self.bottom_margin
                ),
            };
            self.plot_area = PlotArea::new(plot_rect.pos.x, plot_rect.pos.y,
                plot_rect.pos.x + plot_rect.size.x, plot_rect.pos.y + plot_rect.size.y);

            // Initialize colors if not set
            if self.bullish_color == Vec4::default() {
                self.bullish_color = vec4(0.17, 0.63, 0.17, 1.0); // Green
            }
            if self.bearish_color == Vec4::default() {
                self.bearish_color = vec4(0.84, 0.15, 0.16, 1.0); // Red
            }

            let (x_min, x_max, y_min, y_max) = self.compute_ranges();

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 5.0),
                    &self.title, TextAnchor::TopCenter);
            }

            // Draw axes
            self.draw_line.color = vec4(0.3, 0.3, 0.3, 1.0);
            self.draw_line.draw_line(cx,
                dvec2(plot_rect.pos.x, plot_rect.pos.y + plot_rect.size.y),
                dvec2(plot_rect.pos.x + plot_rect.size.x, plot_rect.pos.y + plot_rect.size.y), 1.0);
            self.draw_line.draw_line(cx,
                dvec2(plot_rect.pos.x, plot_rect.pos.y),
                dvec2(plot_rect.pos.x, plot_rect.pos.y + plot_rect.size.y), 1.0);

            // Draw Y axis labels
            let num_ticks = 5;
            for i in 0..=num_ticks {
                let t = i as f64 / num_ticks as f64;
                let y_val = y_min + (y_max - y_min) * t;
                let y_pos = plot_rect.pos.y + plot_rect.size.y - t * plot_rect.size.y;
                self.label.draw_at(cx, dvec2(plot_rect.pos.x - 5.0, y_pos),
                    &format!("{:.1}", y_val), TextAnchor::MiddleRight);
            }

            // Calculate candle width based on number of candles
            let candle_width = if self.candle_width > 0.0 {
                self.candle_width
            } else if !self.candles.is_empty() {
                (plot_rect.size.x / self.candles.len() as f64 * 0.7).min(20.0).max(3.0)
            } else {
                10.0
            };

            // Draw candles
            for candle in &self.candles {
                let x = if x_max > x_min {
                    plot_rect.pos.x + (candle.timestamp - x_min) / (x_max - x_min) * plot_rect.size.x
                } else {
                    plot_rect.pos.x + plot_rect.size.x / 2.0
                };

                let data_to_y = |v: f64| {
                    plot_rect.pos.y + plot_rect.size.y - (v - y_min) / (y_max - y_min) * plot_rect.size.y
                };

                let open_y = data_to_y(candle.open);
                let close_y = data_to_y(candle.close);
                let high_y = data_to_y(candle.high);
                let low_y = data_to_y(candle.low);

                let color = if candle.is_bullish() { self.bullish_color } else { self.bearish_color };

                // Draw wick (high-low line)
                self.draw_line.color = color;
                self.draw_line.draw_line(cx, dvec2(x, high_y), dvec2(x, low_y), 1.0);

                // Draw body
                let body_top = open_y.min(close_y);
                let body_height = (open_y - close_y).abs().max(1.0);

                self.draw_fill.color = color;
                self.draw_fill.draw_abs(cx, Rect {
                    pos: dvec2(x - candle_width / 2.0, body_top),
                    size: dvec2(candle_width, body_height),
                });
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl CandlestickChartRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn set_data(&self, candles: Vec<Candle>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(candles); }
    }
    pub fn add_candle(&self, candle: Candle) {
        if let Some(mut inner) = self.borrow_mut() { inner.add_candle(candle); }
    }
    pub fn set_colors(&self, bullish: Vec4, bearish: Vec4) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_colors(bullish, bearish); }
    }
    pub fn set_show_volume(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_volume(show); }
    }
    pub fn set_candle_width(&self, width: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_candle_width(width); }
    }
    pub fn clear(&self) {
        if let Some(mut inner) = self.borrow_mut() { inner.clear(); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}


// =============================================================================
// WaterfallChart Widget
// =============================================================================

pub struct WaterfallEntry {
    pub label: String,
    pub value: f64,
    pub is_total: bool,  // If true, shows absolute value from baseline
}

impl WaterfallEntry {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self { label: label.into(), value, is_total: false }
    }

    pub fn total(label: impl Into<String>, value: f64) -> Self {
        Self { label: label.into(), value, is_total: true }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct WaterfallChart {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] entries: Vec<WaterfallEntry>,
    #[rust] positive_color: Vec4,
    #[rust] negative_color: Vec4,
    #[rust] total_color: Vec4,
    #[rust] connector_color: Vec4,
    #[rust(50.0)] left_margin: f64,
    #[rust(50.0)] bottom_margin: f64,
    #[rust(20.0)] right_margin: f64,
    #[rust(30.0)] top_margin: f64,
}

impl WaterfallChart {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_data(&mut self, entries: Vec<WaterfallEntry>) {
        self.entries = entries;
    }

    pub fn add_entry(&mut self, entry: WaterfallEntry) {
        self.entries.push(entry);
    }

    pub fn set_colors(&mut self, positive: Vec4, negative: Vec4, total: Vec4) {
        self.positive_color = positive;
        self.negative_color = negative;
        self.total_color = total;
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Widget for WaterfallChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.entries.is_empty() {
            // Initialize colors
            if self.positive_color == Vec4::default() {
                self.positive_color = vec4(0.17, 0.63, 0.17, 1.0);
            }
            if self.negative_color == Vec4::default() {
                self.negative_color = vec4(0.84, 0.15, 0.16, 1.0);
            }
            if self.total_color == Vec4::default() {
                self.total_color = vec4(0.12, 0.47, 0.71, 1.0);
            }
            if self.connector_color == Vec4::default() {
                self.connector_color = vec4(0.5, 0.5, 0.5, 0.5);
            }

            let plot_rect = Rect {
                pos: dvec2(rect.pos.x + self.left_margin, rect.pos.y + self.top_margin),
                size: dvec2(
                    rect.size.x - self.left_margin - self.right_margin,
                    rect.size.y - self.top_margin - self.bottom_margin
                ),
            };

            // Calculate cumulative values and ranges
            let mut cumulative = 0.0;
            let mut min_val = 0.0f64;
            let mut max_val = 0.0f64;
            let mut bar_data: Vec<(f64, f64, bool, f64)> = Vec::new(); // (start, end, is_total, value)

            for entry in &self.entries {
                if entry.is_total {
                    bar_data.push((0.0, entry.value, true, entry.value));
                    min_val = min_val.min(0.0).min(entry.value);
                    max_val = max_val.max(0.0).max(entry.value);
                } else {
                    let start = cumulative;
                    cumulative += entry.value;
                    bar_data.push((start, cumulative, false, entry.value));
                    min_val = min_val.min(start).min(cumulative);
                    max_val = max_val.max(start).max(cumulative);
                }
            }

            // Add padding
            let range = max_val - min_val;
            min_val -= range * 0.1;
            max_val += range * 0.1;

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 5.0),
                    &self.title, TextAnchor::TopCenter);
            }

            // Draw axes
            self.draw_line.color = vec4(0.3, 0.3, 0.3, 1.0);
            self.draw_line.draw_line(cx,
                dvec2(plot_rect.pos.x, plot_rect.pos.y + plot_rect.size.y),
                dvec2(plot_rect.pos.x + plot_rect.size.x, plot_rect.pos.y + plot_rect.size.y), 1.0);
            self.draw_line.draw_line(cx,
                dvec2(plot_rect.pos.x, plot_rect.pos.y),
                dvec2(plot_rect.pos.x, plot_rect.pos.y + plot_rect.size.y), 1.0);

            // Draw zero line if in range
            if min_val < 0.0 && max_val > 0.0 {
                let zero_y = plot_rect.pos.y + plot_rect.size.y - (-min_val) / (max_val - min_val) * plot_rect.size.y;
                self.draw_line.color = vec4(0.5, 0.5, 0.5, 0.5);
                self.draw_line.draw_line(cx,
                    dvec2(plot_rect.pos.x, zero_y),
                    dvec2(plot_rect.pos.x + plot_rect.size.x, zero_y), 1.0);
            }

            let num_bars = self.entries.len();
            let bar_width = (plot_rect.size.x / num_bars as f64 * 0.7).min(60.0);
            let bar_spacing = plot_rect.size.x / num_bars as f64;

            let value_to_y = |v: f64| {
                plot_rect.pos.y + plot_rect.size.y - (v - min_val) / (max_val - min_val) * plot_rect.size.y
            };

            // Draw bars and connectors
            let mut prev_end_y = None;
            for (i, ((start, end, is_total, value), entry)) in bar_data.iter().zip(self.entries.iter()).enumerate() {
                let x = plot_rect.pos.x + i as f64 * bar_spacing + (bar_spacing - bar_width) / 2.0;
                let start_y = value_to_y(*start);
                let end_y = value_to_y(*end);

                // Draw connector from previous bar
                if let Some(prev_y) = prev_end_y {
                    if !is_total {
                        self.draw_line.color = self.connector_color;
                        self.draw_line.draw_line(cx,
                            dvec2(x - (bar_spacing - bar_width) / 2.0, prev_y),
                            dvec2(x, prev_y), 1.0);
                    }
                }

                // Draw bar
                let color = if *is_total {
                    self.total_color
                } else if *value >= 0.0 {
                    self.positive_color
                } else {
                    self.negative_color
                };

                let bar_top = start_y.min(end_y);
                let bar_height = (start_y - end_y).abs().max(1.0);

                self.draw_fill.color = color;
                self.draw_fill.draw_abs(cx, Rect {
                    pos: dvec2(x, bar_top),
                    size: dvec2(bar_width, bar_height),
                });

                // Draw label
                self.label.draw_at(cx,
                    dvec2(x + bar_width / 2.0, plot_rect.pos.y + plot_rect.size.y + 5.0),
                    &entry.label, TextAnchor::TopCenter);

                // Draw value
                let value_y = if *value >= 0.0 { bar_top - 3.0 } else { bar_top + bar_height + 12.0 };
                self.label.draw_at(cx, dvec2(x + bar_width / 2.0, value_y),
                    &format!("{:.0}", value), TextAnchor::BottomCenter);

                prev_end_y = Some(end_y);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl WaterfallChartRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn set_data(&self, entries: Vec<WaterfallEntry>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(entries); }
    }
    pub fn add_entry(&self, entry: WaterfallEntry) {
        if let Some(mut inner) = self.borrow_mut() { inner.add_entry(entry); }
    }
    pub fn set_colors(&self, positive: Vec4, negative: Vec4, total: Vec4) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_colors(positive, negative, total); }
    }
    pub fn clear(&self) {
        if let Some(mut inner) = self.borrow_mut() { inner.clear(); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}

