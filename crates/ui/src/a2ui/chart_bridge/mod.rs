//! Chart Bridge: Maps A2UI ChartComponent → makepad-plot widgets
//!
//! Bridges the gap between the A2UI protocol's ChartComponent data model
//! and the makepad-plot library's widget API. Each A2UI chart type maps
//! to a corresponding makepad-plot widget.

use makepad_widgets::*;
use makepad_plot::*;
use super::message::*;
use super::value::StringValue;
use super::data_model::DataModel;
use super::processor::resolve_string_value_scoped;


/// Get chart color from ChartComponent palette or fallback to makepad-plot default
pub(crate) fn get_bridge_color(chart: &ChartComponent, index: usize) -> Vec4 {
    if index < chart.colors.len() {
        if let Some(color) = parse_hex_color(&chart.colors[index]) {
            return color;
        }
    }
    makepad_plot::get_color(index)
}

/// Parse a hex color string to Vec4
pub(crate) fn parse_hex_color(hex: &str) -> Option<Vec4> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 { return None; }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;
    Some(Vec4 { x: r, y: g, z: b, w: 1.0 })
}

/// Resolve a chart title from StringValue
pub(crate) fn resolve_title(title: &Option<StringValue>, data_model: &DataModel, scope: Option<&str>) -> Option<String> {
    title.as_ref().map(|sv| resolve_string_value_scoped(sv, data_model, scope))
}

/// Parse a colormap name string to Colormap enum
pub(crate) fn parse_colormap(name: &str) -> Colormap {
    match name.to_lowercase().as_str() {
        "viridis" => Colormap::Viridis,
        "plasma" => Colormap::Plasma,
        "inferno" => Colormap::Inferno,
        "magma" => Colormap::Magma,
        "cividis" => Colormap::Cividis,
        "coolwarm" => Colormap::Coolwarm,
        "rdbu" => Colormap::RdBu,
        "spectral" => Colormap::Spectral,
        "blues" => Colormap::Blues,
        "greens" => Colormap::Greens,
        "oranges" => Colormap::Oranges,
        "reds" => Colormap::Reds,
        "greys" => Colormap::Greys,
        "jet" => Colormap::Jet,
        "hot" => Colormap::Hot,
        "turbo" => Colormap::Turbo,
        _ => Colormap::Viridis,
    }
}

mod basic;
mod statistical;
mod advanced;

pub use basic::*;
pub use statistical::*;
pub use advanced::*;
