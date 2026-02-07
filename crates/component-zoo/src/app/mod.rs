//! Component Zoo - Demo application showcasing all Makepad components

mod design;
mod shaders;
mod splash_demo;
mod json_render;
mod app_logic;

pub use shaders::*;
pub use splash_demo::*;
pub use json_render::*;
pub use app_logic::*;

use makepad_widgets::*;

app_main!(App);
