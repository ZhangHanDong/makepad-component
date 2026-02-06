// Plot widgets - matplotlib-style plotting

use makepad_widgets::*;
use crate::elements::*;
use crate::text::*;

// Re-export styling enums
pub use crate::elements::{LineStyle, MarkerStyle};


/// Step plot style - where to place the step
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum StepStyle {
    #[default]
    None,   // Normal line (no step)
    Pre,    // Step before the point (y value changes at x)
    Post,   // Step after the point (y value changes at next x)
    Mid,    // Step in the middle between points
}

/// Vertical line annotation
#[derive(Clone)]
pub struct VLine {
    pub x: f64,
    pub color: Vec4,
    pub line_width: f64,
    pub line_style: LineStyle,
}

/// Horizontal line annotation
#[derive(Clone)]
pub struct HLine {
    pub y: f64,
    pub color: Vec4,
    pub line_width: f64,
    pub line_style: LineStyle,
}

/// Vertical span (shaded region)
#[derive(Clone)]
pub struct VSpan {
    pub x1: f64,
    pub x2: f64,
    pub color: Vec4,
}

/// Horizontal span (shaded region)
#[derive(Clone)]
pub struct HSpan {
    pub y1: f64,
    pub y2: f64,
    pub color: Vec4,
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::elements::DrawPlotLine;
    use crate::elements::DrawPlotPoint;
    use crate::elements::DrawPlotBar;
    use crate::elements::DrawPlotFill;
    use crate::text::PlotLabel;

    // Simple line plot widget
    pub LinePlot = {{LinePlot}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
        math_label: <PlotLabel> {}
    }

    // Simple bar plot widget
    pub BarPlot = {{BarPlot}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Scatter plot widget
    pub ScatterPlot = {{ScatterPlot}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Pie chart widget
    pub PieChart = {{PieChart}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Histogram chart widget
    pub HistogramChart = {{HistogramChart}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Box plot chart widget
    pub BoxPlotChart = {{BoxPlotChart}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Heatmap chart widget
    pub HeatmapChart = {{HeatmapChart}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Stem plot widget (lollipop chart)
    pub StemPlot = {{StemPlot}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Violin plot widget (KDE distribution)
    pub ViolinPlot = {{ViolinPlot}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Polar plot widget
    pub PolarPlot = {{PolarPlot}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Contour plot widget
    pub ContourPlot = {{ContourPlot}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Quiver plot widget (vector field)
    pub QuiverPlot = {{QuiverPlot}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // 3D Surface plot widget
    pub Surface3D = {{Surface3D}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // 3D Scatter plot widget
    pub Scatter3D = {{Scatter3D}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // 3D Line plot widget
    pub Line3D = {{Line3D}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

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

    // Candlestick chart (OHLC financial data)
    pub CandlestickChart = {{CandlestickChart}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Radar/Spider chart (multi-axis comparison)
    pub RadarChart = {{RadarChart}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Waterfall chart (cumulative effect)
    pub WaterfallChart = {{WaterfallChart}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Gauge chart (single value dial)
    pub GaugeChart = {{GaugeChart}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Funnel chart (progressive reduction)
    pub FunnelChart = {{FunnelChart}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Heatmap (grid-based data visualization)
    pub Heatmap = {{Heatmap}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Treemap (hierarchical data)
    pub Treemap = {{Treemap}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Bubble chart (scatter with size)
    pub BubbleChart = {{BubbleChart}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Donut chart (pie with center hole) - uses proper arc shader
    pub DonutChart = {{DonutChart}} {
        width: Fill,
        height: Fill,

        draw_arc: {}
        label: <PlotLabel> {}
    }

    // Area chart (stacked area)
    pub AreaChart = {{AreaChart}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Step plot (discrete steps)
    pub StepPlot = {{StepPlot}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Candlestick chart (financial OHLC data)
    pub CandlestickChart = {{CandlestickChart}} {
        width: Fill,
        height: Fill,

        label: <PlotLabel> {}
    }

    // Stackplot (stacked area chart)
    pub Stackplot = {{Stackplot}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    // HexbinChart (hexagonal binning)
    pub HexbinChart = {{HexbinChart}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    // Streamgraph (centered stacked area)
    pub Streamgraph = {{Streamgraph}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }

    // SankeyDiagram (flow diagram)
    pub SankeyDiagram = {{SankeyDiagram}} {
        width: Fill,
        height: Fill,
        label: <PlotLabel> {}
    }
}

// Color palette similar to matplotlib
pub fn get_color(index: usize) -> Vec4 {
    let colors = [
        vec4(0.12, 0.47, 0.71, 1.0), // blue
        vec4(1.0, 0.5, 0.05, 1.0),   // orange
        vec4(0.17, 0.63, 0.17, 1.0), // green
        vec4(0.84, 0.15, 0.16, 1.0), // red
        vec4(0.58, 0.40, 0.74, 1.0), // purple
        vec4(0.55, 0.34, 0.29, 1.0), // brown
        vec4(0.89, 0.47, 0.76, 1.0), // pink
        vec4(0.5, 0.5, 0.5, 1.0),    // gray
    ];
    colors[index % colors.len()]
}

/// Lighten a color by blending towards white
/// amount: 0.0 = no change, 1.0 = pure white
pub fn lighten(color: Vec4, amount: f32) -> Vec4 {
    vec4(
        (color.x + (1.0 - color.x) * amount).min(1.0),
        (color.y + (1.0 - color.y) * amount).min(1.0),
        (color.z + (1.0 - color.z) * amount).min(1.0),
        color.w,
    )
}

/// Darken a color by blending towards black
/// amount: 0.0 = no change, 1.0 = pure black
pub fn darken(color: Vec4, amount: f32) -> Vec4 {
    vec4(
        (color.x * (1.0 - amount)).max(0.0),
        (color.y * (1.0 - amount)).max(0.0),
        (color.z * (1.0 - amount)).max(0.0),
        color.w,
    )
}

/// Get a gradient color pair (center, outer) for radial gradients
/// Creates a nice visual depth effect with lighter center and darker edge
pub fn gradient_pair(color: Vec4) -> (Vec4, Vec4) {
    let center = lighten(color, 0.4);  // Bright center
    let outer = darken(color, 0.15);   // Slightly darker edge
    (center, outer)
}

/// Get a gradient color pair with custom amounts
pub fn gradient_pair_custom(color: Vec4, lighten_amount: f32, darken_amount: f32) -> (Vec4, Vec4) {
    let center = lighten(color, lighten_amount);
    let outer = darken(color, darken_amount);
    (center, outer)
}

/// Scale type for axis transformation
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ScaleType {
    #[default]
    Linear,
    Log,      // Logarithmic (base 10)
    SymLog,   // Symmetric log (handles negative values)
    Time,     // Time axis (values are Unix timestamps in seconds)
}

impl ScaleType {
    /// Transform a value according to the scale type
    pub fn transform(&self, value: f64) -> f64 {
        match self {
            ScaleType::Linear | ScaleType::Time => value,
            ScaleType::Log => {
                if value > 0.0 {
                    value.log10()
                } else {
                    f64::NEG_INFINITY
                }
            }
            ScaleType::SymLog => {
                // Symmetric log: sign(x) * log10(1 + |x|)
                let sign = if value >= 0.0 { 1.0 } else { -1.0 };
                sign * (1.0 + value.abs()).log10()
            }
        }
    }

    /// Inverse transform a value
    pub fn inverse(&self, value: f64) -> f64 {
        match self {
            ScaleType::Linear | ScaleType::Time => value,
            ScaleType::Log => 10.0_f64.powf(value),
            ScaleType::SymLog => {
                let sign = if value >= 0.0 { 1.0 } else { -1.0 };
                sign * (10.0_f64.powf(value.abs()) - 1.0)
            }
        }
    }

    /// Generate nice tick values for this scale type
    pub fn generate_ticks(&self, min: f64, max: f64, count: usize) -> Vec<f64> {
        match self {
            ScaleType::Linear => {
                let step = (max - min) / count as f64;
                (0..=count).map(|i| min + i as f64 * step).collect()
            }
            ScaleType::Time => {
                // Time intervals in seconds
                let intervals = [
                    1.0,           // 1 second
                    5.0,           // 5 seconds
                    10.0,          // 10 seconds
                    30.0,          // 30 seconds
                    60.0,          // 1 minute
                    300.0,         // 5 minutes
                    600.0,         // 10 minutes
                    1800.0,        // 30 minutes
                    3600.0,        // 1 hour
                    7200.0,        // 2 hours
                    21600.0,       // 6 hours
                    43200.0,       // 12 hours
                    86400.0,       // 1 day
                    172800.0,      // 2 days
                    604800.0,      // 1 week
                    2592000.0,     // 30 days
                    7776000.0,     // 90 days
                    31536000.0,    // 1 year
                ];

                let range = max - min;
                let target_interval = range / count as f64;

                // Find best interval
                let interval = intervals
                    .iter()
                    .copied()
                    .find(|&i| i >= target_interval)
                    .unwrap_or(intervals[intervals.len() - 1]);

                // Generate ticks aligned to interval
                let first_tick = (min / interval).ceil() * interval;
                let mut ticks = Vec::new();
                let mut tick = first_tick;
                while tick <= max {
                    ticks.push(tick);
                    tick += interval;
                }
                ticks
            }
            ScaleType::Log => {
                if min <= 0.0 || max <= 0.0 {
                    return vec![];
                }
                let log_min = min.log10().floor() as i32;
                let log_max = max.log10().ceil() as i32;
                (log_min..=log_max)
                    .map(|exp| 10.0_f64.powi(exp))
                    .filter(|&v| v >= min && v <= max)
                    .collect()
            }
            ScaleType::SymLog => {
                // For symlog, generate ticks including negative, zero, and positive
                let mut ticks = Vec::new();

                // Add negative ticks
                if min < 0.0 {
                    let neg_max = min.abs();
                    let log_max = neg_max.log10().ceil() as i32;
                    for exp in (0..=log_max).rev() {
                        let val = -10.0_f64.powi(exp);
                        if val >= min {
                            ticks.push(val);
                        }
                    }
                }

                // Add zero if in range
                if min <= 0.0 && max >= 0.0 {
                    ticks.push(0.0);
                }

                // Add positive ticks
                if max > 0.0 {
                    let log_max = max.log10().ceil() as i32;
                    for exp in 0..=log_max {
                        let val = 10.0_f64.powi(exp);
                        if val <= max && val >= min {
                            ticks.push(val);
                        }
                    }
                }

                ticks
            }
        }
    }

    /// Format a tick label for this scale type
    pub fn format_tick(&self, value: f64) -> String {
        match self {
            ScaleType::Linear => format!("{:.1}", value),
            ScaleType::Time => {
                // Format Unix timestamp as human-readable date
                let secs = value as i64;
                let days_since_epoch = secs / 86400;

                // Simple year/month/day calculation
                let mut year: i64 = 1970;
                let mut days_left = days_since_epoch;

                // Advance years
                while days_left >= 365 {
                    let days_in_year: i64 = if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 366 } else { 365 };
                    if days_left < days_in_year {
                        break;
                    }
                    days_left -= days_in_year;
                    year += 1;
                }

                // Days in each month
                let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
                let days_in_months: [i64; 12] = [31, if is_leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

                let mut month: i64 = 1;
                for &days in &days_in_months {
                    if days_left < days {
                        break;
                    }
                    days_left -= days;
                    month += 1;
                }

                let day = days_left + 1;

                // Show as M/D format
                format!("{}/{}", month, day)
            }
            ScaleType::Log => {
                if value > 0.0 {
                    let exp = value.log10().round() as i32;
                    if (10.0_f64.powi(exp) - value).abs() < 1e-10 {
                        format!("10^{}", exp)
                    } else {
                        format!("{:.1}", value)
                    }
                } else {
                    format!("{:.1}", value)
                }
            }
            ScaleType::SymLog => {
                if value == 0.0 {
                    "0".to_string()
                } else if value.abs() >= 1.0 {
                    let exp = value.abs().log10().round() as i32;
                    if (10.0_f64.powi(exp) - value.abs()).abs() < 1e-10 {
                        if value < 0.0 {
                            format!("-10^{}", exp)
                        } else {
                            format!("10^{}", exp)
                        }
                    } else {
                        format!("{:.1}", value)
                    }
                } else {
                    format!("{:.2}", value)
                }
            }
        }
    }
}

/// Legend position options
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LegendPosition {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    None, // Hidden
}

/// Data series for plotting
#[derive(Clone, Debug, Default)]
pub struct Series {
    pub label: String,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub color: Option<Vec4>,
    pub line_style: LineStyle,
    pub marker_style: MarkerStyle,
    pub step_style: StepStyle,
    pub line_width: Option<f64>,
    pub marker_size: Option<f64>,
    // Error bar data
    pub xerr_minus: Option<Vec<f64>>,
    pub xerr_plus: Option<Vec<f64>>,
    pub yerr_minus: Option<Vec<f64>>,
    pub yerr_plus: Option<Vec<f64>>,
}

impl Series {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            x: Vec::new(),
            y: Vec::new(),
            color: None,
            line_style: LineStyle::Solid,
            marker_style: MarkerStyle::None,
            step_style: StepStyle::None,
            line_width: None,
            marker_size: None,
            xerr_minus: None,
            xerr_plus: None,
            yerr_minus: None,
            yerr_plus: None,
        }
    }

    pub fn with_data(mut self, x: Vec<f64>, y: Vec<f64>) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_line_style(mut self, style: LineStyle) -> Self {
        self.line_style = style;
        self
    }

    pub fn with_marker(mut self, style: MarkerStyle) -> Self {
        self.marker_style = style;
        self
    }

    pub fn with_step(mut self, style: StepStyle) -> Self {
        self.step_style = style;
        self
    }

    pub fn with_line_width(mut self, width: f64) -> Self {
        self.line_width = Some(width);
        self
    }

    pub fn with_marker_size(mut self, size: f64) -> Self {
        self.marker_size = Some(size);
        self
    }

    /// Add symmetric y error bars
    pub fn with_yerr(mut self, yerr: Vec<f64>) -> Self {
        self.yerr_minus = Some(yerr.clone());
        self.yerr_plus = Some(yerr);
        self
    }

    /// Add asymmetric y error bars
    pub fn with_yerr_asymmetric(mut self, yerr_minus: Vec<f64>, yerr_plus: Vec<f64>) -> Self {
        self.yerr_minus = Some(yerr_minus);
        self.yerr_plus = Some(yerr_plus);
        self
    }

    /// Add symmetric x error bars
    pub fn with_xerr(mut self, xerr: Vec<f64>) -> Self {
        self.xerr_minus = Some(xerr.clone());
        self.xerr_plus = Some(xerr);
        self
    }

    /// Add asymmetric x error bars
    pub fn with_xerr_asymmetric(mut self, xerr_minus: Vec<f64>, xerr_plus: Vec<f64>) -> Self {
        self.xerr_minus = Some(xerr_minus);
        self.xerr_plus = Some(xerr_plus);
        self
    }
}

/// Plot area boundaries
#[derive(Clone, Copy, Debug, Default)]
pub struct PlotArea {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl PlotArea {
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Self { left, top, right, bottom }
    }

    pub fn width(&self) -> f64 {
        self.right - self.left
    }

    pub fn height(&self) -> f64 {
        self.bottom - self.top
    }
}

// =============================================================================
// LinePlot Widget
// =============================================================================

/// Represents a filled region between two y values (for fill_between)
#[derive(Clone)]
pub struct FillRegion {
    pub x: Vec<f64>,
    pub y1: Vec<f64>,
    pub y2: Vec<f64>,
    pub color: Vec4,
}

/// Text annotation on the plot
#[derive(Clone)]
pub struct TextAnnotation {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub color: Vec4,
    pub font_size: f64,
    pub is_math: bool,  // If true, render as LaTeX using Math widget
}

/// Arrow annotation pointing from one location to another
#[derive(Clone)]
pub struct ArrowAnnotation {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
    pub color: Vec4,
    pub line_width: f64,
    pub head_size: f64,
    pub text: Option<String>,  // Optional label near the arrow start
}

impl ArrowAnnotation {
    pub fn new(start_x: f64, start_y: f64, end_x: f64, end_y: f64) -> Self {
        Self {
            start_x,
            start_y,
            end_x,
            end_y,
            color: vec4(0.2, 0.2, 0.2, 1.0),
            line_width: 1.5,
            head_size: 8.0,
            text: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_line_width(mut self, width: f64) -> Self {
        self.line_width = width;
        self
    }

    pub fn with_head_size(mut self, size: f64) -> Self {
        self.head_size = size;
        self
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

// =============================================================================
// BarPlot Widget
// =============================================================================

/// Bar group for grouped/stacked bar charts
#[derive(Clone, Debug)]
pub struct BarGroup {
    pub label: String,
    pub values: Vec<f64>,
    pub color: Option<Vec4>,
}

impl BarGroup {
    pub fn new(label: impl Into<String>, values: Vec<f64>) -> Self {
        Self { label: label.into(), values, color: None }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BarPlot {
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
    categories: Vec<String>,

    #[rust]
    values: Vec<f64>,

    #[rust]
    bar_color: Option<Vec4>,

    #[rust]
    plot_area: PlotArea,

    #[rust]
    title: String,

    #[rust(50.0)]
    left_margin: f64,

    #[rust(40.0)]
    bottom_margin: f64,

    #[rust(20.0)]
    right_margin: f64,

    #[rust(30.0)]
    top_margin: f64,

    #[rust(0.8)]
    bar_width_ratio: f64,

    // New fields for enhanced bar charts
    #[rust]
    horizontal: bool,

    #[rust]
    stacked: bool,

    #[rust]
    groups: Vec<BarGroup>,

    #[rust]
    show_bar_labels: bool,
}

impl Widget for BarPlot {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        let has_data = !self.values.is_empty() || !self.groups.is_empty();
        if rect.size.x > 0.0 && rect.size.y > 0.0 && has_data {
            self.update_plot_area(rect);
            self.draw_grid(cx);
            self.draw_axes(cx);
            self.draw_bars(cx);
            self.draw_labels(cx);
        }

        DrawStep::done()
    }
}

impl BarPlot {
    /// Set bar data (simple mode - single series)
    pub fn set_data(&mut self, categories: Vec<String>, values: Vec<f64>) {
        self.categories = categories;
        self.values = values;
        self.groups.clear();
    }

    /// Set plot title
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    /// Set bar color (for simple mode)
    pub fn set_color(&mut self, color: Vec4) {
        self.bar_color = Some(color);
    }

    /// Set horizontal orientation (barh)
    pub fn set_horizontal(&mut self, horizontal: bool) {
        self.horizontal = horizontal;
    }

    /// Set stacked mode
    pub fn set_stacked(&mut self, stacked: bool) {
        self.stacked = stacked;
    }

    /// Show bar value labels
    pub fn set_show_bar_labels(&mut self, show: bool) {
        self.show_bar_labels = show;
    }

    /// Add a bar group (for grouped/stacked bars)
    pub fn add_group(&mut self, group: BarGroup) {
        self.groups.push(group);
    }

    /// Set multiple groups at once
    pub fn set_groups(&mut self, categories: Vec<String>, groups: Vec<BarGroup>) {
        self.categories = categories;
        self.groups = groups;
        self.values.clear();
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.categories.clear();
        self.values.clear();
        self.groups.clear();
    }

    fn update_plot_area(&mut self, rect: Rect) {
        let left_margin = if self.horizontal { 80.0 } else { self.left_margin };
        let bottom_margin = if self.horizontal { self.bottom_margin } else { 40.0 };
        self.plot_area = PlotArea::new(
            rect.pos.x + left_margin,
            rect.pos.y + self.top_margin,
            rect.pos.x + rect.size.x - self.right_margin,
            rect.pos.y + rect.size.y - bottom_margin,
        );
    }

    fn get_value_range(&self) -> (f64, f64) {
        if !self.groups.is_empty() {
            if self.stacked {
                // For stacked, sum up all groups per category
                let num_cats = self.categories.len();
                let mut max = 0.0f64;
                for cat_idx in 0..num_cats {
                    let sum: f64 = self.groups.iter()
                        .filter_map(|g| g.values.get(cat_idx))
                        .sum();
                    max = max.max(sum);
                }
                (0.0, max * 1.1)
            } else {
                // For grouped, find max across all values
                let max = self.groups.iter()
                    .flat_map(|g| g.values.iter())
                    .cloned()
                    .fold(0.0f64, f64::max);
                (0.0, max * 1.1)
            }
        } else {
            let max = self.values.iter().cloned().fold(0.0f64, f64::max);
            (0.0, max * 1.1)
        }
    }

    fn draw_grid(&mut self, cx: &mut Cx2d) {
        self.draw_line.color = vec4(0.9, 0.9, 0.9, 1.0);

        let (v_min, v_max) = self.get_value_range();
        let v_ticks = self.generate_ticks(v_min, v_max, 5);

        if self.horizontal {
            // Vertical grid lines for horizontal bars
            for v in &v_ticks {
                let x_pixel = self.plot_area.left + (*v - v_min) / (v_max - v_min) * self.plot_area.width();
                let p1 = dvec2(x_pixel, self.plot_area.top);
                let p2 = dvec2(x_pixel, self.plot_area.bottom);
                self.draw_line.draw_line(cx, p1, p2, 0.5);
            }
        } else {
            // Horizontal grid lines for vertical bars
            for v in &v_ticks {
                let y_pixel = self.plot_area.bottom - (*v - v_min) / (v_max - v_min) * self.plot_area.height();
                let p1 = dvec2(self.plot_area.left, y_pixel);
                let p2 = dvec2(self.plot_area.right, y_pixel);
                self.draw_line.draw_line(cx, p1, p2, 0.5);
            }
        }
    }

    fn draw_axes(&mut self, cx: &mut Cx2d) {
        self.draw_line.color = vec4(0.3, 0.3, 0.3, 1.0);

        if self.horizontal {
            // X axis (bottom)
            let x1 = dvec2(self.plot_area.left, self.plot_area.bottom);
            let x2 = dvec2(self.plot_area.right, self.plot_area.bottom);
            self.draw_line.draw_line(cx, x1, x2, 1.0);

            // Y axis (left)
            let y1 = dvec2(self.plot_area.left, self.plot_area.bottom);
            let y2 = dvec2(self.plot_area.left, self.plot_area.top);
            self.draw_line.draw_line(cx, y1, y2, 1.0);
        } else {
            // X axis
            let x1 = dvec2(self.plot_area.left, self.plot_area.bottom);
            let x2 = dvec2(self.plot_area.right, self.plot_area.bottom);
            self.draw_line.draw_line(cx, x1, x2, 1.0);

            // Y axis
            let y1 = dvec2(self.plot_area.left, self.plot_area.bottom);
            let y2 = dvec2(self.plot_area.left, self.plot_area.top);
            self.draw_line.draw_line(cx, y1, y2, 1.0);
        }
    }

    fn draw_bars(&mut self, cx: &mut Cx2d) {
        let (v_min, v_max) = self.get_value_range();

        if !self.groups.is_empty() {
            self.draw_grouped_bars(cx, v_min, v_max);
        } else {
            self.draw_simple_bars(cx, v_min, v_max);
        }
    }

    fn draw_simple_bars(&mut self, cx: &mut Cx2d, v_min: f64, v_max: f64) {
        let n = self.values.len();
        if n == 0 {
            return;
        }

        if self.horizontal {
            let band_height = self.plot_area.height() / n as f64;
            let bar_height = band_height * self.bar_width_ratio;

            for (i, value) in self.values.iter().enumerate() {
                let color = self.bar_color.unwrap_or_else(|| get_color(0));
                self.draw_bar.color = color;

                let y_center = self.plot_area.top + (i as f64 + 0.5) * band_height;
                let bar_width = (*value - v_min) / (v_max - v_min) * self.plot_area.width();

                let rect = Rect {
                    pos: dvec2(self.plot_area.left, y_center - bar_height / 2.0),
                    size: dvec2(bar_width, bar_height),
                };
                self.draw_bar.draw_bar(cx, rect);

                // Bar label
                if self.show_bar_labels {
                    self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
                    let label = format!("{:.1}", value);
                    self.label.draw_at(cx, dvec2(self.plot_area.left + bar_width + 5.0, y_center), &label, TextAnchor::MiddleLeft);
                }
            }
        } else {
            let band_width = self.plot_area.width() / n as f64;
            let bar_width = band_width * self.bar_width_ratio;

            for (i, value) in self.values.iter().enumerate() {
                let color = self.bar_color.unwrap_or_else(|| get_color(0));
                self.draw_bar.color = color;

                let x_center = self.plot_area.left + (i as f64 + 0.5) * band_width;
                let bar_height = (*value - v_min) / (v_max - v_min) * self.plot_area.height();
                let bar_top = self.plot_area.bottom - bar_height;

                let rect = Rect {
                    pos: dvec2(x_center - bar_width / 2.0, bar_top),
                    size: dvec2(bar_width, bar_height),
                };
                self.draw_bar.draw_bar(cx, rect);

                // Bar label
                if self.show_bar_labels {
                    self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
                    let label = format!("{:.1}", value);
                    self.label.draw_at(cx, dvec2(x_center, bar_top - 5.0), &label, TextAnchor::BottomCenter);
                }
            }
        }
    }

    fn draw_grouped_bars(&mut self, cx: &mut Cx2d, v_min: f64, v_max: f64) {
        let num_cats = self.categories.len();
        let num_groups = self.groups.len();
        if num_cats == 0 || num_groups == 0 {
            return;
        }

        if self.stacked {
            self.draw_stacked_bars(cx, v_min, v_max);
        } else {
            self.draw_side_by_side_bars(cx, v_min, v_max);
        }
    }

    fn draw_stacked_bars(&mut self, cx: &mut Cx2d, v_min: f64, v_max: f64) {
        let num_cats = self.categories.len();

        if self.horizontal {
            let band_height = self.plot_area.height() / num_cats as f64;
            let bar_height = band_height * self.bar_width_ratio;

            for cat_idx in 0..num_cats {
                let y_center = self.plot_area.top + (cat_idx as f64 + 0.5) * band_height;
                let mut x_start = self.plot_area.left;

                for (group_idx, group) in self.groups.iter().enumerate() {
                    if let Some(&value) = group.values.get(cat_idx) {
                        let color = group.color.unwrap_or_else(|| get_color(group_idx));
                        self.draw_bar.color = color;

                        let bar_width = (value - v_min) / (v_max - v_min) * self.plot_area.width();
                        let rect = Rect {
                            pos: dvec2(x_start, y_center - bar_height / 2.0),
                            size: dvec2(bar_width, bar_height),
                        };
                        self.draw_bar.draw_bar(cx, rect);
                        x_start += bar_width;
                    }
                }
            }
        } else {
            let band_width = self.plot_area.width() / num_cats as f64;
            let bar_width = band_width * self.bar_width_ratio;

            for cat_idx in 0..num_cats {
                let x_center = self.plot_area.left + (cat_idx as f64 + 0.5) * band_width;
                let mut y_bottom = self.plot_area.bottom;

                for (group_idx, group) in self.groups.iter().enumerate() {
                    if let Some(&value) = group.values.get(cat_idx) {
                        let color = group.color.unwrap_or_else(|| get_color(group_idx));
                        self.draw_bar.color = color;

                        let bar_height = (value - v_min) / (v_max - v_min) * self.plot_area.height();
                        let bar_top = y_bottom - bar_height;
                        let rect = Rect {
                            pos: dvec2(x_center - bar_width / 2.0, bar_top),
                            size: dvec2(bar_width, bar_height),
                        };
                        self.draw_bar.draw_bar(cx, rect);
                        y_bottom = bar_top;
                    }
                }
            }
        }
    }

    fn draw_side_by_side_bars(&mut self, cx: &mut Cx2d, v_min: f64, v_max: f64) {
        let num_cats = self.categories.len();
        let num_groups = self.groups.len();

        if self.horizontal {
            let band_height = self.plot_area.height() / num_cats as f64;
            let group_height = band_height * self.bar_width_ratio / num_groups as f64;

            for cat_idx in 0..num_cats {
                let y_start = self.plot_area.top + (cat_idx as f64 + 0.5) * band_height
                    - (band_height * self.bar_width_ratio) / 2.0;

                for (group_idx, group) in self.groups.iter().enumerate() {
                    if let Some(&value) = group.values.get(cat_idx) {
                        let color = group.color.unwrap_or_else(|| get_color(group_idx));
                        self.draw_bar.color = color;

                        let y_pos = y_start + group_idx as f64 * group_height;
                        let bar_width = (value - v_min) / (v_max - v_min) * self.plot_area.width();
                        let rect = Rect {
                            pos: dvec2(self.plot_area.left, y_pos),
                            size: dvec2(bar_width, group_height * 0.9),
                        };
                        self.draw_bar.draw_bar(cx, rect);
                    }
                }
            }
        } else {
            let band_width = self.plot_area.width() / num_cats as f64;
            let group_width = band_width * self.bar_width_ratio / num_groups as f64;

            for cat_idx in 0..num_cats {
                let x_start = self.plot_area.left + (cat_idx as f64 + 0.5) * band_width
                    - (band_width * self.bar_width_ratio) / 2.0;

                for (group_idx, group) in self.groups.iter().enumerate() {
                    if let Some(&value) = group.values.get(cat_idx) {
                        let color = group.color.unwrap_or_else(|| get_color(group_idx));
                        self.draw_bar.color = color;

                        let x_pos = x_start + group_idx as f64 * group_width;
                        let bar_height = (value - v_min) / (v_max - v_min) * self.plot_area.height();
                        let bar_top = self.plot_area.bottom - bar_height;
                        let rect = Rect {
                            pos: dvec2(x_pos, bar_top),
                            size: dvec2(group_width * 0.9, bar_height),
                        };
                        self.draw_bar.draw_bar(cx, rect);
                    }
                }
            }
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));

        let n = self.categories.len().max(self.values.len());
        let (v_min, v_max) = self.get_value_range();

        if self.horizontal {
            // Category labels on Y axis
            let band_height = self.plot_area.height() / n as f64;
            for (i, cat) in self.categories.iter().enumerate() {
                let y = self.plot_area.top + (i as f64 + 0.5) * band_height;
                self.label.draw_at(cx, dvec2(self.plot_area.left - 5.0, y), cat, TextAnchor::MiddleRight);
            }

            // Value tick labels on X axis
            let v_ticks = self.generate_ticks(v_min, v_max, 5);
            for v in &v_ticks {
                let x_pixel = self.plot_area.left + (*v - v_min) / (v_max - v_min) * self.plot_area.width();
                let label = format!("{:.0}", v);
                self.label.draw_at(cx, dvec2(x_pixel, self.plot_area.bottom + 5.0), &label, TextAnchor::TopCenter);
            }
        } else {
            // Category labels on X axis
            let band_width = self.plot_area.width() / n as f64;
            for (i, cat) in self.categories.iter().enumerate() {
                let x = self.plot_area.left + (i as f64 + 0.5) * band_width;
                self.label.draw_at(cx, dvec2(x, self.plot_area.bottom + 5.0), cat, TextAnchor::TopCenter);
            }

            // Value tick labels on Y axis
            let v_ticks = self.generate_ticks(v_min, v_max, 5);
            for v in &v_ticks {
                let y_pixel = self.plot_area.bottom - (*v - v_min) / (v_max - v_min) * self.plot_area.height();
                let label = format!("{:.0}", v);
                self.label.draw_at(cx, dvec2(self.plot_area.left - 5.0, y_pixel), &label, TextAnchor::MiddleRight);
            }
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

impl BarPlotRef {
    pub fn set_data(&self, categories: Vec<String>, values: Vec<f64>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_data(categories, values);
        }
    }

    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_title(title);
        }
    }

    pub fn set_color(&self, color: Vec4) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_color(color);
        }
    }

    pub fn set_horizontal(&self, horizontal: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_horizontal(horizontal);
        }
    }

    pub fn set_stacked(&self, stacked: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_stacked(stacked);
        }
    }

    pub fn set_show_bar_labels(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_show_bar_labels(show);
        }
    }

    pub fn add_group(&self, group: BarGroup) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_group(group);
        }
    }

    pub fn set_groups(&self, categories: Vec<String>, groups: Vec<BarGroup>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_groups(categories, groups);
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
// ScatterPlot Widget
// =============================================================================

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

// =============================================================================
// PieChart Widget
// =============================================================================

/// Pie slice data
#[derive(Clone, Debug, Default)]
pub struct PieSlice {
    pub label: String,
    pub value: f64,
    pub color: Option<Vec4>,
}

impl PieSlice {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct PieChart {
    #[deref]
    #[live]
    view: View,

    #[live]
    draw_slice: DrawPieSlice,

    #[live]
    draw_line: DrawPlotLine,

    #[live]
    label: PlotLabel,

    #[rust]
    slices: Vec<PieSlice>,

    #[rust]
    title: String,

    #[rust(0.8)]
    radius_ratio: f64,

    #[rust]
    show_labels: bool,

    #[rust]
    show_percentages: bool,

    #[rust]
    legend_position: LegendPosition,
}

impl Widget for PieChart {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.slices.is_empty() {
            self.draw_pie(cx, rect);
            self.draw_title(cx, rect);
            self.draw_legend(cx, rect);
        }

        DrawStep::done()
    }
}

impl PieChart {
    pub fn add_slice(&mut self, slice: PieSlice) {
        self.slices.push(slice);
    }

    pub fn set_slices(&mut self, slices: Vec<PieSlice>) {
        self.slices = slices;
    }

    pub fn set_data(&mut self, labels: Vec<String>, values: Vec<f64>) {
        self.slices = labels.into_iter().zip(values).map(|(l, v)| PieSlice::new(l, v)).collect();
    }

    pub fn clear(&mut self) {
        self.slices.clear();
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_show_percentages(&mut self, show: bool) {
        self.show_percentages = show;
    }

    pub fn set_show_labels(&mut self, show: bool) {
        self.show_labels = show;
    }

    pub fn set_legend(&mut self, position: LegendPosition) {
        self.legend_position = position;
    }

    fn draw_pie(&mut self, cx: &mut Cx2d, rect: Rect) {
        let total: f64 = self.slices.iter().map(|s| s.value).sum();
        if total <= 0.0 {
            return;
        }

        // Account for title and legend margins
        let title_margin = if !self.title.is_empty() { 30.0 } else { 10.0 };
        let bottom_margin = 10.0;
        let available_height = rect.size.y - title_margin - bottom_margin;
        let available_width = rect.size.x - 20.0; // Side margins

        let center = dvec2(
            rect.pos.x + rect.size.x / 2.0,
            rect.pos.y + title_margin + available_height / 2.0,
        );
        let radius = (available_width.min(available_height) / 2.0) * self.radius_ratio;

        let mut start_angle = -std::f64::consts::FRAC_PI_2;

        for (idx, slice) in self.slices.iter().enumerate() {
            let slice_angle = (slice.value / total) * std::f64::consts::TAU;
            let end_angle = start_angle + slice_angle;

            let shader_start = (start_angle + std::f64::consts::TAU) % std::f64::consts::TAU;
            let shader_end = shader_start + slice_angle;

            let color = slice.color.unwrap_or_else(|| get_color(idx));
            self.draw_slice.color = color;
            self.draw_slice.draw_slice(cx, center, radius, shader_start, shader_end);

            if self.show_percentages {
                let mid_angle = start_angle + slice_angle / 2.0;
                let label_radius = radius * 0.65;
                let label_x = center.x + mid_angle.cos() * label_radius;
                let label_y = center.y + mid_angle.sin() * label_radius;

                let percentage = (slice.value / total) * 100.0;
                let label_text = format!("{:.1}%", percentage);

                self.label.set_color(vec4(1.0, 1.0, 1.0, 1.0));
                self.label.draw_at(cx, dvec2(label_x, label_y), &label_text, TextAnchor::Center);
            }

            start_angle = end_angle;
        }
    }

    fn draw_title(&mut self, cx: &mut Cx2d, rect: Rect) {
        if !self.title.is_empty() {
            let center_x = rect.pos.x + rect.size.x / 2.0;
            self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
            self.label.draw_at(cx, dvec2(center_x, rect.pos.y + 10.0), &self.title, TextAnchor::TopCenter);
        }
    }

    fn draw_legend(&mut self, cx: &mut Cx2d, rect: Rect) {
        if self.legend_position == LegendPosition::None || self.slices.is_empty() {
            return;
        }

        let padding = 8.0;
        let line_height = 16.0;
        let marker_size = 10.0;
        let marker_text_gap = 6.0;
        let legend_height = self.slices.len() as f64 * line_height + padding * 2.0;
        let legend_width = 100.0;

        let (legend_x, legend_y) = match self.legend_position {
            LegendPosition::TopRight => (
                rect.pos.x + rect.size.x - legend_width - 10.0,
                rect.pos.y + 30.0,
            ),
            LegendPosition::TopLeft => (
                rect.pos.x + 10.0,
                rect.pos.y + 30.0,
            ),
            LegendPosition::BottomRight => (
                rect.pos.x + rect.size.x - legend_width - 10.0,
                rect.pos.y + rect.size.y - legend_height - 10.0,
            ),
            LegendPosition::BottomLeft => (
                rect.pos.x + 10.0,
                rect.pos.y + rect.size.y - legend_height - 10.0,
            ),
            LegendPosition::None => return,
        };

        self.draw_line.color = vec4(0.95, 0.95, 0.95, 0.9);
        let bg_rect = Rect {
            pos: dvec2(legend_x, legend_y),
            size: dvec2(legend_width, legend_height),
        };
        self.draw_line.draw_abs(cx, bg_rect);

        self.draw_line.color = vec4(0.8, 0.8, 0.8, 1.0);
        self.draw_line.draw_line(cx, dvec2(legend_x, legend_y), dvec2(legend_x + legend_width, legend_y), 1.0);
        self.draw_line.draw_line(cx, dvec2(legend_x, legend_y + legend_height), dvec2(legend_x + legend_width, legend_y + legend_height), 1.0);
        self.draw_line.draw_line(cx, dvec2(legend_x, legend_y), dvec2(legend_x, legend_y + legend_height), 1.0);
        self.draw_line.draw_line(cx, dvec2(legend_x + legend_width, legend_y), dvec2(legend_x + legend_width, legend_y + legend_height), 1.0);

        for (idx, slice) in self.slices.iter().enumerate() {
            let color = slice.color.unwrap_or_else(|| get_color(idx));
            let entry_y = legend_y + padding + idx as f64 * line_height + line_height / 2.0;

            self.draw_line.color = color;
            let marker_rect = Rect {
                pos: dvec2(legend_x + padding, entry_y - marker_size / 2.0),
                size: dvec2(marker_size, marker_size),
            };
            self.draw_line.draw_abs(cx, marker_rect);

            self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
            self.label.draw_at(
                cx,
                dvec2(legend_x + padding + marker_size + marker_text_gap, entry_y),
                &slice.label,
                TextAnchor::MiddleLeft,
            );
        }
    }
}

impl PieChartRef {
    pub fn add_slice(&self, slice: PieSlice) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_slice(slice);
        }
    }

    pub fn set_slices(&self, slices: Vec<PieSlice>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_slices(slices);
        }
    }

    pub fn set_data(&self, labels: Vec<String>, values: Vec<f64>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_data(labels, values);
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

    pub fn set_show_percentages(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_show_percentages(show);
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

// =============================================================================
// HistogramChart Widget
// =============================================================================

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

// =============================================================================
// HeatmapChart Widget
// =============================================================================

/// Colormap for heatmap and other visualizations
#[derive(Clone, Debug, PartialEq)]
pub enum Colormap {
    // Perceptually uniform sequential
    Viridis,
    Plasma,
    Inferno,
    Magma,
    Cividis,  // Colorblind-friendly
    // Diverging
    Coolwarm,
    RdBu,     // Red-Blue diverging
    Spectral,
    // Sequential
    Blues,
    Greens,
    Oranges,
    Reds,
    Greys,
    // Classic
    Jet,      // Rainbow (legacy)
    Hot,      // Black-Red-Yellow-White
    // Special
    Turbo,    // Improved rainbow
    Custom(Vec<(f64, Vec4)>),  // User-defined color stops
}

impl Default for Colormap {
    fn default() -> Self {
        Colormap::Viridis
    }
}

impl Colormap {
    /// Sample a color from the colormap at position t (0.0 to 1.0)
    pub fn sample(&self, t: f64) -> Vec4 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Colormap::Viridis => {
                // Perceptually uniform purple-green-yellow
                let r = 0.267 + t * 0.329 - t * t * 0.5 + t * t * t * 0.9;
                let g = 0.004 + t * 0.873;
                let b = 0.329 + t * 0.5 - t * t * 0.6;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Plasma => {
                // Purple-red-yellow
                let r = 0.05 + t * 0.9 + t * t * 0.05;
                let g = t * t * 0.9;
                let b = 0.53 + (1.0 - t) * 0.47 - t * t * 0.6;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Inferno => {
                // Black-purple-red-yellow
                let r = t * t * 1.2;
                let g = t * t * t * 1.5;
                let b = (1.0 - t) * t * 2.0 + 0.1 * t;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Magma => {
                // Black-purple-pink-white
                let r = t * t * 0.8 + t * 0.2;
                let g = t * t * t * 1.2;
                let b = t * 0.5 + (1.0 - t) * t * 1.0;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Cividis => {
                // Colorblind-friendly blue-yellow
                let r = -0.01 + t * 1.0 + t * t * 0.01;
                let g = 0.14 + t * 0.72;
                let b = 0.35 + t * 0.1 - t * t * 0.35;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Coolwarm => {
                // Blue-white-red diverging
                let r = if t < 0.5 { 0.2 + t * 1.6 } else { 1.0 };
                let g = if t < 0.5 { 0.2 + t * 1.0 } else { 1.0 - (t - 0.5) * 1.6 };
                let b = if t < 0.5 { 1.0 } else { 1.0 - (t - 0.5) * 1.6 };
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::RdBu => {
                // Red-white-blue diverging (red=high, blue=low)
                let r = if t < 0.5 { 0.1 + t * 1.8 } else { 1.0 - (t - 0.5) * 1.4 };
                let g = if t < 0.5 { t * 1.8 } else { 0.9 - (t - 0.5) * 1.6 };
                let b = if t < 0.5 { 1.0 - t * 0.2 } else { 0.9 - (t - 0.5) * 1.0 };
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Spectral => {
                // Red-orange-yellow-green-blue (diverging rainbow)
                let (r, g, b) = if t < 0.25 {
                    let s = t / 0.25;
                    (0.62 + s * 0.38, 0.0 + s * 0.5, 0.26 * (1.0 - s))
                } else if t < 0.5 {
                    let s = (t - 0.25) / 0.25;
                    (1.0, 0.5 + s * 0.5, 0.0)
                } else if t < 0.75 {
                    let s = (t - 0.5) / 0.25;
                    (1.0 - s * 0.5, 1.0 - s * 0.2, s * 0.4)
                } else {
                    let s = (t - 0.75) / 0.25;
                    (0.5 - s * 0.3, 0.8 - s * 0.4, 0.4 + s * 0.6)
                };
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Blues => {
                let r = 1.0 - t * 0.8;
                let g = 1.0 - t * 0.5;
                let b = 1.0;
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Greens => {
                let r = 1.0 - t * 0.75;
                let g = 1.0 - t * 0.15;
                let b = 1.0 - t * 0.7;
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Oranges => {
                let r = 1.0;
                let g = 1.0 - t * 0.6;
                let b = 1.0 - t * 0.85;
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Reds => {
                let r = 1.0;
                let g = 1.0 - t * 0.85;
                let b = 1.0 - t * 0.85;
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Greys => {
                let v = 1.0 - t * 0.9;
                vec4(v as f32, v as f32, v as f32, 1.0)
            }
            Colormap::Jet => {
                // Classic rainbow: blue-cyan-green-yellow-red
                let (r, g, b) = if t < 0.125 {
                    (0.0, 0.0, 0.5 + t * 4.0)
                } else if t < 0.375 {
                    let s = (t - 0.125) / 0.25;
                    (0.0, s, 1.0)
                } else if t < 0.625 {
                    let s = (t - 0.375) / 0.25;
                    (s, 1.0, 1.0 - s)
                } else if t < 0.875 {
                    let s = (t - 0.625) / 0.25;
                    (1.0, 1.0 - s, 0.0)
                } else {
                    let s = (t - 0.875) / 0.125;
                    (1.0 - s * 0.5, 0.0, 0.0)
                };
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Hot => {
                // Black-red-yellow-white
                let (r, g, b) = if t < 0.33 {
                    (t * 3.0, 0.0, 0.0)
                } else if t < 0.67 {
                    let s = (t - 0.33) / 0.34;
                    (1.0, s, 0.0)
                } else {
                    let s = (t - 0.67) / 0.33;
                    (1.0, 1.0, s)
                };
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Turbo => {
                // Improved rainbow with better perceptual uniformity
                let r = 0.13572 + t * (4.6153 + t * (-42.66 + t * (132.13 + t * (-152.95 + t * 56.31))));
                let g = 0.09140 + t * (2.1745 + t * (4.8321 + t * (-36.60 + t * (43.05 + t * (-13.22)))));
                let b = 0.10667 + t * (12.755 + t * (-60.58 + t * (109.33 + t * (-87.15 + t * 25.25))));
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Custom(stops) => {
                if stops.is_empty() {
                    return vec4(0.5, 0.5, 0.5, 1.0);
                }
                if stops.len() == 1 {
                    return stops[0].1;
                }
                // Find surrounding stops and interpolate
                for i in 0..stops.len() - 1 {
                    if t <= stops[i + 1].0 {
                        let t0 = stops[i].0;
                        let t1 = stops[i + 1].0;
                        let c0 = stops[i].1;
                        let c1 = stops[i + 1].1;
                        let s = if t1 > t0 { (t - t0) / (t1 - t0) } else { 0.0 };
                        return vec4(
                            c0.x + (c1.x - c0.x) * s as f32,
                            c0.y + (c1.y - c0.y) * s as f32,
                            c0.z + (c1.z - c0.z) * s as f32,
                            c0.w + (c1.w - c0.w) * s as f32,
                        );
                    }
                }
                stops.last().unwrap().1
            }
        }
    }

    /// Get a list of all named colormaps
    pub fn all_named() -> Vec<Colormap> {
        vec![
            Colormap::Viridis,
            Colormap::Plasma,
            Colormap::Inferno,
            Colormap::Magma,
            Colormap::Cividis,
            Colormap::Coolwarm,
            Colormap::RdBu,
            Colormap::Spectral,
            Colormap::Blues,
            Colormap::Greens,
            Colormap::Oranges,
            Colormap::Reds,
            Colormap::Greys,
            Colormap::Jet,
            Colormap::Hot,
            Colormap::Turbo,
        ]
    }

    /// Get the name of this colormap
    pub fn name(&self) -> &'static str {
        match self {
            Colormap::Viridis => "Viridis",
            Colormap::Plasma => "Plasma",
            Colormap::Inferno => "Inferno",
            Colormap::Magma => "Magma",
            Colormap::Cividis => "Cividis",
            Colormap::Coolwarm => "Coolwarm",
            Colormap::RdBu => "RdBu",
            Colormap::Spectral => "Spectral",
            Colormap::Blues => "Blues",
            Colormap::Greens => "Greens",
            Colormap::Oranges => "Oranges",
            Colormap::Reds => "Reds",
            Colormap::Greys => "Greys",
            Colormap::Jet => "Jet",
            Colormap::Hot => "Hot",
            Colormap::Turbo => "Turbo",
            Colormap::Custom(_) => "Custom",
        }
    }

    /// Create a custom colormap from color stops
    /// Stops should be (position, color) pairs where position is 0.0 to 1.0
    pub fn custom(stops: Vec<(f64, Vec4)>) -> Self {
        Colormap::Custom(stops)
    }
}

// =============================================================================
// Normalization for Colormaps
// =============================================================================

/// Linear normalization: maps [vmin, vmax] to [0, 1]
#[derive(Clone, Debug)]
pub struct Normalize {
    pub vmin: f64,
    pub vmax: f64,
    pub clip: bool,
}

impl Normalize {
    pub fn new(vmin: f64, vmax: f64) -> Self {
        Self { vmin, vmax, clip: true }
    }

    pub fn with_clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Normalize a value to [0, 1]
    pub fn normalize(&self, value: f64) -> f64 {
        if self.vmax == self.vmin {
            return 0.5;
        }
        let t = (value - self.vmin) / (self.vmax - self.vmin);
        if self.clip { t.clamp(0.0, 1.0) } else { t }
    }

    /// Inverse: convert [0, 1] back to original scale
    pub fn inverse(&self, t: f64) -> f64 {
        self.vmin + t * (self.vmax - self.vmin)
    }
}

impl Default for Normalize {
    fn default() -> Self {
        Self { vmin: 0.0, vmax: 1.0, clip: true }
    }
}

/// Logarithmic normalization: maps [vmin, vmax] to [0, 1] on log scale
/// Values must be positive
#[derive(Clone, Debug)]
pub struct LogNorm {
    pub vmin: f64,
    pub vmax: f64,
    pub clip: bool,
    log_vmin: f64,
    log_vmax: f64,
}

impl LogNorm {
    pub fn new(vmin: f64, vmax: f64) -> Self {
        let vmin = vmin.max(1e-10);  // Ensure positive
        let vmax = vmax.max(vmin + 1e-10);
        Self {
            vmin,
            vmax,
            clip: true,
            log_vmin: vmin.log10(),
            log_vmax: vmax.log10(),
        }
    }

    pub fn with_clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Normalize a value to [0, 1] on log scale
    pub fn normalize(&self, value: f64) -> f64 {
        if value <= 0.0 {
            return 0.0;
        }
        let log_val = value.log10();
        let t = (log_val - self.log_vmin) / (self.log_vmax - self.log_vmin);
        if self.clip { t.clamp(0.0, 1.0) } else { t }
    }

    /// Inverse: convert [0, 1] back to original scale
    pub fn inverse(&self, t: f64) -> f64 {
        10.0_f64.powf(self.log_vmin + t * (self.log_vmax - self.log_vmin))
    }
}

impl Default for LogNorm {
    fn default() -> Self {
        Self::new(1.0, 10.0)
    }
}

/// Symmetric log normalization: handles negative values
#[derive(Clone, Debug)]
pub struct SymLogNorm {
    pub vmin: f64,
    pub vmax: f64,
    pub linthresh: f64,  // Linear threshold
    pub clip: bool,
}

impl SymLogNorm {
    pub fn new(vmin: f64, vmax: f64, linthresh: f64) -> Self {
        Self {
            vmin,
            vmax,
            linthresh: linthresh.abs().max(1e-10),
            clip: true,
        }
    }

    pub fn with_clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    fn transform(&self, value: f64) -> f64 {
        if value.abs() <= self.linthresh {
            value / self.linthresh
        } else {
            let sign = if value >= 0.0 { 1.0 } else { -1.0 };
            sign * (1.0 + (value.abs() / self.linthresh).log10())
        }
    }

    /// Normalize a value to [0, 1]
    pub fn normalize(&self, value: f64) -> f64 {
        let t_val = self.transform(value);
        let t_min = self.transform(self.vmin);
        let t_max = self.transform(self.vmax);

        if t_max == t_min {
            return 0.5;
        }

        let t = (t_val - t_min) / (t_max - t_min);
        if self.clip { t.clamp(0.0, 1.0) } else { t }
    }
}

impl Default for SymLogNorm {
    fn default() -> Self {
        Self::new(-10.0, 10.0, 1.0)
    }
}

// =============================================================================
// StemPlot Widget (Lollipop Chart)
// =============================================================================

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

// =============================================================================
// HeatmapChart Widget
// =============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct HeatmapChart {
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
    data: Vec<Vec<f64>>,

    #[rust]
    x_labels: Option<Vec<String>>,

    #[rust]
    y_labels: Option<Vec<String>>,

    #[rust]
    plot_area: PlotArea,

    #[rust]
    title: String,

    #[rust]
    colormap: Colormap,

    #[rust]
    vmin: Option<f64>,

    #[rust]
    vmax: Option<f64>,

    #[rust(true)]
    show_values: bool,

    #[rust(60.0)]
    left_margin: f64,

    #[rust(30.0)]
    bottom_margin: f64,

    #[rust(60.0)]
    right_margin: f64,

    #[rust(30.0)]
    top_margin: f64,
}

impl Widget for HeatmapChart {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.data.is_empty() {
            self.update_plot_area(rect);
            self.draw_cells(cx);
            self.draw_labels(cx);
            self.draw_colorbar(cx, rect);
        }

        DrawStep::done()
    }
}

impl HeatmapChart {
    pub fn set_data(&mut self, data: Vec<Vec<f64>>) {
        self.data = data;
    }

    pub fn set_x_labels(&mut self, labels: Vec<String>) {
        self.x_labels = Some(labels);
    }

    pub fn set_y_labels(&mut self, labels: Vec<String>) {
        self.y_labels = Some(labels);
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_colormap(&mut self, colormap: Colormap) {
        self.colormap = colormap;
    }

    pub fn set_vmin(&mut self, vmin: f64) {
        self.vmin = Some(vmin);
    }

    pub fn set_vmax(&mut self, vmax: f64) {
        self.vmax = Some(vmax);
    }

    pub fn set_show_values(&mut self, show: bool) {
        self.show_values = show;
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.x_labels = None;
        self.y_labels = None;
    }

    fn update_plot_area(&mut self, rect: Rect) {
        self.plot_area = PlotArea::new(
            rect.pos.x + self.left_margin,
            rect.pos.y + self.top_margin,
            rect.pos.x + rect.size.x - self.right_margin,
            rect.pos.y + rect.size.y - self.bottom_margin,
        );
    }

    fn get_value_range(&self) -> (f64, f64) {
        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for row in &self.data {
            for &val in row {
                min = min.min(val);
                max = max.max(val);
            }
        }

        (self.vmin.unwrap_or(min), self.vmax.unwrap_or(max))
    }

    fn draw_cells(&mut self, cx: &mut Cx2d) {
        let rows = self.data.len();
        if rows == 0 {
            return;
        }
        let cols = self.data[0].len();
        if cols == 0 {
            return;
        }

        let (vmin, vmax) = self.get_value_range();
        let range = (vmax - vmin).max(1e-10);

        let cell_width = self.plot_area.width() / cols as f64;
        let cell_height = self.plot_area.height() / rows as f64;

        for (row_idx, row) in self.data.iter().enumerate() {
            for (col_idx, &value) in row.iter().enumerate() {
                let t = (value - vmin) / range;
                let color = self.colormap.sample(t);
                self.draw_bar.color = color;

                let x = self.plot_area.left + col_idx as f64 * cell_width;
                let y = self.plot_area.top + row_idx as f64 * cell_height;

                let rect = Rect {
                    pos: dvec2(x, y),
                    size: dvec2(cell_width, cell_height),
                };
                self.draw_bar.draw_bar(cx, rect);

                // Draw value text in cell
                if self.show_values {
                    let text_color = if t > 0.5 {
                        vec4(0.0, 0.0, 0.0, 1.0)
                    } else {
                        vec4(1.0, 1.0, 1.0, 1.0)
                    };
                    self.label.set_color(text_color);
                    let label = format!("{:.1}", value);
                    self.label.draw_at(
                        cx,
                        dvec2(x + cell_width / 2.0, y + cell_height / 2.0),
                        &label,
                        TextAnchor::Center,
                    );
                }
            }
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));

        let rows = self.data.len();
        let cols = if rows > 0 { self.data[0].len() } else { 0 };

        let cell_width = self.plot_area.width() / cols.max(1) as f64;
        let cell_height = self.plot_area.height() / rows.max(1) as f64;

        // X labels (column labels)
        if let Some(ref labels) = self.x_labels {
            for (i, label) in labels.iter().enumerate().take(cols) {
                let x = self.plot_area.left + (i as f64 + 0.5) * cell_width;
                let y = self.plot_area.bottom + 5.0;
                self.label.draw_at(cx, dvec2(x, y), label, TextAnchor::TopCenter);
            }
        }

        // Y labels (row labels)
        if let Some(ref labels) = self.y_labels {
            for (i, label) in labels.iter().enumerate().take(rows) {
                let x = self.plot_area.left - 5.0;
                let y = self.plot_area.top + (i as f64 + 0.5) * cell_height;
                self.label.draw_at(cx, dvec2(x, y), label, TextAnchor::MiddleRight);
            }
        }

        // Title
        if !self.title.is_empty() {
            let center_x = (self.plot_area.left + self.plot_area.right) / 2.0;
            self.label.draw_at(cx, dvec2(center_x, self.plot_area.top - 10.0), &self.title, TextAnchor::BottomCenter);
        }
    }

    fn draw_colorbar(&mut self, cx: &mut Cx2d, rect: Rect) {
        let bar_width = 15.0;
        let bar_x = rect.pos.x + rect.size.x - self.right_margin + 10.0;
        let bar_top = self.plot_area.top;
        let bar_height = self.plot_area.height();

        // Draw colorbar gradient
        let steps = 50;
        let step_height = bar_height / steps as f64;

        for i in 0..steps {
            let t = 1.0 - i as f64 / steps as f64;
            let color = self.colormap.sample(t);
            self.draw_bar.color = color;

            let rect = Rect {
                pos: dvec2(bar_x, bar_top + i as f64 * step_height),
                size: dvec2(bar_width, step_height + 1.0),
            };
            self.draw_bar.draw_bar(cx, rect);
        }

        // Draw colorbar labels
        let (vmin, vmax) = self.get_value_range();
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));

        let label_max = format!("{:.1}", vmax);
        let label_min = format!("{:.1}", vmin);
        let label_mid = format!("{:.1}", (vmin + vmax) / 2.0);

        self.label.draw_at(cx, dvec2(bar_x + bar_width + 3.0, bar_top), &label_max, TextAnchor::MiddleLeft);
        self.label.draw_at(cx, dvec2(bar_x + bar_width + 3.0, bar_top + bar_height / 2.0), &label_mid, TextAnchor::MiddleLeft);
        self.label.draw_at(cx, dvec2(bar_x + bar_width + 3.0, bar_top + bar_height), &label_min, TextAnchor::MiddleLeft);
    }
}

impl HeatmapChartRef {
    pub fn set_data(&self, data: Vec<Vec<f64>>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_data(data);
        }
    }

    pub fn set_x_labels(&self, labels: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_x_labels(labels);
        }
    }

    pub fn set_y_labels(&self, labels: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_y_labels(labels);
        }
    }

    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_title(title);
        }
    }

    pub fn set_colormap(&self, colormap: Colormap) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_colormap(colormap);
        }
    }

    pub fn set_show_values(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_show_values(show);
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
// ViolinPlot Widget
// =============================================================================

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

// =============================================================================
// PolarPlot Widget
// =============================================================================

#[derive(Clone, Debug)]
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
// ContourPlot Widget
// =============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct ContourPlot {
    #[deref] #[live] view: View,
    #[live] draw_line: DrawPlotLine,
    #[live] draw_fill: DrawPlotFill,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] data: Vec<Vec<f64>>,
    #[rust] x_range: (f64, f64),
    #[rust] y_range: (f64, f64),
    #[rust] filled: bool,
    #[rust] colormap: Colormap,
    #[rust] n_levels: usize,
    #[rust] plot_area: PlotArea,
    #[live(50.0)] left_margin: f64,
    #[live(30.0)] right_margin: f64,
    #[live(30.0)] top_margin: f64,
    #[live(50.0)] bottom_margin: f64,
}

impl Widget for ContourPlot {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) { self.view.handle_event(cx, event, scope); }
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.data.is_empty() {
            self.plot_area = PlotArea::new(rect.pos.x + self.left_margin, rect.pos.y + self.top_margin,
                rect.pos.x + rect.size.x - self.right_margin, rect.pos.y + rect.size.y - self.bottom_margin);
            self.draw_contours(cx);
            self.draw_axis_labels(cx);
            self.draw_labels(cx);
        }
        DrawStep::done()
    }
}

impl ContourPlot {
    pub fn set_title(&mut self, title: impl Into<String>) { self.title = title.into(); }
    pub fn set_data(&mut self, data: Vec<Vec<f64>>) { self.data = data; }
    pub fn set_x_range(&mut self, min: f64, max: f64) { self.x_range = (min, max); }
    pub fn set_y_range(&mut self, min: f64, max: f64) { self.y_range = (min, max); }
    pub fn set_filled(&mut self, filled: bool) { self.filled = filled; }
    pub fn set_colormap(&mut self, colormap: Colormap) { self.colormap = colormap; }
    pub fn set_n_levels(&mut self, n: usize) { self.n_levels = n; }
    pub fn clear(&mut self) { self.data.clear(); self.n_levels = 0; }

    fn draw_contours(&mut self, cx: &mut Cx2d) {
        let rows = self.data.len(); if rows < 2 { return; }
        let cols = self.data[0].len(); if cols < 2 { return; }
        let (mut v_min, mut v_max) = (f64::MAX, f64::MIN);
        for row in &self.data { for &v in row { v_min = v_min.min(v); v_max = v_max.max(v); } }
        let v_range = (v_max - v_min).max(1e-10);
        let cell_w = self.plot_area.width() / (cols - 1) as f64;
        let cell_h = self.plot_area.height() / (rows - 1) as f64;

        if self.filled {
            for row in 0..rows - 1 {
                for col in 0..cols - 1 {
                    let avg = (self.data[row][col] + self.data[row][col + 1] + self.data[row + 1][col] + self.data[row + 1][col + 1]) / 4.0;
                    self.draw_fill.color = self.colormap.sample((avg - v_min) / v_range);
                    self.draw_fill.draw_abs(cx, Rect { pos: dvec2(self.plot_area.left + col as f64 * cell_w, self.plot_area.top + row as f64 * cell_h), size: dvec2(cell_w, cell_h) });
                }
            }
        }

        let n_levels = if self.n_levels > 0 { self.n_levels } else { 10 };
        for lvl in 1..=n_levels {
            let level = v_min + lvl as f64 * (v_max - v_min) / (n_levels + 1) as f64;
            self.draw_line.color = if self.filled { vec4(0.2, 0.2, 0.2, 0.8) } else { self.colormap.sample((level - v_min) / v_range) };
            for row in 0..rows - 1 {
                for col in 0..cols - 1 {
                    let (v00, v10, v01, v11) = (self.data[row][col], self.data[row][col + 1], self.data[row + 1][col], self.data[row + 1][col + 1]);
                    let case = ((v00 >= level) as u8) | (((v10 >= level) as u8) << 1) | (((v01 >= level) as u8) << 2) | (((v11 >= level) as u8) << 3);
                    if case == 0 || case == 15 { continue; }
                    let x0 = self.plot_area.left + col as f64 * cell_w;
                    let y0 = self.plot_area.top + row as f64 * cell_h;
                    let interp = |a: f64, b: f64| if (b - a).abs() < 1e-10 { 0.5 } else { (level - a) / (b - a) };
                    let (tx, bx, ly, ry) = (x0 + interp(v00, v10) * cell_w, x0 + interp(v01, v11) * cell_w, y0 + interp(v00, v01) * cell_h, y0 + interp(v10, v11) * cell_h);
                    match case {
                        1 | 14 => self.draw_line.draw_line(cx, dvec2(x0, ly), dvec2(tx, y0), 1.5),
                        2 | 13 => self.draw_line.draw_line(cx, dvec2(tx, y0), dvec2(x0 + cell_w, ry), 1.5),
                        3 | 12 => self.draw_line.draw_line(cx, dvec2(x0, ly), dvec2(x0 + cell_w, ry), 1.5),
                        4 | 11 => self.draw_line.draw_line(cx, dvec2(x0, ly), dvec2(bx, y0 + cell_h), 1.5),
                        6 | 9 => self.draw_line.draw_line(cx, dvec2(tx, y0), dvec2(bx, y0 + cell_h), 1.5),
                        7 | 8 => self.draw_line.draw_line(cx, dvec2(bx, y0 + cell_h), dvec2(x0 + cell_w, ry), 1.5),
                        5 => { self.draw_line.draw_line(cx, dvec2(x0, ly), dvec2(tx, y0), 1.5); self.draw_line.draw_line(cx, dvec2(bx, y0 + cell_h), dvec2(x0 + cell_w, ry), 1.5); }
                        10 => { self.draw_line.draw_line(cx, dvec2(tx, y0), dvec2(x0 + cell_w, ry), 1.5); self.draw_line.draw_line(cx, dvec2(x0, ly), dvec2(bx, y0 + cell_h), 1.5); }
                        _ => {}
                    }
                }
            }
        }

        self.draw_line.color = vec4(0.3, 0.3, 0.3, 1.0);
        self.draw_line.draw_line(cx, dvec2(self.plot_area.left, self.plot_area.bottom), dvec2(self.plot_area.right, self.plot_area.bottom), 1.0);
        self.draw_line.draw_line(cx, dvec2(self.plot_area.left, self.plot_area.bottom), dvec2(self.plot_area.left, self.plot_area.top), 1.0);
    }

    fn draw_axis_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.4, 0.4, 0.4, 1.0));
        let n_ticks = 5;
        // X-axis ticks
        for i in 0..=n_ticks {
            let frac = i as f64 / n_ticks as f64;
            let x_pos = self.plot_area.left + frac * self.plot_area.width();
            let val = self.x_range.0 + frac * (self.x_range.1 - self.x_range.0);
            let txt = format!("{:.1}", val);
            self.label.draw_at(cx, dvec2(x_pos, self.plot_area.bottom + 12.0), &txt, TextAnchor::Center);
            // Tick mark
            self.draw_line.color = vec4(0.4, 0.4, 0.4, 0.6);
            self.draw_line.draw_line(cx, dvec2(x_pos, self.plot_area.bottom), dvec2(x_pos, self.plot_area.bottom + 4.0), 1.0);
        }
        // Y-axis ticks
        for i in 0..=n_ticks {
            let frac = i as f64 / n_ticks as f64;
            let y_pos = self.plot_area.bottom - frac * self.plot_area.height();
            let val = self.y_range.0 + frac * (self.y_range.1 - self.y_range.0);
            let txt = format!("{:.1}", val);
            self.label.draw_at(cx, dvec2(self.plot_area.left - 8.0, y_pos), &txt, TextAnchor::MiddleRight);
            self.draw_line.color = vec4(0.4, 0.4, 0.4, 0.6);
            self.draw_line.draw_line(cx, dvec2(self.plot_area.left - 4.0, y_pos), dvec2(self.plot_area.left, y_pos), 1.0);
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
        if !self.title.is_empty() {
            self.label.draw_at(cx, dvec2((self.plot_area.left + self.plot_area.right) / 2.0, self.plot_area.top - 15.0), &self.title, TextAnchor::Center);
        }
    }
}

impl ContourPlotRef {
    pub fn set_title(&self, title: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); } }
    pub fn set_data(&self, data: Vec<Vec<f64>>) { if let Some(mut inner) = self.borrow_mut() { inner.set_data(data); } }
    pub fn set_x_range(&self, min: f64, max: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_x_range(min, max); } }
    pub fn set_y_range(&self, min: f64, max: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_y_range(min, max); } }
    pub fn set_filled(&self, filled: bool) { if let Some(mut inner) = self.borrow_mut() { inner.set_filled(filled); } }
    pub fn set_colormap(&self, colormap: Colormap) { if let Some(mut inner) = self.borrow_mut() { inner.set_colormap(colormap); } }
    pub fn set_n_levels(&self, n: usize) { if let Some(mut inner) = self.borrow_mut() { inner.set_n_levels(n); } }
    pub fn clear(&self) { if let Some(mut inner) = self.borrow_mut() { inner.clear(); } }
    pub fn redraw(&self, cx: &mut Cx) { if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); } }
}

// =============================================================================
// QuiverPlot Widget (Vector Field)
// =============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct QuiverPlot {
    #[deref] #[live] view: View,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] x: Vec<f64>,
    #[rust] y: Vec<f64>,
    #[rust] u: Vec<f64>,
    #[rust] v: Vec<f64>,
    #[rust] scale: f64,
    #[rust] arrow_color: Vec4,
    #[rust] plot_area: PlotArea,
    #[live(50.0)] left_margin: f64,
    #[live(30.0)] right_margin: f64,
    #[live(30.0)] top_margin: f64,
    #[live(50.0)] bottom_margin: f64,
}

impl Widget for QuiverPlot {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) { self.view.handle_event(cx, event, scope); }
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.x.is_empty() {
            self.plot_area = PlotArea::new(rect.pos.x + self.left_margin, rect.pos.y + self.top_margin,
                rect.pos.x + rect.size.x - self.right_margin, rect.pos.y + rect.size.y - self.bottom_margin);
            self.draw_arrows(cx);
            self.draw_labels(cx);
        }
        DrawStep::done()
    }
}

impl QuiverPlot {
    pub fn set_title(&mut self, title: impl Into<String>) { self.title = title.into(); }
    pub fn set_data(&mut self, x: Vec<f64>, y: Vec<f64>, u: Vec<f64>, v: Vec<f64>) { self.x = x; self.y = y; self.u = u; self.v = v; }
    pub fn set_scale(&mut self, scale: f64) { self.scale = scale; }
    pub fn set_color(&mut self, color: Vec4) { self.arrow_color = color; }
    pub fn clear(&mut self) { self.x.clear(); self.y.clear(); self.u.clear(); self.v.clear(); }

    fn draw_arrows(&mut self, cx: &mut Cx2d) {
        let n = self.x.len().min(self.y.len()).min(self.u.len()).min(self.v.len());
        if n == 0 { return; }
        let (x_min, x_max) = (self.x.iter().cloned().fold(f64::MAX, f64::min), self.x.iter().cloned().fold(f64::MIN, f64::max));
        let (y_min, y_max) = (self.y.iter().cloned().fold(f64::MAX, f64::min), self.y.iter().cloned().fold(f64::MIN, f64::max));
        let (xr, yr) = ((x_max - x_min).max(1e-10), (y_max - y_min).max(1e-10));
        let max_mag = self.u.iter().zip(self.v.iter()).map(|(&u, &v)| (u * u + v * v).sqrt()).fold(0.0f64, f64::max);
        let scale = if self.scale > 0.0 { self.scale } else if max_mag > 0.0 { 0.1 * self.plot_area.width().min(self.plot_area.height()) / max_mag } else { 1.0 };
        self.draw_line.color = if self.arrow_color.w > 0.0 { self.arrow_color } else { vec4(0.12, 0.47, 0.71, 1.0) };

        self.draw_line.draw_line(cx, dvec2(self.plot_area.left, self.plot_area.bottom), dvec2(self.plot_area.right, self.plot_area.bottom), 1.0);
        self.draw_line.draw_line(cx, dvec2(self.plot_area.left, self.plot_area.bottom), dvec2(self.plot_area.left, self.plot_area.top), 1.0);

        let color = self.draw_line.color;
        for i in 0..n {
            let px = self.plot_area.left + (self.x[i] - x_min) / xr * self.plot_area.width();
            let py = self.plot_area.bottom - (self.y[i] - y_min) / yr * self.plot_area.height();
            let (dx, dy) = (self.u[i] * scale, -self.v[i] * scale);
            let (p1, p2) = (dvec2(px, py), dvec2(px + dx, py + dy));
            self.draw_line.color = color;
            self.draw_line.draw_line(cx, p1, p2, 1.5);
            let len = ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt();
            if len > 1.0 {
                let (dirx, diry) = ((p2.x - p1.x) / len, (p2.y - p1.y) / len);
                let (perpx, perpy) = (-diry, dirx);
                let (al, aa) = (5.0, 0.4);
                self.draw_line.draw_line(cx, p2, dvec2(p2.x - dirx * al + perpx * al * aa, p2.y - diry * al + perpy * al * aa), 1.5);
                self.draw_line.draw_line(cx, p2, dvec2(p2.x - dirx * al - perpx * al * aa, p2.y - diry * al - perpy * al * aa), 1.5);
            }
        }
    }

    fn draw_labels(&mut self, cx: &mut Cx2d) {
        self.label.set_color(vec4(0.3, 0.3, 0.3, 1.0));
        if !self.title.is_empty() {
            self.label.draw_at(cx, dvec2((self.plot_area.left + self.plot_area.right) / 2.0, self.plot_area.top - 15.0), &self.title, TextAnchor::Center);
        }
    }
}

impl QuiverPlotRef {
    pub fn set_title(&self, title: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); } }
    pub fn set_data(&self, x: Vec<f64>, y: Vec<f64>, u: Vec<f64>, v: Vec<f64>) { if let Some(mut inner) = self.borrow_mut() { inner.set_data(x, y, u, v); } }
    pub fn set_scale(&self, scale: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_scale(scale); } }
    pub fn set_color(&self, color: Vec4) { if let Some(mut inner) = self.borrow_mut() { inner.set_color(color); } }
    pub fn clear(&self) { if let Some(mut inner) = self.borrow_mut() { inner.clear(); } }
    pub fn redraw(&self, cx: &mut Cx) { if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); } }
}

// =============================================================================
// 3D Infrastructure
// =============================================================================

/// Camera/view settings for 3D plots
#[derive(Clone, Debug, Default)]
pub struct View3D {
    pub azimuth: f64,     // Horizontal rotation (degrees)
    pub elevation: f64,   // Vertical rotation (degrees)
    pub distance: f64,    // Distance from origin
}

impl View3D {
    pub fn new() -> Self {
        Self {
            azimuth: -60.0,
            elevation: 30.0,
            distance: 3.0,
        }
    }
}

impl View3D {
    /// Project 3D point to 2D screen coordinates
    pub fn project(&self, x: f64, y: f64, z: f64) -> (f64, f64) {
        let az = self.azimuth.to_radians();
        let el = self.elevation.to_radians();

        // Rotate around Z axis (azimuth)
        let x1 = x * az.cos() - y * az.sin();
        let y1 = x * az.sin() + y * az.cos();
        let z1 = z;

        // Rotate around X axis (elevation)
        let x2 = x1;
        let y2 = y1 * el.cos() - z1 * el.sin();
        let z2 = y1 * el.sin() + z1 * el.cos();

        // Simple perspective projection
        let perspective = self.distance / (self.distance + y2 + 2.0);
        let screen_x = x2 * perspective;
        let screen_y = z2 * perspective;

        (screen_x, screen_y)
    }

    /// Get depth for z-sorting (larger = further away)
    pub fn depth(&self, x: f64, y: f64, z: f64) -> f64 {
        let az = self.azimuth.to_radians();
        let el = self.elevation.to_radians();

        let x1 = x * az.cos() - y * az.sin();
        let y1 = x * az.sin() + y * az.cos();
        let z1 = z;

        let y2 = y1 * el.cos() - z1 * el.sin();
        y2
    }
}

// =============================================================================
// Surface3D Widget
// =============================================================================
//
// Interactive 3D surface plot with mouse controls:
// - Click and drag horizontally: rotate view (azimuth)
// - Click and drag vertically: tilt view (elevation)
// - Scroll wheel: zoom in/out
//
// Supports multiple charts via per-chart state tracking: each chart_id gets
// its own data, view angles, zoom, and drag state stored in a HashMap.
//
// Implementation note: Uses manual coordinate-based hit testing instead of
// Makepad's event.hits() system because Surface3D is used as a render helper
// inside A2uiSurface (not in the widget tree), so area-based hit testing fails.
// =============================================================================

/// Per-chart instance state for multi-chart support
#[derive(Clone, Debug)]
pub struct Surface3DChart {
    pub title: String,
    pub z_data: Vec<Vec<f64>>,
    pub x_range: (f64, f64),
    pub y_range: (f64, f64),
    pub z_range: (f64, f64),
    pub view3d: View3D,
    pub colormap: Colormap,
    pub show_wireframe: bool,
    pub show_surface: bool,
    pub zoom: f64,
    pub drag_start: Option<DVec2>,
    pub start_azimuth: f64,
    pub start_elevation: f64,
    pub hit_rect: Rect,
}

impl Default for Surface3DChart {
    fn default() -> Self {
        Self {
            title: String::new(),
            z_data: Vec::new(),
            x_range: (0.0, 1.0),
            y_range: (0.0, 1.0),
            z_range: (0.0, 1.0),
            view3d: View3D::new(),
            colormap: Colormap::default(),
            show_wireframe: true,
            show_surface: false,
            zoom: 1.0,
            drag_start: None,
            start_azimuth: 0.0,
            start_elevation: 0.0,
            hit_rect: Rect::default(),
        }
    }
}

impl Surface3DChart {
    pub fn set_data(&mut self, z: Vec<Vec<f64>>) {
        if z.is_empty() || z[0].is_empty() { return; }
        let mut z_min = f64::MAX;
        let mut z_max = f64::MIN;
        for row in &z {
            for &val in row {
                if val < z_min { z_min = val; }
                if val > z_max { z_max = val; }
            }
        }
        self.x_range = (0.0, (z[0].len() - 1) as f64);
        self.y_range = (0.0, (z.len() - 1) as f64);
        self.z_range = (z_min, z_max);
        self.z_data = z;
    }

    fn normalize_z(&self, z: f64) -> f64 {
        if self.z_range.1 == self.z_range.0 { return 0.5; }
        (z - self.z_range.0) / (self.z_range.1 - self.z_range.0)
    }
}

/// Wrapper for HashMap to work with Makepad's #[rust] derive
#[derive(Clone, Debug, Default)]
pub struct Surface3DCharts {
    pub map: std::collections::HashMap<String, Surface3DChart>,
}

#[derive(Live, LiveHook, Widget)]
pub struct Surface3D {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    // Legacy single-chart fields (used when no chart_id is provided)
    #[rust] title: String,
    #[rust] z_data: Vec<Vec<f64>>,
    #[rust] x_range: (f64, f64),
    #[rust] y_range: (f64, f64),
    #[rust] z_range: (f64, f64),
    #[rust] view3d: View3D,
    #[rust] colormap: Colormap,
    #[rust] show_wireframe: bool,
    #[rust] show_surface: bool,
    #[rust] zoom: f64,
    #[rust] drag_start: Option<DVec2>,
    #[rust] start_azimuth: f64,
    #[rust] start_elevation: f64,
    #[rust] hit_rect: Rect,
    // Multi-chart support: per-chart state keyed by chart component ID
    #[rust] charts: Surface3DCharts,
    // Which chart is currently being dragged (by chart_id)
    #[rust] active_drag_id: String,
}

impl Surface3D {
    pub fn set_title(&mut self, title: impl Into<String>) { self.title = title.into(); }

    pub fn set_data(&mut self, z: Vec<Vec<f64>>) {
        if z.is_empty() || z[0].is_empty() { return; }

        // Calculate z range
        let mut z_min = f64::MAX;
        let mut z_max = f64::MIN;
        for row in &z {
            for &val in row {
                if val < z_min { z_min = val; }
                if val > z_max { z_max = val; }
            }
        }

        self.z_data = z;
        self.z_range = (z_min, z_max);
        self.x_range = (0.0, (self.z_data[0].len() - 1) as f64);
        self.y_range = (0.0, (self.z_data.len() - 1) as f64);
    }

    pub fn set_x_range(&mut self, min: f64, max: f64) { self.x_range = (min, max); }
    pub fn set_y_range(&mut self, min: f64, max: f64) { self.y_range = (min, max); }
    pub fn set_view(&mut self, view: View3D) { self.view3d = view; }
    pub fn set_azimuth(&mut self, az: f64) { self.view3d.azimuth = az; }
    pub fn set_elevation(&mut self, el: f64) { self.view3d.elevation = el; }
    pub fn set_colormap(&mut self, cm: Colormap) { self.colormap = cm; }
    pub fn set_wireframe(&mut self, show: bool) { self.show_wireframe = show; }
    pub fn set_surface(&mut self, show: bool) { self.show_surface = show; }
    pub fn clear(&mut self) { self.z_data.clear(); }
    pub fn redraw(&mut self, cx: &mut Cx) { self.view.redraw(cx); }

    fn normalize_z(&self, z: f64) -> f64 {
        if self.z_range.1 == self.z_range.0 { return 0.5; }
        (z - self.z_range.0) / (self.z_range.1 - self.z_range.0)
    }

    // ── Multi-chart API ──────────────────────────────────────────────

    /// Get or create a per-chart instance by ID. Preserves interactive state
    /// (view angles, zoom) across redraws while allowing data updates.
    pub fn get_chart_mut(&mut self, chart_id: &str) -> &mut Surface3DChart {
        self.charts.map.entry(chart_id.to_string()).or_insert_with(Surface3DChart::default)
    }

    /// Draw a specific chart instance using its per-chart state.
    pub fn draw_chart_instance(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk, chart_id: &str) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        // Store hit_rect in the per-chart state
        if let Some(chart) = self.charts.map.get_mut(chart_id) {
            chart.hit_rect = rect;
        }

        // Get chart data (need to clone to avoid borrow issues with self.draw_*)
        let chart_data = match self.charts.map.get(chart_id) {
            Some(c) if !c.z_data.is_empty() && rect.size.x > 0.0 && rect.size.y > 0.0 => {
                Some((
                    c.z_data.clone(),
                    c.x_range, c.y_range, c.z_range,
                    c.view3d.clone(),
                    c.colormap.clone(),
                    c.show_wireframe, c.show_surface,
                    c.zoom,
                    c.title.clone(),
                ))
            }
            _ => None,
        };

        if let Some((z_data, x_range, y_range, z_range, view3d, colormap, show_wireframe, show_surface, zoom, title)) = chart_data {
            let cx_center = rect.pos.x + rect.size.x * 0.5;
            let cy_center = rect.pos.y + rect.size.y * 0.5;
            let zoom_val = if zoom == 0.0 { 1.0 } else { zoom };
            let scale = rect.size.x.min(rect.size.y) * 0.35 * zoom_val;

            let rows = z_data.len();
            let cols = z_data[0].len();

            let x_scale = 2.0 / (cols - 1).max(1) as f64;
            let y_scale = 2.0 / (rows - 1).max(1) as f64;
            let z_scale = if z_range.1 != z_range.0 { 1.5 / (z_range.1 - z_range.0) } else { 1.0 };
            let z_offset = (z_range.0 + z_range.1) * 0.5;

            let normalize_z = |z: f64| -> f64 {
                if z_range.1 == z_range.0 { 0.5 } else { (z - z_range.0) / (z_range.1 - z_range.0) }
            };

            // Draw filled surface quads with depth sorting
            if show_surface {
                let mut quads: Vec<(f64, usize, usize, Vec4)> = Vec::new();
                for i in 0..rows-1 {
                    for j in 0..cols-1 {
                        let avg_z = (z_data[i][j] + z_data[i+1][j] + z_data[i][j+1] + z_data[i+1][j+1]) * 0.25;
                        let t = normalize_z(avg_z);
                        let color = colormap.sample(t);
                        let cx_q = (j as f64 + 0.5) * x_scale - 1.0;
                        let cy_q = (i as f64 + 0.5) * y_scale - 1.0;
                        let cz_q = (avg_z - z_offset) * z_scale;
                        let depth = view3d.depth(cx_q, cy_q, cz_q);
                        quads.push((depth, i, j, color));
                    }
                }
                quads.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

                for (_, i, j, color) in quads {
                    let corners = [(j, i), (j+1, i), (j+1, i+1), (j, i+1)];
                    let mut pts: Vec<DVec2> = Vec::new();
                    for &(cj, ci) in &corners {
                        let x = cj as f64 * x_scale - 1.0;
                        let y = ci as f64 * y_scale - 1.0;
                        let z = (z_data[ci][cj] - z_offset) * z_scale;
                        let (sx, sy) = view3d.project(x, y, z);
                        pts.push(dvec2(cx_center + sx * scale, cy_center - sy * scale));
                    }
                    self.draw_fill.color = color;
                    let min_x = pts.iter().map(|p| p.x).fold(f64::MAX, f64::min);
                    let max_x = pts.iter().map(|p| p.x).fold(f64::MIN, f64::max);
                    let min_y = pts.iter().map(|p| p.y).fold(f64::MAX, f64::min);
                    let max_y = pts.iter().map(|p| p.y).fold(f64::MIN, f64::max);
                    self.draw_fill.draw_abs(cx, Rect {
                        pos: dvec2(min_x, min_y),
                        size: dvec2(max_x - min_x + 1.0, max_y - min_y + 1.0),
                    });
                }
            }

            // Draw wireframe
            if show_wireframe {
                let wire_color = if show_surface { vec4(0.0, 0.0, 0.0, 0.5) } else { vec4(0.2, 0.4, 0.8, 1.0) };
                self.draw_line.color = wire_color;
                for i in 0..rows-1 {
                    for j in 0..cols-1 {
                        let corners = [(j, i), (j+1, i), (j+1, i+1), (j, i+1), (j, i)];
                        for k in 0..4 {
                            let (j0, i0) = corners[k];
                            let (j1, i1) = corners[k+1];
                            let x0 = j0 as f64 * x_scale - 1.0;
                            let y0 = i0 as f64 * y_scale - 1.0;
                            let z0 = (z_data[i0][j0] - z_offset) * z_scale;
                            let (sx0, sy0) = view3d.project(x0, y0, z0);
                            let x1 = j1 as f64 * x_scale - 1.0;
                            let y1 = i1 as f64 * y_scale - 1.0;
                            let z1 = (z_data[i1][j1] - z_offset) * z_scale;
                            let (sx1, sy1) = view3d.project(x1, y1, z1);
                            self.draw_line.draw_line(cx,
                                dvec2(cx_center + sx0 * scale, cy_center - sy0 * scale),
                                dvec2(cx_center + sx1 * scale, cy_center - sy1 * scale),
                                1.0);
                        }
                    }
                }
            }

            // Draw 3D axes
            self.draw_line.color = vec4(0.5, 0.5, 0.5, 0.8);
            let axis_len = 1.2;
            let axes = [
                ((-axis_len, 0.0, 0.0), (axis_len, 0.0, 0.0)),
                ((0.0, -axis_len, 0.0), (0.0, axis_len, 0.0)),
                ((0.0, 0.0, -axis_len * 0.5), (0.0, 0.0, axis_len)),
            ];
            for ((x0, y0, z0), (x1, y1, z1)) in axes {
                let (sx0, sy0) = view3d.project(x0, y0, z0);
                let (sx1, sy1) = view3d.project(x1, y1, z1);
                self.draw_line.draw_line(cx,
                    dvec2(cx_center + sx0 * scale, cy_center - sy0 * scale),
                    dvec2(cx_center + sx1 * scale, cy_center - sy1 * scale),
                    1.0);
            }

            // Draw title
            if !title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + 10.0, rect.pos.y + 5.0), &title, TextAnchor::TopLeft);
            }
        }

        DrawStep::done()
    }

    /// Handle events for all chart instances (multi-chart drag support).
    pub fn handle_multi_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::MouseDown(me) => {
                // Find which chart was clicked
                for (chart_id, chart) in self.charts.map.iter_mut() {
                    let r = chart.hit_rect;
                    if r.size.x > 0.0 && r.size.y > 0.0 && r.contains(me.abs) {
                        chart.drag_start = Some(me.abs);
                        chart.start_azimuth = chart.view3d.azimuth;
                        chart.start_elevation = chart.view3d.elevation;
                        self.active_drag_id = chart_id.clone();
                        return;
                    }
                }
            }
            Event::MouseMove(me) => {
                if !self.active_drag_id.is_empty() {
                    if let Some(chart) = self.charts.map.get_mut(&self.active_drag_id) {
                        if let Some(start) = chart.drag_start {
                            let delta = me.abs - start;
                            chart.view3d.azimuth = chart.start_azimuth + delta.x * 0.5;
                            chart.view3d.elevation = (chart.start_elevation - delta.y * 0.5).clamp(-89.0, 89.0);
                            cx.redraw_all();
                        }
                    }
                }
            }
            Event::MouseUp(_) => {
                if !self.active_drag_id.is_empty() {
                    if let Some(chart) = self.charts.map.get_mut(&self.active_drag_id) {
                        chart.drag_start = None;
                    }
                    self.active_drag_id.clear();
                }
            }
            Event::Scroll(se) => {
                for (_chart_id, chart) in self.charts.map.iter_mut() {
                    let r = chart.hit_rect;
                    if r.size.x > 0.0 && r.size.y > 0.0 && r.contains(se.abs) {
                        if chart.zoom == 0.0 { chart.zoom = 1.0; }
                        let zoom_delta = 1.0 + se.scroll.y * 0.001;
                        chart.zoom = (chart.zoom * zoom_delta).clamp(0.2, 5.0);
                        cx.redraw_all();
                        return;
                    }
                }
            }
            _ => {}
        }
    }
}

impl Widget for Surface3D {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        self.hit_rect = rect;  // Store for manual hit testing

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.z_data.is_empty() {
            // Initialize defaults
            if self.view3d.distance == 0.0 { self.view3d = View3D::new(); }
            if !self.show_wireframe && !self.show_surface { self.show_wireframe = true; }
            if self.zoom == 0.0 { self.zoom = 1.0; }

            let cx_center = rect.pos.x + rect.size.x * 0.5;
            let cy_center = rect.pos.y + rect.size.y * 0.5;
            let scale = rect.size.x.min(rect.size.y) * 0.35 * self.zoom;

            let rows = self.z_data.len();
            let cols = self.z_data[0].len();

            // Normalize coordinates to [-1, 1] range
            let x_scale = 2.0 / (cols - 1).max(1) as f64;
            let y_scale = 2.0 / (rows - 1).max(1) as f64;
            let z_scale = if self.z_range.1 != self.z_range.0 {
                1.5 / (self.z_range.1 - self.z_range.0)
            } else { 1.0 };
            let z_offset = (self.z_range.0 + self.z_range.1) * 0.5;

            // Draw filled surface quads with depth sorting (painter's algorithm)
            if self.show_surface {
                // Collect all quads with their depth for sorting
                let mut quads: Vec<(f64, usize, usize, Vec4)> = Vec::new();
                for i in 0..rows-1 {
                    for j in 0..cols-1 {
                        let avg_z = (self.z_data[i][j] + self.z_data[i+1][j] +
                                     self.z_data[i][j+1] + self.z_data[i+1][j+1]) * 0.25;
                        let t = self.normalize_z(avg_z);
                        let color = self.colormap.sample(t);

                        // Calculate center point for depth
                        let cx_q = (j as f64 + 0.5) * x_scale - 1.0;
                        let cy_q = (i as f64 + 0.5) * y_scale - 1.0;
                        let cz_q = (avg_z - z_offset) * z_scale;
                        let depth = self.view3d.depth(cx_q, cy_q, cz_q);

                        quads.push((depth, i, j, color));
                    }
                }

                // Sort by depth (back to front)
                quads.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

                // Draw quads in sorted order
                for (_, i, j, color) in quads {
                    // Get projected corners
                    let corners = [(j, i), (j+1, i), (j+1, i+1), (j, i+1)];
                    let mut pts: Vec<DVec2> = Vec::new();
                    for &(cj, ci) in &corners {
                        let x = cj as f64 * x_scale - 1.0;
                        let y = ci as f64 * y_scale - 1.0;
                        let z = (self.z_data[ci][cj] - z_offset) * z_scale;
                        let (sx, sy) = self.view3d.project(x, y, z);
                        pts.push(dvec2(cx_center + sx * scale, cy_center - sy * scale));
                    }

                    // Draw filled quad as two triangles using lines
                    self.draw_fill.color = color;
                    let min_x = pts.iter().map(|p| p.x).fold(f64::MAX, f64::min);
                    let max_x = pts.iter().map(|p| p.x).fold(f64::MIN, f64::max);
                    let min_y = pts.iter().map(|p| p.y).fold(f64::MAX, f64::min);
                    let max_y = pts.iter().map(|p| p.y).fold(f64::MIN, f64::max);
                    self.draw_fill.draw_abs(cx, Rect {
                        pos: dvec2(min_x, min_y),
                        size: dvec2(max_x - min_x + 1.0, max_y - min_y + 1.0),
                    });
                }
            }

            // Draw wireframe
            if self.show_wireframe {
                let wire_color = if self.show_surface { vec4(0.0, 0.0, 0.0, 0.5) } else { vec4(0.2, 0.4, 0.8, 1.0) };
                self.draw_line.color = wire_color;

                for i in 0..rows-1 {
                    for j in 0..cols-1 {
                        let corners = [(j, i), (j+1, i), (j+1, i+1), (j, i+1), (j, i)];
                        for k in 0..4 {
                            let (j0, i0) = corners[k];
                            let (j1, i1) = corners[k+1];

                            let x0 = j0 as f64 * x_scale - 1.0;
                            let y0 = i0 as f64 * y_scale - 1.0;
                            let z0 = (self.z_data[i0][j0] - z_offset) * z_scale;
                            let (sx0, sy0) = self.view3d.project(x0, y0, z0);

                            let x1 = j1 as f64 * x_scale - 1.0;
                            let y1 = i1 as f64 * y_scale - 1.0;
                            let z1 = (self.z_data[i1][j1] - z_offset) * z_scale;
                            let (sx1, sy1) = self.view3d.project(x1, y1, z1);

                            self.draw_line.draw_line(cx,
                                dvec2(cx_center + sx0 * scale, cy_center - sy0 * scale),
                                dvec2(cx_center + sx1 * scale, cy_center - sy1 * scale),
                                1.0);
                        }
                    }
                }
            }

            // Draw 3D axes
            self.draw_line.color = vec4(0.5, 0.5, 0.5, 0.8);
            let axis_len = 1.2;
            let axes = [
                ((-axis_len, 0.0, 0.0), (axis_len, 0.0, 0.0)),
                ((0.0, -axis_len, 0.0), (0.0, axis_len, 0.0)),
                ((0.0, 0.0, -axis_len * 0.5), (0.0, 0.0, axis_len)),
            ];
            for ((x0, y0, z0), (x1, y1, z1)) in axes {
                let (sx0, sy0) = self.view3d.project(x0, y0, z0);
                let (sx1, sy1) = self.view3d.project(x1, y1, z1);
                self.draw_line.draw_line(cx,
                    dvec2(cx_center + sx0 * scale, cy_center - sy0 * scale),
                    dvec2(cx_center + sx1 * scale, cy_center - sy1 * scale),
                    1.0);
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + 10.0, rect.pos.y + 5.0), &self.title, TextAnchor::TopLeft);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        // Handle multi-chart instances (used by A2uiSurface for multiple 3D charts)
        if !self.charts.map.is_empty() {
            self.handle_multi_event(cx, event);
            return;
        }

        // Legacy single-chart path (used when Surface3D is used standalone)
        let r = self.hit_rect;
        if r.size.x <= 0.0 || r.size.y <= 0.0 { return; }

        match event {
            Event::MouseDown(me) => {
                if r.contains(me.abs) {
                    self.drag_start = Some(me.abs);
                    self.start_azimuth = self.view3d.azimuth;
                    self.start_elevation = self.view3d.elevation;
                }
            }
            Event::MouseMove(me) => {
                if let Some(start) = self.drag_start {
                    let delta = me.abs - start;
                    self.view3d.azimuth = self.start_azimuth + delta.x * 0.5;
                    self.view3d.elevation = (self.start_elevation - delta.y * 0.5).clamp(-89.0, 89.0);
                    cx.redraw_all();
                }
            }
            Event::MouseUp(_) => {
                self.drag_start = None;
            }
            Event::Scroll(se) => {
                if r.contains(se.abs) {
                    if self.zoom == 0.0 { self.zoom = 1.0; }
                    let zoom_delta = 1.0 + se.scroll.y * 0.001;
                    self.zoom = (self.zoom * zoom_delta).clamp(0.2, 5.0);
                    cx.redraw_all();
                }
            }
            _ => {}
        }
    }
}

impl Surface3DRef {
    pub fn set_title(&self, title: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); } }
    pub fn set_data(&self, z: Vec<Vec<f64>>) { if let Some(mut inner) = self.borrow_mut() { inner.set_data(z); } }
    pub fn set_x_range(&self, min: f64, max: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_x_range(min, max); } }
    pub fn set_y_range(&self, min: f64, max: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_y_range(min, max); } }
    pub fn set_view(&self, view: View3D) { if let Some(mut inner) = self.borrow_mut() { inner.set_view(view); } }
    pub fn set_azimuth(&self, az: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_azimuth(az); } }
    pub fn set_elevation(&self, el: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_elevation(el); } }
    pub fn set_colormap(&self, cm: Colormap) { if let Some(mut inner) = self.borrow_mut() { inner.set_colormap(cm); } }
    pub fn set_wireframe(&self, show: bool) { if let Some(mut inner) = self.borrow_mut() { inner.set_wireframe(show); } }
    pub fn set_surface(&self, show: bool) { if let Some(mut inner) = self.borrow_mut() { inner.set_surface(show); } }
    pub fn clear(&self) { if let Some(mut inner) = self.borrow_mut() { inner.clear(); } }
    pub fn redraw(&self, cx: &mut Cx) { if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); } }
}

// =============================================================================
// Scatter3D Widget
// =============================================================================

#[derive(Clone, Debug)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub color: Option<Vec4>,
    pub size: Option<f64>,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, color: None, size: None }
    }
    pub fn with_color(mut self, c: Vec4) -> Self { self.color = Some(c); self }
    pub fn with_size(mut self, s: f64) -> Self { self.size = Some(s); self }
}

#[derive(Live, LiveHook, Widget)]
pub struct Scatter3D {
    #[deref] #[live] view: View,
    #[live] draw_point: DrawPlotPoint,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] points: Vec<Point3D>,
    #[rust] default_color: Vec4,
    #[rust] default_size: f64,
    #[rust] view3d: View3D,
    #[rust] x_range: (f64, f64),
    #[rust] y_range: (f64, f64),
    #[rust] z_range: (f64, f64),
    #[rust] zoom: f64,
    #[rust] drag_start: Option<DVec2>,
    #[rust] start_azimuth: f64,
    #[rust] start_elevation: f64,
}

impl Scatter3D {
    pub fn set_title(&mut self, title: impl Into<String>) { self.title = title.into(); }

    pub fn set_data(&mut self, x: Vec<f64>, y: Vec<f64>, z: Vec<f64>) {
        self.points.clear();
        let n = x.len().min(y.len()).min(z.len());
        for i in 0..n {
            self.points.push(Point3D::new(x[i], y[i], z[i]));
        }
        self.auto_range();
    }

    pub fn add_point(&mut self, p: Point3D) {
        self.points.push(p);
    }

    fn auto_range(&mut self) {
        if self.points.is_empty() { return; }
        let mut x_min = f64::MAX; let mut x_max = f64::MIN;
        let mut y_min = f64::MAX; let mut y_max = f64::MIN;
        let mut z_min = f64::MAX; let mut z_max = f64::MIN;
        for p in &self.points {
            if p.x < x_min { x_min = p.x; } if p.x > x_max { x_max = p.x; }
            if p.y < y_min { y_min = p.y; } if p.y > y_max { y_max = p.y; }
            if p.z < z_min { z_min = p.z; } if p.z > z_max { z_max = p.z; }
        }
        let pad_x = (x_max - x_min).max(0.1) * 0.1;
        let pad_y = (y_max - y_min).max(0.1) * 0.1;
        let pad_z = (z_max - z_min).max(0.1) * 0.1;
        self.x_range = (x_min - pad_x, x_max + pad_x);
        self.y_range = (y_min - pad_y, y_max + pad_y);
        self.z_range = (z_min - pad_z, z_max + pad_z);
    }

    pub fn set_color(&mut self, c: Vec4) { self.default_color = c; }
    pub fn set_point_size(&mut self, s: f64) { self.default_size = s; }
    pub fn set_view(&mut self, view: View3D) { self.view3d = view; }
    pub fn set_azimuth(&mut self, az: f64) { self.view3d.azimuth = az; }
    pub fn set_elevation(&mut self, el: f64) { self.view3d.elevation = el; }
    pub fn clear(&mut self) { self.points.clear(); }
    pub fn redraw(&mut self, cx: &mut Cx) { self.view.redraw(cx); }
}

impl Widget for Scatter3D {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.points.is_empty() {
            // Initialize defaults
            if self.view3d.distance == 0.0 { self.view3d = View3D::new(); }
            if self.default_size == 0.0 { self.default_size = 6.0; }
            if self.default_color == Vec4::default() { self.default_color = vec4(0.12, 0.47, 0.71, 1.0); }
            if self.zoom == 0.0 { self.zoom = 1.0; }

            let cx_center = rect.pos.x + rect.size.x * 0.5;
            let cy_center = rect.pos.y + rect.size.y * 0.5;
            let scale = rect.size.x.min(rect.size.y) * 0.35 * self.zoom;

            // Normalize to [-1, 1]
            let x_scale = if self.x_range.1 != self.x_range.0 { 2.0 / (self.x_range.1 - self.x_range.0) } else { 1.0 };
            let y_scale = if self.y_range.1 != self.y_range.0 { 2.0 / (self.y_range.1 - self.y_range.0) } else { 1.0 };
            let z_scale = if self.z_range.1 != self.z_range.0 { 2.0 / (self.z_range.1 - self.z_range.0) } else { 1.0 };
            let x_off = (self.x_range.0 + self.x_range.1) * 0.5;
            let y_off = (self.y_range.0 + self.y_range.1) * 0.5;
            let z_off = (self.z_range.0 + self.z_range.1) * 0.5;

            // Sort points back to front
            let mut sorted: Vec<(f64, usize)> = self.points.iter().enumerate().map(|(i, p)| {
                let x = (p.x - x_off) * x_scale;
                let y = (p.y - y_off) * y_scale;
                let z = (p.z - z_off) * z_scale;
                (self.view3d.depth(x, y, z), i)
            }).collect();
            sorted.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

            // Draw points
            for (_, idx) in sorted {
                let p = &self.points[idx];
                let x = (p.x - x_off) * x_scale;
                let y = (p.y - y_off) * y_scale;
                let z = (p.z - z_off) * z_scale;

                let (sx, sy) = self.view3d.project(x, y, z);
                let screen_x = cx_center + sx * scale;
                let screen_y = cy_center - sy * scale;

                let color = p.color.unwrap_or(self.default_color);
                let size = p.size.unwrap_or(self.default_size);

                self.draw_point.color = color;
                self.draw_point.draw_point(cx, dvec2(screen_x, screen_y), size);
            }

            // Draw 3D axes
            self.draw_line.color = vec4(0.5, 0.5, 0.5, 0.8);
            let axis_len = 1.2;
            let axes = [
                ((-axis_len, 0.0, 0.0), (axis_len, 0.0, 0.0)),
                ((0.0, -axis_len, 0.0), (0.0, axis_len, 0.0)),
                ((0.0, 0.0, -axis_len * 0.5), (0.0, 0.0, axis_len)),
            ];
            for ((x0, y0, z0), (x1, y1, z1)) in axes {
                let (sx0, sy0) = self.view3d.project(x0, y0, z0);
                let (sx1, sy1) = self.view3d.project(x1, y1, z1);
                self.draw_line.draw_line(cx,
                    dvec2(cx_center + sx0 * scale, cy_center - sy0 * scale),
                    dvec2(cx_center + sx1 * scale, cy_center - sy1 * scale),
                    1.0);
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + 10.0, rect.pos.y + 5.0), &self.title, TextAnchor::TopLeft);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        match event.hits(cx, self.view.area()) {
            Hit::FingerDown(fe) => {
                self.drag_start = Some(fe.abs);
                self.start_azimuth = self.view3d.azimuth;
                self.start_elevation = self.view3d.elevation;
            }
            Hit::FingerMove(fe) => {
                if let Some(start) = self.drag_start {
                    let delta = fe.abs - start;
                    self.view3d.azimuth = self.start_azimuth + delta.x * 0.5;
                    self.view3d.elevation = (self.start_elevation - delta.y * 0.5).clamp(-89.0, 89.0);
                    self.view.redraw(cx);
                }
            }
            Hit::FingerUp(_) => {
                self.drag_start = None;
            }
            Hit::FingerScroll(fe) => {
                if self.zoom == 0.0 { self.zoom = 1.0; }
                let zoom_delta = 1.0 + fe.scroll.y * 0.001;
                self.zoom = (self.zoom * zoom_delta).clamp(0.2, 5.0);
                self.view.redraw(cx);
            }
            _ => {}
        }
    }
}

impl Scatter3DRef {
    pub fn set_title(&self, title: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); } }
    pub fn set_data(&self, x: Vec<f64>, y: Vec<f64>, z: Vec<f64>) { if let Some(mut inner) = self.borrow_mut() { inner.set_data(x, y, z); } }
    pub fn set_color(&self, c: Vec4) { if let Some(mut inner) = self.borrow_mut() { inner.set_color(c); } }
    pub fn set_point_size(&self, s: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_point_size(s); } }
    pub fn set_view(&self, view: View3D) { if let Some(mut inner) = self.borrow_mut() { inner.set_view(view); } }
    pub fn set_azimuth(&self, az: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_azimuth(az); } }
    pub fn set_elevation(&self, el: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_elevation(el); } }
    pub fn clear(&self) { if let Some(mut inner) = self.borrow_mut() { inner.clear(); } }
    pub fn redraw(&self, cx: &mut Cx) { if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); } }
}

// =============================================================================
// Line3D Widget
// =============================================================================

#[derive(Clone, Debug)]
pub struct Line3DSeries {
    pub label: String,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub color: Vec4,
    pub width: f64,
}

impl Line3DSeries {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), x: Vec::new(), y: Vec::new(), z: Vec::new(),
               color: vec4(0.12, 0.47, 0.71, 1.0), width: 1.5 }
    }
    pub fn with_data(mut self, x: Vec<f64>, y: Vec<f64>, z: Vec<f64>) -> Self {
        self.x = x; self.y = y; self.z = z; self
    }
    pub fn with_color(mut self, c: Vec4) -> Self { self.color = c; self }
    pub fn with_width(mut self, w: f64) -> Self { self.width = w; self }
}

#[derive(Live, LiveHook, Widget)]
pub struct Line3D {
    #[deref] #[live] view: View,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] series: Vec<Line3DSeries>,
    #[rust] view3d: View3D,
    #[rust] x_range: (f64, f64),
    #[rust] y_range: (f64, f64),
    #[rust] z_range: (f64, f64),
    #[rust] zoom: f64,
    #[rust] drag_start: Option<DVec2>,
    #[rust] start_azimuth: f64,
    #[rust] start_elevation: f64,
}

impl Line3D {
    pub fn set_title(&mut self, title: impl Into<String>) { self.title = title.into(); }

    pub fn add_series(&mut self, s: Line3DSeries) {
        self.series.push(s);
        self.auto_range();
    }

    fn auto_range(&mut self) {
        let mut x_min = f64::MAX; let mut x_max = f64::MIN;
        let mut y_min = f64::MAX; let mut y_max = f64::MIN;
        let mut z_min = f64::MAX; let mut z_max = f64::MIN;

        for s in &self.series {
            for &v in &s.x { if v < x_min { x_min = v; } if v > x_max { x_max = v; } }
            for &v in &s.y { if v < y_min { y_min = v; } if v > y_max { y_max = v; } }
            for &v in &s.z { if v < z_min { z_min = v; } if v > z_max { z_max = v; } }
        }

        if x_min != f64::MAX {
            let pad_x = (x_max - x_min).max(0.1) * 0.1;
            let pad_y = (y_max - y_min).max(0.1) * 0.1;
            let pad_z = (z_max - z_min).max(0.1) * 0.1;
            self.x_range = (x_min - pad_x, x_max + pad_x);
            self.y_range = (y_min - pad_y, y_max + pad_y);
            self.z_range = (z_min - pad_z, z_max + pad_z);
        }
    }

    pub fn set_view(&mut self, view: View3D) { self.view3d = view; }
    pub fn set_azimuth(&mut self, az: f64) { self.view3d.azimuth = az; }
    pub fn set_elevation(&mut self, el: f64) { self.view3d.elevation = el; }
    pub fn clear(&mut self) { self.series.clear(); }
    pub fn redraw(&mut self, cx: &mut Cx) { self.view.redraw(cx); }
}

impl Widget for Line3D {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.series.is_empty() {
            // Initialize defaults
            if self.view3d.distance == 0.0 { self.view3d = View3D::new(); }
            if self.zoom == 0.0 { self.zoom = 1.0; }

            let cx_center = rect.pos.x + rect.size.x * 0.5;
            let cy_center = rect.pos.y + rect.size.y * 0.5;
            let scale = rect.size.x.min(rect.size.y) * 0.35 * self.zoom;

            // Normalize to [-1, 1]
            let x_scale = if self.x_range.1 != self.x_range.0 { 2.0 / (self.x_range.1 - self.x_range.0) } else { 1.0 };
            let y_scale = if self.y_range.1 != self.y_range.0 { 2.0 / (self.y_range.1 - self.y_range.0) } else { 1.0 };
            let z_scale = if self.z_range.1 != self.z_range.0 { 2.0 / (self.z_range.1 - self.z_range.0) } else { 1.0 };
            let x_off = (self.x_range.0 + self.x_range.1) * 0.5;
            let y_off = (self.y_range.0 + self.y_range.1) * 0.5;
            let z_off = (self.z_range.0 + self.z_range.1) * 0.5;

            // Draw 3D axes first
            self.draw_line.color = vec4(0.5, 0.5, 0.5, 0.8);
            let axis_len = 1.2;
            let axes = [
                ((-axis_len, 0.0, 0.0), (axis_len, 0.0, 0.0)),
                ((0.0, -axis_len, 0.0), (0.0, axis_len, 0.0)),
                ((0.0, 0.0, -axis_len * 0.5), (0.0, 0.0, axis_len)),
            ];
            for ((x0, y0, z0), (x1, y1, z1)) in axes {
                let (sx0, sy0) = self.view3d.project(x0, y0, z0);
                let (sx1, sy1) = self.view3d.project(x1, y1, z1);
                self.draw_line.draw_line(cx,
                    dvec2(cx_center + sx0 * scale, cy_center - sy0 * scale),
                    dvec2(cx_center + sx1 * scale, cy_center - sy1 * scale),
                    1.0);
            }

            // Draw series
            for s in &self.series {
                self.draw_line.color = s.color;
                let n = s.x.len().min(s.y.len()).min(s.z.len());

                for i in 1..n {
                    let x0 = (s.x[i-1] - x_off) * x_scale;
                    let y0 = (s.y[i-1] - y_off) * y_scale;
                    let z0 = (s.z[i-1] - z_off) * z_scale;
                    let (sx0, sy0) = self.view3d.project(x0, y0, z0);

                    let x1 = (s.x[i] - x_off) * x_scale;
                    let y1 = (s.y[i] - y_off) * y_scale;
                    let z1 = (s.z[i] - z_off) * z_scale;
                    let (sx1, sy1) = self.view3d.project(x1, y1, z1);

                    self.draw_line.draw_line(cx,
                        dvec2(cx_center + sx0 * scale, cy_center - sy0 * scale),
                        dvec2(cx_center + sx1 * scale, cy_center - sy1 * scale),
                        s.width);
                }
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + 10.0, rect.pos.y + 5.0), &self.title, TextAnchor::TopLeft);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        match event.hits(cx, self.view.area()) {
            Hit::FingerDown(fe) => {
                self.drag_start = Some(fe.abs);
                self.start_azimuth = self.view3d.azimuth;
                self.start_elevation = self.view3d.elevation;
            }
            Hit::FingerMove(fe) => {
                if let Some(start) = self.drag_start {
                    let delta = fe.abs - start;
                    self.view3d.azimuth = self.start_azimuth + delta.x * 0.5;
                    self.view3d.elevation = (self.start_elevation - delta.y * 0.5).clamp(-89.0, 89.0);
                    self.view.redraw(cx);
                }
            }
            Hit::FingerUp(_) => {
                self.drag_start = None;
            }
            Hit::FingerScroll(fe) => {
                if self.zoom == 0.0 { self.zoom = 1.0; }
                let zoom_delta = 1.0 + fe.scroll.y * 0.001;
                self.zoom = (self.zoom * zoom_delta).clamp(0.2, 5.0);
                self.view.redraw(cx);
            }
            _ => {}
        }
    }
}

impl Line3DRef {
    pub fn set_title(&self, title: impl Into<String>) { if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); } }
    pub fn add_series(&self, s: Line3DSeries) { if let Some(mut inner) = self.borrow_mut() { inner.add_series(s); } }
    pub fn set_view(&self, view: View3D) { if let Some(mut inner) = self.borrow_mut() { inner.set_view(view); } }
    pub fn set_azimuth(&self, az: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_azimuth(az); } }
    pub fn set_elevation(&self, el: f64) { if let Some(mut inner) = self.borrow_mut() { inner.set_elevation(el); } }
    pub fn clear(&self) { if let Some(mut inner) = self.borrow_mut() { inner.clear(); } }
    pub fn redraw(&self, cx: &mut Cx) { if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); } }
}

// =============================================================================
// SubplotGrid - Grid layout for multiple plots
// =============================================================================

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

// =============================================================================
// CandlestickChart Widget - Financial OHLC visualization
// =============================================================================

/// A single candlestick data point (OHLC)
#[derive(Clone, Debug)]
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
// RadarChart Widget - Multi-axis comparison (Spider chart)
// =============================================================================

/// A radar series with values for each axis
#[derive(Clone, Debug)]
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

// =============================================================================
// WaterfallChart Widget - Cumulative effect visualization
// =============================================================================

/// A single waterfall bar entry
#[derive(Clone, Debug)]
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

// =============================================================================
// GaugeChart Widget - Single value dial display
// =============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct GaugeChart {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] value: f64,
    #[rust] min_value: f64,
    #[rust] max_value: f64,
    #[rust] thresholds: Vec<(f64, Vec4)>,  // (value, color) pairs
    #[rust] show_value: bool,
    #[rust] unit: String,
    #[rust] arc_width: f64,
}

impl GaugeChart {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn set_range(&mut self, min: f64, max: f64) {
        self.min_value = min;
        self.max_value = max;
    }

    pub fn set_thresholds(&mut self, thresholds: Vec<(f64, Vec4)>) {
        self.thresholds = thresholds;
    }

    pub fn set_unit(&mut self, unit: impl Into<String>) {
        self.unit = unit.into();
    }

    pub fn set_show_value(&mut self, show: bool) {
        self.show_value = show;
    }

    pub fn set_arc_width(&mut self, width: f64) {
        self.arc_width = width;
    }

    fn get_color_for_value(&self, value: f64) -> Vec4 {
        if self.thresholds.is_empty() {
            return vec4(0.12, 0.47, 0.71, 1.0);
        }
        for &(threshold, color) in self.thresholds.iter().rev() {
            if value >= threshold {
                return color;
            }
        }
        self.thresholds.first().map(|&(_, c)| c).unwrap_or(vec4(0.12, 0.47, 0.71, 1.0))
    }
}

impl Widget for GaugeChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            // Initialize defaults
            if self.max_value == 0.0 && self.min_value == 0.0 {
                self.max_value = 100.0;
            }
            if self.arc_width == 0.0 {
                self.arc_width = 20.0;
            }
            if self.thresholds.is_empty() {
                self.thresholds = vec![
                    (0.0, vec4(0.17, 0.63, 0.17, 1.0)),    // Green
                    (60.0, vec4(1.0, 0.65, 0.0, 1.0)),     // Orange
                    (80.0, vec4(0.84, 0.15, 0.16, 1.0)),   // Red
                ];
            }

            let center = dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + rect.size.y * 0.6);
            let radius = (rect.size.x.min(rect.size.y) / 2.0 - 40.0).max(30.0);

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 5.0),
                    &self.title, TextAnchor::TopCenter);
            }

            // Draw arc segments (from -135° to 135°, i.e., 270° total)
            let start_angle = -std::f64::consts::PI * 0.75;  // -135°
            let end_angle = std::f64::consts::PI * 0.75;     // 135°
            let total_angle = end_angle - start_angle;

            // Draw background arc
            let num_segments = 60;
            self.draw_line.color = vec4(0.85, 0.85, 0.85, 1.0);
            for i in 0..num_segments {
                let t1 = i as f64 / num_segments as f64;
                let t2 = (i + 1) as f64 / num_segments as f64;
                let a1 = start_angle + t1 * total_angle;
                let a2 = start_angle + t2 * total_angle;
                let p1 = dvec2(center.x + radius * a1.cos(), center.y + radius * a1.sin());
                let p2 = dvec2(center.x + radius * a2.cos(), center.y + radius * a2.sin());
                self.draw_line.draw_line(cx, p1, p2, self.arc_width);
            }

            // Draw colored arc based on value
            let value_ratio = ((self.value - self.min_value) / (self.max_value - self.min_value)).clamp(0.0, 1.0);
            let value_angle = start_angle + value_ratio * total_angle;

            let color = self.get_color_for_value(self.value);
            self.draw_line.color = color;

            let value_segments = (value_ratio * num_segments as f64) as usize;
            for i in 0..value_segments {
                let t1 = i as f64 / num_segments as f64;
                let t2 = (i + 1) as f64 / num_segments as f64;
                let a1 = start_angle + t1 * total_angle;
                let a2 = start_angle + t2 * total_angle;
                let p1 = dvec2(center.x + radius * a1.cos(), center.y + radius * a1.sin());
                let p2 = dvec2(center.x + radius * a2.cos(), center.y + radius * a2.sin());
                self.draw_line.draw_line(cx, p1, p2, self.arc_width);
            }

            // Draw needle
            let needle_length = radius - self.arc_width / 2.0 - 5.0;
            let needle_end = dvec2(
                center.x + needle_length * value_angle.cos(),
                center.y + needle_length * value_angle.sin()
            );
            self.draw_line.color = vec4(0.2, 0.2, 0.2, 1.0);
            self.draw_line.draw_line(cx, center, needle_end, 3.0);

            // Draw center circle
            self.draw_fill.color = vec4(0.3, 0.3, 0.3, 1.0);
            self.draw_fill.draw_abs(cx, Rect {
                pos: dvec2(center.x - 8.0, center.y - 8.0),
                size: dvec2(16.0, 16.0),
            });

            // Draw value text
            let value_text = if self.unit.is_empty() {
                format!("{:.1}", self.value)
            } else {
                format!("{:.1}{}", self.value, self.unit)
            };
            self.label.draw_at(cx, dvec2(center.x, center.y + 30.0), &value_text, TextAnchor::TopCenter);

            // Draw min/max labels
            let min_pos = dvec2(center.x + radius * start_angle.cos(), center.y + radius * start_angle.sin() + 15.0);
            let max_pos = dvec2(center.x + radius * end_angle.cos(), center.y + radius * end_angle.sin() + 15.0);
            self.label.draw_at(cx, min_pos, &format!("{:.0}", self.min_value), TextAnchor::TopCenter);
            self.label.draw_at(cx, max_pos, &format!("{:.0}", self.max_value), TextAnchor::TopCenter);
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl GaugeChartRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn set_value(&self, value: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_value(value); }
    }
    pub fn set_range(&self, min: f64, max: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_range(min, max); }
    }
    pub fn set_thresholds(&self, thresholds: Vec<(f64, Vec4)>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_thresholds(thresholds); }
    }
    pub fn set_unit(&self, unit: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_unit(unit); }
    }
    pub fn set_show_value(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_value(show); }
    }
    pub fn set_arc_width(&self, width: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_arc_width(width); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}

// =============================================================================
// FunnelChart Widget - Progressive reduction stages
// =============================================================================

/// A funnel stage entry
#[derive(Clone, Debug)]
pub struct FunnelStage {
    pub label: String,
    pub value: f64,
    pub color: Option<Vec4>,
}

impl FunnelStage {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self { label: label.into(), value, color: None }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct FunnelChart {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] stages: Vec<FunnelStage>,
    #[rust] show_percentages: bool,
    #[rust] show_values: bool,
    #[rust(30.0)] left_margin: f64,
    #[rust(20.0)] bottom_margin: f64,
    #[rust(30.0)] right_margin: f64,
    #[rust(30.0)] top_margin: f64,
}

impl FunnelChart {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_data(&mut self, stages: Vec<FunnelStage>) {
        self.stages = stages;
    }

    pub fn add_stage(&mut self, stage: FunnelStage) {
        self.stages.push(stage);
    }

    pub fn set_show_percentages(&mut self, show: bool) {
        self.show_percentages = show;
    }

    pub fn set_show_values(&mut self, show: bool) {
        self.show_values = show;
    }

    pub fn clear(&mut self) {
        self.stages.clear();
    }
}

impl Widget for FunnelChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.stages.is_empty() {
            let plot_rect = Rect {
                pos: dvec2(rect.pos.x + self.left_margin, rect.pos.y + self.top_margin),
                size: dvec2(
                    rect.size.x - self.left_margin - self.right_margin,
                    rect.size.y - self.top_margin - self.bottom_margin
                ),
            };

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 5.0),
                    &self.title, TextAnchor::TopCenter);
            }

            let max_value = self.stages.iter().map(|s| s.value).fold(0.0f64, f64::max);
            if max_value == 0.0 { return DrawStep::done(); }

            let num_stages = self.stages.len();
            let stage_height = plot_rect.size.y / num_stages as f64;
            let center_x = plot_rect.pos.x + plot_rect.size.x / 2.0;
            let max_width = plot_rect.size.x * 0.9;

            for (i, stage) in self.stages.iter().enumerate() {
                let ratio = stage.value / max_value;
                let width = max_width * ratio;
                let y = plot_rect.pos.y + i as f64 * stage_height;

                // Get color
                let color = stage.color.unwrap_or_else(|| get_color(i));

                // Draw trapezoid (approximated as rectangle for simplicity)
                // For a proper funnel, we'd need next stage's width
                let next_ratio = if i + 1 < num_stages {
                    self.stages[i + 1].value / max_value
                } else {
                    ratio * 0.3  // Taper at bottom
                };
                let next_width = max_width * next_ratio;

                // Draw as a series of lines to create trapezoid effect
                let top_left = dvec2(center_x - width / 2.0, y);
                let top_right = dvec2(center_x + width / 2.0, y);
                let bottom_left = dvec2(center_x - next_width / 2.0, y + stage_height);
                let bottom_right = dvec2(center_x + next_width / 2.0, y + stage_height);

                // Fill trapezoid using horizontal lines
                self.draw_fill.color = color;
                let num_lines = (stage_height as usize).max(1);
                for j in 0..num_lines {
                    let t = j as f64 / num_lines as f64;
                    let line_y = y + t * stage_height;
                    let line_width = width + (next_width - width) * t;
                    self.draw_fill.draw_abs(cx, Rect {
                        pos: dvec2(center_x - line_width / 2.0, line_y),
                        size: dvec2(line_width, 2.0),
                    });
                }

                // Draw outline
                self.draw_line.color = vec4(1.0, 1.0, 1.0, 0.8);
                self.draw_line.draw_line(cx, top_left, top_right, 1.0);
                self.draw_line.draw_line(cx, top_left, bottom_left, 1.0);
                self.draw_line.draw_line(cx, top_right, bottom_right, 1.0);

                // Draw label on left
                self.label.draw_at(cx, dvec2(plot_rect.pos.x - 5.0, y + stage_height / 2.0),
                    &stage.label, TextAnchor::MiddleRight);

                // Draw value/percentage on right
                let value_text = if self.show_percentages {
                    format!("{:.1}%", ratio * 100.0)
                } else {
                    format!("{:.0}", stage.value)
                };
                self.label.draw_at(cx, dvec2(plot_rect.pos.x + plot_rect.size.x + 5.0, y + stage_height / 2.0),
                    &value_text, TextAnchor::MiddleLeft);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl FunnelChartRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn set_data(&self, stages: Vec<FunnelStage>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(stages); }
    }
    pub fn add_stage(&self, stage: FunnelStage) {
        if let Some(mut inner) = self.borrow_mut() { inner.add_stage(stage); }
    }
    pub fn set_show_percentages(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_percentages(show); }
    }
    pub fn set_show_values(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_values(show); }
    }
    pub fn clear(&self) {
        if let Some(mut inner) = self.borrow_mut() { inner.clear(); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}

// ============================================================================
// Heatmap Widget - Grid-based data visualization with color mapping
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct Heatmap {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] data: Vec<Vec<f64>>,
    #[rust] x_labels: Vec<String>,
    #[rust] y_labels: Vec<String>,
    #[rust] colormap: Colormap,
    #[rust] show_values: bool,
    #[rust] min_value: Option<f64>,
    #[rust] max_value: Option<f64>,
}

impl Heatmap {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_data(&mut self, data: Vec<Vec<f64>>) {
        self.data = data;
    }

    pub fn set_labels(&mut self, x_labels: Vec<String>, y_labels: Vec<String>) {
        self.x_labels = x_labels;
        self.y_labels = y_labels;
    }

    pub fn set_colormap(&mut self, colormap: Colormap) {
        self.colormap = colormap;
    }

    pub fn set_show_values(&mut self, show: bool) {
        self.show_values = show;
    }

    pub fn set_range(&mut self, min: f64, max: f64) {
        self.min_value = Some(min);
        self.max_value = Some(max);
    }

    fn get_data_range(&self) -> (f64, f64) {
        if let (Some(min), Some(max)) = (self.min_value, self.max_value) {
            return (min, max);
        }
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        for row in &self.data {
            for &val in row {
                if val < min { min = val; }
                if val > max { max = val; }
            }
        }
        if min == f64::MAX { min = 0.0; }
        if max == f64::MIN { max = 1.0; }
        (min, max)
    }
}

impl Widget for Heatmap {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.data.is_empty() {
            let padding_left = 60.0;
            let padding_right = 20.0;
            let padding_top = 40.0;
            let padding_bottom = 40.0;

            let plot_left = rect.pos.x + padding_left;
            let plot_top = rect.pos.y + padding_top;
            let plot_right = rect.pos.x + rect.size.x - padding_right;
            let plot_bottom = rect.pos.y + rect.size.y - padding_bottom;
            let plot_width = plot_right - plot_left;
            let plot_height = plot_bottom - plot_top;

            let rows = self.data.len();
            let cols = if rows > 0 { self.data[0].len() } else { 0 };

            if cols > 0 {
                let cell_width = plot_width / cols as f64;
                let cell_height = plot_height / rows as f64;
                let (min_val, max_val) = self.get_data_range();
                let range = if (max_val - min_val).abs() < 1e-10 { 1.0 } else { max_val - min_val };

                // Draw cells
                for (row_idx, row) in self.data.iter().enumerate() {
                    for (col_idx, &val) in row.iter().enumerate() {
                        let x = plot_left + col_idx as f64 * cell_width;
                        let y = plot_top + row_idx as f64 * cell_height;

                        let normalized = (val - min_val) / range;
                        let color = self.colormap.sample(normalized);

                        self.draw_fill.color = color;
                        self.draw_fill.draw_abs(cx, Rect {
                            pos: dvec2(x, y),
                            size: dvec2(cell_width - 1.0, cell_height - 1.0),
                        });

                        // Draw value text
                        if self.show_values && cell_width > 25.0 && cell_height > 15.0 {
                            let text = if val.abs() < 10.0 {
                                format!("{:.1}", val)
                            } else {
                                format!("{:.0}", val)
                            };
                            let brightness = color.x * 0.299 + color.y * 0.587 + color.z * 0.114;
                            self.label.draw_text.color = if brightness > 0.5 { vec4(0.0, 0.0, 0.0, 1.0) } else { vec4(1.0, 1.0, 1.0, 1.0) };
                            self.label.draw_at(cx, dvec2(x + cell_width / 2.0, y + cell_height / 2.0), &text, TextAnchor::Center);
                        }
                    }
                }

                // Draw X labels
                self.label.draw_text.color = vec4(0.3, 0.3, 0.3, 1.0);
                for (i, label) in self.x_labels.iter().enumerate() {
                    if i < cols {
                        let x = plot_left + (i as f64 + 0.5) * cell_width;
                        self.label.draw_at(cx, dvec2(x, plot_bottom + 15.0), label, TextAnchor::TopCenter);
                    }
                }

                // Draw Y labels
                for (i, label) in self.y_labels.iter().enumerate() {
                    if i < rows {
                        let y = plot_top + (i as f64 + 0.5) * cell_height;
                        self.label.draw_at(cx, dvec2(plot_left - 10.0, y), label, TextAnchor::MiddleRight);
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

impl Heatmap {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }
}

impl HeatmapRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn set_data(&self, data: Vec<Vec<f64>>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(data); }
    }
    pub fn set_labels(&self, x_labels: Vec<String>, y_labels: Vec<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_labels(x_labels, y_labels); }
    }
    pub fn set_colormap(&self, colormap: Colormap) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_colormap(colormap); }
    }
    pub fn set_show_values(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_values(show); }
    }
    pub fn set_range(&self, min: f64, max: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_range(min, max); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}

// ============================================================================
// Treemap Widget - Hierarchical data visualization
// ============================================================================

#[derive(Clone)]
pub struct TreemapNode {
    pub label: String,
    pub value: f64,
    pub color: Option<Vec4>,
}

impl TreemapNode {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct Treemap {
    #[deref] #[live] view: View,
    #[live] draw_fill: DrawPlotFill,
    #[live] draw_line: DrawPlotLine,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] nodes: Vec<TreemapNode>,
    #[rust] show_labels: bool,
}

impl Treemap {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_data(&mut self, nodes: Vec<TreemapNode>) {
        self.nodes = nodes;
    }

    pub fn set_show_labels(&mut self, show: bool) {
        self.show_labels = show;
    }
}

impl Widget for Treemap {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.nodes.is_empty() {
            let padding = 20.0;
            let title_space = if self.title.is_empty() { 0.0 } else { 30.0 };

            let plot_left = rect.pos.x + padding;
            let plot_top = rect.pos.y + padding + title_space;
            let plot_width = rect.size.x - padding * 2.0;
            let plot_height = rect.size.y - padding * 2.0 - title_space;

            let total: f64 = self.nodes.iter().map(|n| n.value).sum();
            if total > 0.0 {
                let area = plot_width * plot_height;
                let mut x = plot_left;
                let mut y = plot_top;
                let mut remaining_width = plot_width;
                let mut remaining_height = plot_height;
                let horizontal = plot_width > plot_height;

                for (i, node) in self.nodes.iter().enumerate() {
                    let node_area = (node.value / total) * area;
                    let (node_x, node_y, node_w, node_h) = if horizontal {
                        let w = if remaining_height > 0.0 { node_area / remaining_height } else { 0.0 };
                        let w = w.min(remaining_width);
                        let result = (x, y, w, remaining_height);
                        x += w;
                        remaining_width -= w;
                        result
                    } else {
                        let h = if remaining_width > 0.0 { node_area / remaining_width } else { 0.0 };
                        let h = h.min(remaining_height);
                        let result = (x, y, remaining_width, h);
                        y += h;
                        remaining_height -= h;
                        result
                    };

                    if node_w > 2.0 && node_h > 2.0 {
                        let color = node.color.unwrap_or_else(|| get_color(i));

                        // Draw filled rectangle
                        self.draw_fill.color = color;
                        self.draw_fill.draw_abs(cx, Rect {
                            pos: dvec2(node_x, node_y),
                            size: dvec2(node_w - 2.0, node_h - 2.0),
                        });

                        // Draw border
                        self.draw_line.color = vec4(1.0, 1.0, 1.0, 0.8);
                        let corners = [
                            dvec2(node_x, node_y),
                            dvec2(node_x + node_w - 2.0, node_y),
                            dvec2(node_x + node_w - 2.0, node_y + node_h - 2.0),
                            dvec2(node_x, node_y + node_h - 2.0),
                        ];
                        for j in 0..4 {
                            self.draw_line.draw_line(cx, corners[j], corners[(j + 1) % 4], 1.0);
                        }

                        // Draw label
                        if self.show_labels && node_w > 40.0 && node_h > 25.0 {
                            let center = dvec2(node_x + node_w / 2.0, node_y + node_h / 2.0);
                            let brightness = color.x * 0.299 + color.y * 0.587 + color.z * 0.114;
                            self.label.draw_text.color = if brightness > 0.5 { vec4(0.0, 0.0, 0.0, 1.0) } else { vec4(1.0, 1.0, 1.0, 1.0) };
                            self.label.draw_at(cx, center, &node.label, TextAnchor::Center);
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

impl Treemap {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }
}

impl TreemapRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn set_data(&self, nodes: Vec<TreemapNode>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(nodes); }
    }
    pub fn set_show_labels(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_labels(show); }
    }
    pub fn redraw(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() { inner.redraw(cx); }
    }
}

// ============================================================================
// BubbleChart Widget - Scatter plot with size dimension
// ============================================================================

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

// ============================================================================
// DonutChart Widget - Pie chart with center hole
// ============================================================================

#[derive(Clone)]
pub struct DonutSlice {
    pub label: String,
    pub value: f64,
    pub color: Option<Vec4>,
}

impl DonutSlice {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self { label: label.into(), value, color: None }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct DonutChart {
    #[deref] #[live] view: View,
    #[live] draw_arc: DrawArc,
    #[live] label: PlotLabel,
    #[rust] title: String,
    #[rust] slices: Vec<DonutSlice>,
    #[rust] inner_radius_ratio: f64,
    #[rust] center_label: String,
    #[rust] show_labels: bool,
    #[rust] show_percentages: bool,
    #[rust] use_gradient: bool,
}

impl DonutChart {
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn set_data(&mut self, slices: Vec<DonutSlice>) {
        self.slices = slices;
    }

    pub fn add_slice(&mut self, slice: DonutSlice) {
        self.slices.push(slice);
    }

    pub fn set_inner_radius_ratio(&mut self, ratio: f64) {
        self.inner_radius_ratio = ratio.clamp(0.0, 0.9);
    }

    pub fn set_center_label(&mut self, label: impl Into<String>) {
        self.center_label = label.into();
    }

    pub fn set_show_labels(&mut self, show: bool) {
        self.show_labels = show;
    }

    pub fn set_show_percentages(&mut self, show: bool) {
        self.show_percentages = show;
    }

    pub fn set_use_gradient(&mut self, use_gradient: bool) {
        self.use_gradient = use_gradient;
    }

    pub fn clear(&mut self) {
        self.slices.clear();
    }
}

impl Widget for DonutChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 && !self.slices.is_empty() {
            // Set defaults
            if self.inner_radius_ratio == 0.0 { self.inner_radius_ratio = 0.5; }
            // Enable gradient by default for better visuals
            self.use_gradient = true;

            let total: f64 = self.slices.iter().map(|s| s.value).sum();
            if total <= 0.0 { return DrawStep::done(); }

            let center = dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + rect.size.y / 2.0);
            let outer_radius = (rect.size.x.min(rect.size.y) / 2.0 - 40.0).max(20.0);

            let mut start_angle = -std::f64::consts::PI / 2.0;

            for (i, slice) in self.slices.iter().enumerate() {
                let sweep_angle = (slice.value / total) * 2.0 * std::f64::consts::PI;
                let end_angle = start_angle + sweep_angle;
                let color = slice.color.unwrap_or_else(|| get_color(i));

                // Use proper arc shader for clean rendering
                self.draw_arc.color = color;
                if self.use_gradient {
                    // Create a subtle radial gradient
                    let lighter = vec4(
                        (color.x * 1.3).min(1.0),
                        (color.y * 1.3).min(1.0),
                        (color.z * 1.3).min(1.0),
                        color.w
                    );
                    self.draw_arc.draw_arc_gradient(
                        cx, center, outer_radius, self.inner_radius_ratio,
                        start_angle, end_angle, lighter, color, 0  // radial gradient
                    );
                } else {
                    self.draw_arc.draw_arc(cx, center, outer_radius, self.inner_radius_ratio, start_angle, end_angle);
                }

                // Draw label
                if self.show_labels || self.show_percentages {
                    let mid_angle = start_angle + sweep_angle / 2.0;
                    let label_radius = outer_radius + 15.0;
                    let label_pos = dvec2(center.x + label_radius * mid_angle.cos(), center.y + label_radius * mid_angle.sin());

                    let label_text = if self.show_percentages {
                        let pct = (slice.value / total) * 100.0;
                        if self.show_labels {
                            format!("{} ({:.1}%)", slice.label, pct)
                        } else {
                            format!("{:.1}%", pct)
                        }
                    } else {
                        slice.label.clone()
                    };

                    self.label.draw_text.color = vec4(0.3, 0.3, 0.3, 1.0);
                    let anchor = if mid_angle.cos() > 0.0 { TextAnchor::MiddleLeft } else { TextAnchor::MiddleRight };
                    self.label.draw_at(cx, label_pos, &label_text, anchor);
                }

                start_angle = end_angle;
            }

            // Draw center label
            if !self.center_label.is_empty() {
                self.label.draw_text.color = vec4(0.2, 0.2, 0.2, 1.0);
                self.label.draw_at(cx, center, &self.center_label, TextAnchor::Center);
            }

            // Draw title
            if !self.title.is_empty() {
                self.label.draw_at(cx, dvec2(rect.pos.x + rect.size.x / 2.0, rect.pos.y + 15.0), &self.title, TextAnchor::TopCenter);
            }
        }

        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl DonutChart {
    pub fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }
}

impl DonutChartRef {
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
    pub fn set_data(&self, slices: Vec<DonutSlice>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(slices); }
    }
    pub fn add_slice(&self, slice: DonutSlice) {
        if let Some(mut inner) = self.borrow_mut() { inner.add_slice(slice); }
    }
    pub fn set_inner_radius_ratio(&self, ratio: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_inner_radius_ratio(ratio); }
    }
    pub fn set_center_label(&self, label: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_center_label(label); }
    }
    pub fn set_show_labels(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_labels(show); }
    }
    pub fn set_show_percentages(&self, show: bool) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_show_percentages(show); }
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

// ============================================================================
// AreaChart Widget - Stacked area visualization
// ============================================================================

#[derive(Clone)]
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

// =============================================================================
// Stackplot (Stacked Area Chart) - Adapted from makepad-d3
// =============================================================================

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
// HexbinChart - Adapted from makepad-d3
// =============================================================================

#[derive(Clone, Debug)]
pub struct HexbinPoint {
    pub x: f64,
    pub y: f64,
}

impl HexbinPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
struct HexBin {
    center: DVec2,
    count: usize,
    ring: i32,
}

#[derive(Live, LiveHook, Widget)]
pub struct HexbinChart {
    #[redraw] #[live] draw_bg: DrawQuad,
    #[redraw] #[live] draw_triangle: DrawTriangle,
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    #[live] label: PlotLabel,

    #[rust] points: Vec<HexbinPoint>,
    #[rust] hex_radius: f64,
    #[rust] color_low: Vec4,
    #[rust] color_high: Vec4,
    #[rust] title: String,
    #[rust] area: Area,
}

impl HexbinChart {
    pub fn set_data(&mut self, points: Vec<HexbinPoint>) {
        self.points = points;
    }

    pub fn set_hex_radius(&mut self, radius: f64) {
        self.hex_radius = radius.max(5.0);
    }

    pub fn set_colors(&mut self, low: Vec4, high: Vec4) {
        self.color_low = low;
        self.color_high = high;
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    fn cube_round(q: f64, r: f64) -> (i32, i32, i32) {
        let s = -q - r;
        let mut rq = q.round();
        let mut rr = r.round();
        let mut rs = s.round();

        let q_diff = (rq - q).abs();
        let r_diff = (rr - r).abs();
        let s_diff = (rs - s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            rq = -rr - rs;
        } else if r_diff > s_diff {
            rr = -rq - rs;
        } else {
            rs = -rq - rr;
        }

        (rq as i32, rr as i32, rs as i32)
    }

    fn calculate_bins(&self, chart_x: f64, chart_y: f64, chart_w: f64, chart_h: f64) -> (Vec<HexBin>, i32) {
        let center_x = chart_x + chart_w / 2.0;
        let center_y = chart_y + chart_h / 2.0;
        let chart_size = chart_w.min(chart_h);
        let hex_radius = if self.hex_radius > 0.0 { self.hex_radius } else { 14.0 };
        let rings = ((chart_size / 2.0) / (hex_radius * 1.5)).floor() as i32;

        let mut bin_data: std::collections::HashMap<(i32, i32, i32), (usize, i32)> = std::collections::HashMap::new();

        // Generate hexagonal grid using cube coordinates
        for q in -rings..=rings {
            for r in (-rings).max(-q - rings)..=rings.min(-q + rings) {
                let s = -q - r;
                let ring = q.abs().max(r.abs()).max(s.abs());
                bin_data.insert((q, r, s), (0, ring));
            }
        }

        // Assign points to bins
        if !self.points.is_empty() {
            let x_min = self.points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
            let x_max = self.points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
            let y_min = self.points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
            let y_max = self.points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);

            let x_range = (x_max - x_min).max(1.0);
            let y_range = (y_max - y_min).max(1.0);

            for point in &self.points {
                let px = ((point.x - x_min) / x_range) * chart_size - chart_size / 2.0;
                let py = ((point.y - y_min) / y_range) * chart_size - chart_size / 2.0;

                // Convert pixel to cube coordinates (pointy-topped)
                let q = (px * 3.0_f64.sqrt() / 3.0 - py / 3.0) / hex_radius;
                let r = (py * 2.0 / 3.0) / hex_radius;

                let (q, r, s) = Self::cube_round(q, r);

                if let Some((count, _)) = bin_data.get_mut(&(q, r, s)) {
                    *count += 1;
                }
            }
        }

        // Convert to pixel positions
        let mut bins = Vec::new();
        for ((q, r, _s), (count, ring)) in bin_data {
            let px = hex_radius * (3.0_f64.sqrt() * q as f64 + 3.0_f64.sqrt() / 2.0 * r as f64);
            let py = hex_radius * (3.0 / 2.0 * r as f64);

            bins.push(HexBin {
                center: DVec2 { x: center_x + px, y: center_y + py },
                count,
                ring,
            });
        }

        (bins, rings)
    }

    fn interpolate_color(&self, t: f64) -> Vec4 {
        // Radial gradient: dark at center (t=0), light at edge (t=1)
        vec4(
            self.color_high.x + t as f32 * (self.color_low.x - self.color_high.x),
            self.color_high.y + t as f32 * (self.color_low.y - self.color_high.y),
            self.color_high.z + t as f32 * (self.color_low.z - self.color_high.z),
            self.color_high.w + t as f32 * (self.color_low.w - self.color_high.w),
        )
    }

    fn draw_hexagon(&mut self, cx: &mut Cx2d, center: DVec2, radius: f64, color: Vec4) {
        let corners: Vec<DVec2> = (0..6)
            .map(|i| {
                let angle = std::f64::consts::PI / 3.0 * i as f64 + std::f64::consts::PI / 2.0;
                DVec2 {
                    x: center.x + radius * angle.cos(),
                    y: center.y + radius * angle.sin(),
                }
            })
            .collect();

        self.draw_triangle.color = color;

        for i in 0..6 {
            self.draw_triangle.draw_triangle(cx, center, corners[i], corners[(i + 1) % 6]);
        }
    }
}

impl Widget for HexbinChart {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 10.0 && rect.size.y > 10.0 {
            let padding = 30.0;
            let chart_x = rect.pos.x + padding;
            let chart_y = rect.pos.y + padding;
            let chart_w = rect.size.x - padding * 2.0;
            let chart_h = rect.size.y - padding * 2.0;

            // Initialize defaults if not set
            if self.hex_radius <= 0.0 { self.hex_radius = 14.0; }
            if self.color_high == Vec4::default() {
                self.color_high = vec4(0.05, 0.15, 0.45, 1.0);
                self.color_low = vec4(0.92, 0.95, 0.98, 1.0);
            }

            let (bins, max_ring) = self.calculate_bins(chart_x, chart_y, chart_w, chart_h);

            // Sort by ring for proper layering
            let mut sorted_bins: Vec<_> = bins.iter().collect();
            sorted_bins.sort_by_key(|b| b.ring);

            for bin in sorted_bins {
                let t = if max_ring > 0 { bin.ring as f64 / max_ring as f64 } else { 0.0 };
                let t_eased = t * t * (3.0 - 2.0 * t); // smoothstep
                let color = self.interpolate_color(t_eased);
                self.draw_hexagon(cx, bin.center, self.hex_radius * 0.94, color);
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

impl HexbinChartRef {
    pub fn set_data(&self, points: Vec<HexbinPoint>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(points); }
    }
    pub fn set_hex_radius(&self, radius: f64) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_hex_radius(radius); }
    }
    pub fn set_colors(&self, low: Vec4, high: Vec4) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_colors(low, high); }
    }
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
}

// =============================================================================
// Streamgraph - Adapted from makepad-d3
// =============================================================================

#[derive(Clone, Debug)]
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

// =============================================================================
// SankeyDiagram - Adapted from makepad-d3
// =============================================================================

#[derive(Clone, Debug)]
pub struct SankeyNode {
    pub name: String,
    pub layer: usize,
    pub value: f64,
    pub color: Vec4,
    // Layout computed values
    y: f64,
    height: f64,
}

impl SankeyNode {
    pub fn new(name: impl Into<String>, layer: usize, value: f64, color: Vec4) -> Self {
        Self {
            name: name.into(),
            layer,
            value,
            color,
            y: 0.0,
            height: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SankeyLink {
    pub source: usize,
    pub target: usize,
    pub value: f64,
    // Layout computed values
    source_y: f64,
    target_y: f64,
}

impl SankeyLink {
    pub fn new(source: usize, target: usize, value: f64) -> Self {
        Self {
            source,
            target,
            value,
            source_y: 0.0,
            target_y: 0.0,
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SankeyDiagram {
    #[redraw] #[live] draw_bg: DrawQuad,
    #[redraw] #[live] draw_triangle: DrawTriangle,
    #[redraw] #[live] draw_line: DrawPlotLine,
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    #[live] label: PlotLabel,

    #[rust] nodes: Vec<SankeyNode>,
    #[rust] links: Vec<SankeyLink>,
    #[rust] title: String,
    #[rust] area: Area,
}

impl SankeyDiagram {
    pub fn set_data(&mut self, nodes: Vec<SankeyNode>, links: Vec<SankeyLink>) {
        self.nodes = nodes;
        self.links = links;
        self.compute_layout();
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    fn compute_layout(&mut self) {
        if self.nodes.is_empty() { return; }

        // Calculate incoming totals
        let mut incoming_totals: Vec<f64> = vec![0.0; self.nodes.len()];
        for link in &self.links {
            incoming_totals[link.target] += link.value;
        }

        // Update node values from incoming
        for i in 0..self.nodes.len() {
            if self.nodes[i].layer > 0 {
                self.nodes[i].value = incoming_totals[i];
            }
        }

        let max_layer = self.nodes.iter().map(|n| n.layer).max().unwrap_or(0);

        // Layout each layer
        for layer in 0..=max_layer {
            let layer_nodes: Vec<usize> = self.nodes.iter()
                .enumerate()
                .filter(|(_, n)| n.layer == layer)
                .map(|(i, _)| i)
                .collect();

            let total_value: f64 = layer_nodes.iter()
                .map(|&i| if layer == 0 { self.nodes[i].value } else { incoming_totals[i] })
                .sum();

            let mut y = 0.0;
            let gap_fraction = 0.08;

            for &idx in &layer_nodes {
                let node_value = if layer == 0 { self.nodes[idx].value } else { incoming_totals[idx] };
                let height = if total_value > 0.0 {
                    node_value / total_value * (1.0 - gap_fraction * (layer_nodes.len() - 1) as f64)
                } else { 0.0 };
                self.nodes[idx].y = y;
                self.nodes[idx].height = height;
                y += height + gap_fraction;
            }
        }

        // Compute source totals
        let source_totals: Vec<f64> = self.nodes.iter().enumerate().map(|(i, node)| {
            if node.layer == 0 { node.value } else { incoming_totals[i] }
        }).collect();

        // Compute link positions
        let mut source_offsets: Vec<f64> = vec![0.0; self.nodes.len()];
        let mut target_offsets: Vec<f64> = vec![0.0; self.nodes.len()];

        for link in &mut self.links {
            let source_idx = link.source;
            let target_idx = link.target;

            link.source_y = self.nodes[source_idx].y + source_offsets[source_idx];
            link.target_y = self.nodes[target_idx].y + target_offsets[target_idx];

            let source_total = source_totals[source_idx];
            let target_total = incoming_totals[target_idx];

            if source_total > 0.0 {
                source_offsets[source_idx] += link.value / source_total * self.nodes[source_idx].height;
            }
            if target_total > 0.0 {
                target_offsets[target_idx] += link.value / target_total * self.nodes[target_idx].height;
            }
        }
    }
}

impl Widget for SankeyDiagram {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);

        if rect.size.x > 10.0 && rect.size.y > 10.0 && !self.nodes.is_empty() {
            let padding = 30.0;
            let chart_x = rect.pos.x + padding;
            let chart_y = rect.pos.y + 30.0;
            let chart_width = rect.size.x - padding * 2.0;
            let chart_height = rect.size.y - padding - 40.0;

            if chart_width <= 0.0 || chart_height <= 0.0 { return DrawStep::done(); }

            let max_layer = self.nodes.iter().map(|n| n.layer).max().unwrap_or(0);
            let node_width = 24.0;
            let layer_spacing = if max_layer > 0 {
                (chart_width - node_width) / max_layer as f64
            } else { chart_width };

            // Precompute totals
            let mut incoming_totals: Vec<f64> = vec![0.0; self.nodes.len()];
            for link in &self.links {
                incoming_totals[link.target] += link.value;
            }

            let source_totals: Vec<f64> = self.nodes.iter().enumerate().map(|(i, node)| {
                if node.layer == 0 { node.value } else { incoming_totals[i] }
            }).collect();

            // Draw links
            for link in &self.links {
                let source = &self.nodes[link.source];
                let target = &self.nodes[link.target];

                let sx = chart_x + source.layer as f64 * layer_spacing + node_width;
                let sy = chart_y + link.source_y * chart_height;
                let tx = chart_x + target.layer as f64 * layer_spacing;
                let ty = chart_y + link.target_y * chart_height;

                let source_total = source_totals[link.source];
                let target_total = incoming_totals[link.target];

                let link_height_source = if source_total > 0.0 {
                    (link.value / source_total) * source.height * chart_height
                } else { 0.0 };
                let link_height_target = if target_total > 0.0 {
                    (link.value / target_total) * target.height * chart_height
                } else { 0.0 };

                // Draw curved flow
                let segments = 24;
                for i in 0..segments {
                    let t1 = i as f64 / segments as f64;
                    let t2 = (i + 1) as f64 / segments as f64;

                    let ease1 = t1 * t1 * (3.0 - 2.0 * t1);
                    let ease2 = t2 * t2 * (3.0 - 2.0 * t2);

                    let x1 = sx + (tx - sx) * t1;
                    let x2 = sx + (tx - sx) * t2;
                    let y1_top = sy + (ty - sy) * ease1;
                    let y2_top = sy + (ty - sy) * ease2;

                    let h1 = link_height_source + (link_height_target - link_height_source) * ease1;
                    let h2 = link_height_source + (link_height_target - link_height_source) * ease2;

                    let t_color = t1 as f32;
                    let color = vec4(
                        source.color.x + (target.color.x - source.color.x) * t_color,
                        source.color.y + (target.color.y - source.color.y) * t_color,
                        source.color.z + (target.color.z - source.color.z) * t_color,
                        0.55,
                    );

                    self.draw_triangle.color = color;
                    self.draw_triangle.draw_triangle(cx, dvec2(x1, y1_top), dvec2(x2, y2_top), dvec2(x2, y2_top + h2));
                    self.draw_triangle.draw_triangle(cx, dvec2(x1, y1_top), dvec2(x2, y2_top + h2), dvec2(x1, y1_top + h1));
                }
            }

            // Draw nodes
            for node in &self.nodes {
                let x = chart_x + node.layer as f64 * layer_spacing;
                let y = chart_y + node.y * chart_height;
                let height = node.height * chart_height;

                self.draw_triangle.color = node.color;
                self.draw_triangle.draw_triangle(cx, dvec2(x, y), dvec2(x + node_width, y), dvec2(x + node_width, y + height));
                self.draw_triangle.draw_triangle(cx, dvec2(x, y), dvec2(x + node_width, y + height), dvec2(x, y + height));
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

impl SankeyDiagramRef {
    pub fn set_data(&self, nodes: Vec<SankeyNode>, links: Vec<SankeyLink>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_data(nodes, links); }
    }
    pub fn set_title(&self, title: impl Into<String>) {
        if let Some(mut inner) = self.borrow_mut() { inner.set_title(title); }
    }
}
