// Text rendering for plot labels

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;

    pub PlotLabel = {{PlotLabel}} {
        draw_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: 10.0
            }
            color: #666666
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TextAnchor {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    #[default]
    Center,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(Live, LiveHook, LiveRegister)]
pub struct PlotLabel {
    #[live] pub draw_text: DrawText,
}

impl PlotLabel {
    pub fn draw_at(&mut self, cx: &mut Cx2d, pos: DVec2, text: &str, anchor: TextAnchor) {
        // Layout text to get dimensions
        let layout = self.draw_text.layout(cx, 0.0, 0.0, None, false, Align::default(), text);
        let text_width = layout.size_in_lpxs.width as f64 * self.draw_text.font_scale as f64;
        let text_height = layout.size_in_lpxs.height as f64 * self.draw_text.font_scale as f64;

        // Calculate offset based on anchor
        let offset = match anchor {
            TextAnchor::TopLeft => dvec2(0.0, 0.0),
            TextAnchor::TopCenter => dvec2(-text_width / 2.0, 0.0),
            TextAnchor::TopRight => dvec2(-text_width, 0.0),
            TextAnchor::MiddleLeft => dvec2(0.0, -text_height / 2.0),
            TextAnchor::Center => dvec2(-text_width / 2.0, -text_height / 2.0),
            TextAnchor::MiddleRight => dvec2(-text_width, -text_height / 2.0),
            TextAnchor::BottomLeft => dvec2(0.0, -text_height),
            TextAnchor::BottomCenter => dvec2(-text_width / 2.0, -text_height),
            TextAnchor::BottomRight => dvec2(-text_width, -text_height),
        };

        let draw_pos = pos + offset;
        self.draw_text.draw_abs(cx, draw_pos, text);
    }

    pub fn set_color(&mut self, color: Vec4) {
        self.draw_text.color = color;
    }

    pub fn set_font_size(&mut self, size: f64) {
        self.draw_text.font_scale = (size / 10.0) as f32; // Base font size is 10.0
    }
}
