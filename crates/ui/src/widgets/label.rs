use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================
    // Font Size Constants
    // ============================================
    LABEL_FONT_SIZE_XS = 10.0
    LABEL_FONT_SIZE_SM = 12.0
    LABEL_FONT_SIZE_MD = 14.0
    LABEL_FONT_SIZE_LG = 16.0
    LABEL_FONT_SIZE_XL = 18.0
    LABEL_FONT_SIZE_2XL = 24.0
    LABEL_FONT_SIZE_3XL = 30.0

    // Line height multiplier
    LABEL_LINE_HEIGHT = 1.4

    // ============================================
    // Base Label Component
    // ============================================
    pub MpLabel = {{MpLabel}} {
        width: Fit,
        height: Fit,

        // Main text
        draw_text: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: (LABEL_FONT_SIZE_MD)
                line_spacing: (LABEL_LINE_HEIGHT)
            }
            color: (FOREGROUND)
        }

        // Secondary text (muted color, shown after main text)
        draw_secondary: {
            text_style: <THEME_FONT_REGULAR> {
                font_size: (LABEL_FONT_SIZE_MD)
                line_spacing: (LABEL_LINE_HEIGHT)
            }
            color: (MUTED_FOREGROUND)
        }

        text: ""
    }

    // ============================================
    // Size Variants
    // ============================================
    pub MpLabelXs = <MpLabel> {
        draw_text: { text_style: { font_size: (LABEL_FONT_SIZE_XS) } }
        draw_secondary: { text_style: { font_size: (LABEL_FONT_SIZE_XS) } }
    }

    pub MpLabelSm = <MpLabel> {
        draw_text: { text_style: { font_size: (LABEL_FONT_SIZE_SM) } }
        draw_secondary: { text_style: { font_size: (LABEL_FONT_SIZE_SM) } }
    }

    pub MpLabelMd = <MpLabel> {
        draw_text: { text_style: { font_size: (LABEL_FONT_SIZE_MD) } }
        draw_secondary: { text_style: { font_size: (LABEL_FONT_SIZE_MD) } }
    }

    pub MpLabelLg = <MpLabel> {
        draw_text: { text_style: { font_size: (LABEL_FONT_SIZE_LG) } }
        draw_secondary: { text_style: { font_size: (LABEL_FONT_SIZE_LG) } }
    }

    pub MpLabelXl = <MpLabel> {
        draw_text: { text_style: { font_size: (LABEL_FONT_SIZE_XL) } }
        draw_secondary: { text_style: { font_size: (LABEL_FONT_SIZE_XL) } }
    }

    pub MpLabel2xl = <MpLabel> {
        draw_text: { text_style: { font_size: (LABEL_FONT_SIZE_2XL) } }
        draw_secondary: { text_style: { font_size: (LABEL_FONT_SIZE_2XL) } }
    }

    pub MpLabel3xl = <MpLabel> {
        draw_text: { text_style: { font_size: (LABEL_FONT_SIZE_3XL) } }
        draw_secondary: { text_style: { font_size: (LABEL_FONT_SIZE_3XL) } }
    }

    // ============================================
    // Weight Variants
    // ============================================
    pub MpLabelBold = <MpLabel> {
        draw_text: {
            text_style: <THEME_FONT_BOLD> {
                font_size: (LABEL_FONT_SIZE_MD)
                line_spacing: (LABEL_LINE_HEIGHT)
            }
        }
        draw_secondary: {
            text_style: <THEME_FONT_BOLD> {
                font_size: (LABEL_FONT_SIZE_MD)
                line_spacing: (LABEL_LINE_HEIGHT)
            }
        }
    }

    // ============================================
    // Color Variants
    // ============================================
    pub MpLabelMuted = <MpLabel> {
        draw_text: { color: (MUTED_FOREGROUND) }
    }

    pub MpLabelPrimary = <MpLabel> {
        draw_text: { color: (PRIMARY) }
    }

    pub MpLabelDanger = <MpLabel> {
        draw_text: { color: (DANGER) }
    }

    pub MpLabelSuccess = <MpLabel> {
        draw_text: { color: (SUCCESS) }
    }

    pub MpLabelWarning = <MpLabel> {
        draw_text: { color: (WARNING) }
    }

    pub MpLabelInfo = <MpLabel> {
        draw_text: { color: (INFO) }
    }

    // ============================================
    // Heading Variants (Bold + Larger)
    // ============================================
    pub MpHeading1 = <MpLabel> {
        draw_text: {
            text_style: <THEME_FONT_BOLD> {
                font_size: (LABEL_FONT_SIZE_3XL)
                line_spacing: (LABEL_LINE_HEIGHT)
            }
        }
    }

    pub MpHeading2 = <MpLabel> {
        draw_text: {
            text_style: <THEME_FONT_BOLD> {
                font_size: (LABEL_FONT_SIZE_2XL)
                line_spacing: (LABEL_LINE_HEIGHT)
            }
        }
    }

    pub MpHeading3 = <MpLabel> {
        draw_text: {
            text_style: <THEME_FONT_BOLD> {
                font_size: (LABEL_FONT_SIZE_XL)
                line_spacing: (LABEL_LINE_HEIGHT)
            }
        }
    }

    pub MpHeading4 = <MpLabel> {
        draw_text: {
            text_style: <THEME_FONT_BOLD> {
                font_size: (LABEL_FONT_SIZE_LG)
                line_spacing: (LABEL_LINE_HEIGHT)
            }
        }
    }

    pub MpHeading5 = <MpLabel> {
        draw_text: {
            text_style: <THEME_FONT_BOLD> {
                font_size: (LABEL_FONT_SIZE_MD)
                line_spacing: (LABEL_LINE_HEIGHT)
            }
        }
    }

    pub MpHeading6 = <MpLabel> {
        draw_text: {
            text_style: <THEME_FONT_BOLD> {
                font_size: (LABEL_FONT_SIZE_SM)
                line_spacing: (LABEL_LINE_HEIGHT)
            }
        }
    }
}

const MASK_CHAR: &str = "•";

/// Label widget with support for secondary text, masking, and highlighting
#[derive(Live, LiveHook, Widget)]
pub struct MpLabel {
    #[redraw]
    #[live]
    draw_text: DrawText,
    #[live]
    draw_secondary: DrawText,

    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    /// Main text content
    #[live]
    text: ArcStringMut,

    /// Secondary text (displayed after main text in muted color)
    #[live]
    secondary: ArcStringMut,

    /// Whether to mask the text (for passwords)
    #[live(false)]
    masked: bool,

    /// Text to highlight (case-insensitive search)
    #[live]
    highlight: ArcStringMut,

    /// Highlight color
    #[live]
    highlight_color: Option<Vec4>,

    #[rust]
    area: Area,
}

impl Widget for MpLabel {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {
        // Label is non-interactive by default
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let main_text = self.get_display_text();
        let secondary_text = self.secondary.as_ref().to_string();
        let highlight_str = self.highlight.as_ref().to_string();

        // Calculate positions
        let has_secondary = !secondary_text.is_empty();

        // Start drawing
        cx.begin_turtle(walk, self.layout);

        // Draw main text (with optional highlighting)
        if !highlight_str.is_empty() && !self.masked {
            self.draw_highlighted_text(cx, &main_text, &highlight_str);
        } else {
            self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), &main_text);
        }

        // Draw secondary text if present
        if has_secondary {
            // Add a space separator
            self.draw_secondary.draw_walk(cx, Walk::fit(), Align::default(), &format!(" {}", secondary_text));
        }

        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

impl MpLabel {
    /// Get the display text (masked if needed)
    fn get_display_text(&self) -> String {
        let text = self.text.as_ref();
        if self.masked {
            MASK_CHAR.repeat(text.chars().count())
        } else {
            text.to_string()
        }
    }

    /// Draw text with highlighting
    fn draw_highlighted_text(&mut self, cx: &mut Cx2d, text: &str, highlight: &str) {
        if highlight.is_empty() {
            self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), text);
            return;
        }

        let text_lower = text.to_lowercase();
        let highlight_lower = highlight.to_lowercase();

        let mut last_end = 0;
        let mut search_start = 0;

        while let Some(pos) = text_lower[search_start..].find(&highlight_lower) {
            let match_start = search_start + pos;
            let match_end = match_start + highlight.len();

            // Draw text before match
            if match_start > last_end {
                let before = &text[last_end..match_start];
                self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), before);
            }

            // Draw highlighted text
            // For now, we draw with a different approach - using the primary color
            // In a more advanced implementation, we could use a background highlight
            let matched = &text[match_start..match_end];
            let original_color = self.draw_text.color;

            // Use highlight color or default to primary
            if let Some(color) = self.highlight_color {
                self.draw_text.color = color;
            } else {
                self.draw_text.color = vec4(0.231, 0.51, 0.965, 1.0); // PRIMARY color
            }
            self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), matched);
            self.draw_text.color = original_color;

            last_end = match_end;
            search_start = match_start + 1;

            // Handle char boundary
            while search_start < text.len() && !text.is_char_boundary(search_start) {
                search_start += 1;
            }
        }

        // Draw remaining text
        if last_end < text.len() {
            let remaining = &text[last_end..];
            self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), remaining);
        }
    }

    /// Set the main text
    pub fn set_text(&mut self, text: &str) {
        self.text.as_mut_empty().push_str(text);
    }

    /// Get the main text
    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    /// Set the secondary text
    pub fn set_secondary(&mut self, text: &str) {
        self.secondary.as_mut_empty().push_str(text);
    }

    /// Get the secondary text
    pub fn secondary(&self) -> &str {
        self.secondary.as_ref()
    }

    /// Set whether the text should be masked
    pub fn set_masked(&mut self, masked: bool) {
        self.masked = masked;
    }

    /// Check if the text is masked
    pub fn is_masked(&self) -> bool {
        self.masked
    }

    /// Set the highlight text (for search)
    pub fn set_highlight(&mut self, text: &str) {
        self.highlight.as_mut_empty().push_str(text);
    }

    /// Clear the highlight
    pub fn clear_highlight(&mut self) {
        self.highlight.as_mut_empty();
    }

    /// Set the highlight color
    pub fn set_highlight_color(&mut self, color: Vec4) {
        self.highlight_color = Some(color);
    }
}

impl MpLabelRef {
    /// Set the main text
    pub fn set_text(&self, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_text(text);
        }
    }

    /// Get the main text
    pub fn text(&self) -> String {
        if let Some(inner) = self.borrow() {
            inner.text().to_string()
        } else {
            String::new()
        }
    }

    /// Set the secondary text
    pub fn set_secondary(&self, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_secondary(text);
        }
    }

    /// Set whether the text should be masked
    pub fn set_masked(&self, masked: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_masked(masked);
        }
    }

    /// Set the highlight text
    pub fn set_highlight(&self, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_highlight(text);
        }
    }

    /// Clear the highlight
    pub fn clear_highlight(&self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear_highlight();
        }
    }
}
