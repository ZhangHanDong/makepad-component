// Makepad Plot - Matplotlib-style plotting library for Makepad

pub mod plot;
pub mod elements;
pub mod text;

pub use plot::*;
pub use elements::*;
pub use text::*;

use makepad_widgets::*;

pub fn live_design(cx: &mut Cx) {
    crate::elements::live_design(cx);
    crate::text::live_design(cx);
    crate::plot::live_design(cx);
}
