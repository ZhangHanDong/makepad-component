use makepad_widgets::*;

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
