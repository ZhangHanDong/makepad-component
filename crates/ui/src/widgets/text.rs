use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================
    // Font Size Constants
    // ============================================
    TEXT_FONT_SIZE_XS = 10.0
    TEXT_FONT_SIZE_SM = 12.0
    TEXT_FONT_SIZE_MD = 14.0
    TEXT_FONT_SIZE_LG = 16.0
    TEXT_FONT_SIZE_XL = 18.0

    // Line height
    TEXT_LINE_HEIGHT = 1.6

    // ============================================
    // Base Text Component (Paragraph-like)
    // ============================================
    pub MpText = {{MpText}} {
        width: Fill,
        height: Fit,

        draw_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: (TEXT_FONT_SIZE_MD)
                line_spacing: (TEXT_LINE_HEIGHT)
            }
            color: (FOREGROUND)
            wrap: Word,
        }

        text: ""
    }

    // ============================================
    // Size Variants
    // ============================================
    pub MpTextXs = <MpText> {
        draw_text: { text_style: { font_size: (TEXT_FONT_SIZE_XS) } }
    }

    pub MpTextSm = <MpText> {
        draw_text: { text_style: { font_size: (TEXT_FONT_SIZE_SM) } }
    }

    pub MpTextMd = <MpText> {
        draw_text: { text_style: { font_size: (TEXT_FONT_SIZE_MD) } }
    }

    pub MpTextLg = <MpText> {
        draw_text: { text_style: { font_size: (TEXT_FONT_SIZE_LG) } }
    }

    pub MpTextXl = <MpText> {
        draw_text: { text_style: { font_size: (TEXT_FONT_SIZE_XL) } }
    }

    // ============================================
    // Color Variants
    // ============================================
    pub MpTextMuted = <MpText> {
        draw_text: { color: (MUTED_FOREGROUND) }
    }

    pub MpTextPrimary = <MpText> {
        draw_text: { color: (PRIMARY) }
    }

    pub MpTextDanger = <MpText> {
        draw_text: { color: (DANGER) }
    }

    pub MpTextSuccess = <MpText> {
        draw_text: { color: (SUCCESS) }
    }

    pub MpTextWarning = <MpText> {
        draw_text: { color: (WARNING) }
    }

    // ============================================
    // Weight Variants
    // ============================================
    pub MpTextBold = <MpText> {
        draw_text: {
            text_style: <THEME_FONT_BOLD> {
                font_size: (TEXT_FONT_SIZE_MD)
                line_spacing: (TEXT_LINE_HEIGHT)
            }
        }
    }

    // ============================================
    // Special Variants
    // ============================================

    // Inline text (no word wrap, fits content)
    pub MpTextInline = <MpText> {
        width: Fit,
        draw_text: { wrap: None }
    }

    // Code/Monospace text
    pub MpTextCode = <MpText> {
        width: Fit,
        padding: { left: 4, right: 4, top: 2, bottom: 2 }

        show_bg: true,
        draw_bg: {
            instance color: (MUTED)
            instance radius: 4.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.radius
                );
                sdf.fill(self.color);
                return sdf.result;
            }
        }

        draw_text: {
            text_style: <THEME_FONT_CODE> {
                font_size: 13.0
            }
            wrap: None,
        }
    }

    // Blockquote style
    pub MpTextBlockquote = <View> {
        width: Fill,
        height: Fit,
        padding: { left: 16, top: 8, bottom: 8 }
        margin: { top: 8, bottom: 8 }

        show_bg: true,
        draw_bg: {
            instance border_color: (MUTED_FOREGROUND)
            instance bg_color: #00000008

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                // Left border
                sdf.rect(0.0, 0.0, 3.0, self.rect_size.y);
                sdf.fill(self.border_color);
                // Background
                sdf.rect(0.0, 0.0, self.rect_size.x, self.rect_size.y);
                sdf.fill(self.bg_color);
                return sdf.result;
            }
        }

        <MpText> {
            draw_text: {
                color: (MUTED_FOREGROUND)
                text_style: { font_size: (TEXT_FONT_SIZE_MD) }
            }
        }
    }

    // Lead text (larger, intro paragraph)
    pub MpTextLead = <MpText> {
        draw_text: {
            text_style: {
                font_size: (TEXT_FONT_SIZE_XL)
                line_spacing: 1.7
            }
            color: (MUTED_FOREGROUND)
        }
    }

    // Small/Caption text
    pub MpTextCaption = <MpText> {
        draw_text: {
            text_style: { font_size: (TEXT_FONT_SIZE_XS) }
            color: (MUTED_FOREGROUND)
        }
    }
}

/// Text widget for paragraph-like text display with word wrapping
#[derive(Live, LiveHook, Widget)]
pub struct MpText {
    #[redraw]
    #[live]
    draw_text: DrawText,
    #[live]
    draw_bg: DrawQuad,

    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[live(false)]
    show_bg: bool,

    /// Text content
    #[live]
    text: ArcStringMut,

    /// Maximum number of lines (0 = unlimited)
    #[live(0usize)]
    max_lines: usize,

    /// Text overflow indicator (e.g., "...")
    #[live]
    overflow: ArcStringMut,

    #[rust]
    area: Area,
}

impl Widget for MpText {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {
        // Text is non-interactive by default
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let text = self.text.as_ref();

        if self.show_bg {
            self.draw_bg.begin(cx, walk, self.layout);
            self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), text);
            self.draw_bg.end(cx);
            self.area = self.draw_bg.area();
        } else {
            cx.begin_turtle(walk, self.layout);
            self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), text);
            cx.end_turtle_with_area(&mut self.area);
        }

        DrawStep::done()
    }
}

impl MpText {
    /// Set the text content
    pub fn set_text(&mut self, text: &str) {
        self.text.as_mut_empty().push_str(text);
    }

    /// Get the text content
    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    /// Set maximum number of lines (0 = unlimited)
    pub fn set_max_lines(&mut self, lines: usize) {
        self.max_lines = lines;
    }

    /// Set the overflow indicator
    pub fn set_overflow(&mut self, overflow: &str) {
        self.overflow.as_mut_empty().push_str(overflow);
    }
}

impl MpTextRef {
    /// Set the text content
    pub fn set_text(&self, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_text(text);
        }
    }

    /// Get the text content
    pub fn text(&self) -> String {
        if let Some(inner) = self.borrow() {
            inner.text().to_string()
        } else {
            String::new()
        }
    }

    /// Set maximum number of lines
    pub fn set_max_lines(&self, lines: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_max_lines(lines);
        }
    }
}
