pub use makepad_widgets;

pub mod a2ui;
pub mod theme;
pub mod widgets;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    crate::theme::live_design(cx);
    crate::widgets::live_design(cx);
    crate::a2ui::live_design(cx);
}
