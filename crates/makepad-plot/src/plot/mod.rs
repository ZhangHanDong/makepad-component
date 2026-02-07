// Plot widgets - matplotlib-style plotting library for Makepad

pub mod types;
pub mod scale;
pub mod colormap;
pub mod line;
pub mod bar;
pub mod scatter;
pub mod pie;
pub mod histogram;
pub mod stem;
pub mod heatmap;
pub mod polar;
pub mod contour;
pub mod surface3d;
pub mod scatter3d;
pub mod dual;
pub mod financial;
pub mod gauge;
pub mod treemap;
pub mod bubble;
pub mod area;
pub mod stack;
pub mod hexbin;

// Re-export everything for backwards compatibility
pub use types::*;
pub use scale::*;
pub use colormap::*;
pub use line::*;
pub use bar::*;
pub use scatter::*;
pub use pie::*;
pub use histogram::*;
pub use stem::*;
pub use heatmap::*;
pub use polar::*;
pub use contour::*;
pub use surface3d::*;
pub use scatter3d::*;
pub use dual::*;
pub use financial::*;
pub use gauge::*;
pub use treemap::*;
pub use bubble::*;
pub use area::*;
pub use stack::*;
pub use hexbin::*;

use makepad_widgets::*;

pub fn live_design(cx: &mut Cx) {
    line::live_design(cx);
    bar::live_design(cx);
    scatter::live_design(cx);
    pie::live_design(cx);
    histogram::live_design(cx);
    stem::live_design(cx);
    heatmap::live_design(cx);
    polar::live_design(cx);
    contour::live_design(cx);
    surface3d::live_design(cx);
    scatter3d::live_design(cx);
    dual::live_design(cx);
    financial::live_design(cx);
    gauge::live_design(cx);
    treemap::live_design(cx);
    bubble::live_design(cx);
    area::live_design(cx);
    stack::live_design(cx);
    hexbin::live_design(cx);
}
