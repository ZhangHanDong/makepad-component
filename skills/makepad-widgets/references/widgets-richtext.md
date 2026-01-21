# Rich Text Widgets - Markdown, Html, TextFlow

> **Version:** makepad-widgets 0.7 | **Last Updated:** 2026-01-21

This document covers Makepad's rich text rendering widgets: **Markdown**, **Html**, and **TextFlow**.

## Widget Overview

| Widget | Purpose | Parser | Best For |
|--------|---------|--------|----------|
| `Markdown` | Markdown rendering | `pulldown_cmark` | AI chat, documentation, READMEs |
| `Html` | HTML subset rendering | `makepad_html` | Formatted text from APIs |
| `TextFlow` | Base text engine | N/A | Custom rich text widgets |

---

## 1. Markdown Widget

### Basic Usage (DSL)

```rust
live_design! {
    <Markdown> {
        width: Fill
        height: Fit
        body: "# Hello World\n\nThis is **bold** and *italic* text."
    }
}
```

### Widget Structure

```rust
#[derive(Live, LiveHook, Widget)]
pub struct Markdown {
    #[deref] text_flow: TextFlow,
    #[live] body: ArcStringMut,              // Markdown content
    #[live] paragraph_spacing: f64,           // Space between paragraphs
    #[live] pre_code_spacing: f64,            // Space before code blocks
    #[live(false)] use_code_block_widget: bool, // Use custom code widget
    #[live(false)] use_math_widget: bool,     // Enable math rendering
    #[live] heading_base_scale: f64,          // H1 scale (H2 = scale*0.8, etc.)
}
```

### Setting Content Dynamically

```rust
// Via WidgetRef
let markdown_ref = self.view.widget(id!(my_markdown)).as_markdown();
markdown_ref.set_text(cx, "# New Content\n\nWith **formatting**");

// Via borrow_mut
if let Some(mut inner) = widget_ref.borrow_mut() {
    inner.set_text(cx, "# Updated markdown");
}
```

### DSL Properties

```rust
<Markdown> {
    width: Fill, height: Fit,

    // Content
    body: "# Markdown here"

    // Spacing
    paragraph_spacing: 16.0,
    pre_code_spacing: 8.0,
    heading_base_scale: 1.8,    // H1 scale factor

    // Font
    font_size: 14.0,
    font_color: #FFFFFF,

    // Text style variants
    draw_normal: {
        text_style: { font_size: 14.0 }
        color: #FFFFFF
    }
    draw_italic: {
        text_style: { font_size: 14.0, font: { path: dep("crate://makepad-widgets/resources/IBMPlexSans-Italic.ttf") } }
        color: #FFFFFF
    }
    draw_bold: {
        text_style: { font_size: 14.0, font: { path: dep("crate://makepad-widgets/resources/IBMPlexSans-Bold.ttf") } }
        color: #FFFFFF
    }
    draw_fixed: {
        text_style: { font_size: 13.0, font: { path: dep("crate://makepad-widgets/resources/LiberationMono-Regular.ttf") } }
        color: #00FF00
    }

    // Block styling
    code_layout: {
        flow: Right { wrap: true }
        padding: { left: 10, right: 10, top: 8, bottom: 8 }
    }
    quote_layout: {
        flow: Right { wrap: true }
        padding: { left: 16 }
    }
    list_item_layout: {
        flow: Right { wrap: true }
        padding: { left: 20 }
    }

    // Inline code
    inline_code_padding: { left: 4, right: 4, top: 2, bottom: 2 }
    inline_code_margin: { left: 2, right: 2 }

    // Link widget template
    link = <MarkdownLink> {
        draw_text: { color: #0088FF }
    }
}
```

### Supported Markdown Features

| Feature | Syntax | Example |
|---------|--------|---------|
| **Headings** | `# ## ### ####` | `# H1` to `###### H6` |
| **Bold** | `**text**` or `__text__` | `**bold**` |
| **Italic** | `*text*` or `_text_` | `*italic*` |
| **Strikethrough** | `~~text~~` | `~~deleted~~` |
| **Inline code** | `` `code` `` | `` `variable` `` |
| **Code blocks** | ` ``` ` | Fenced code blocks |
| **Quotes** | `> text` | `> Quote text` |
| **Lists (unordered)** | `- item` or `* item` | `- List item` |
| **Lists (ordered)** | `1. item` | `1. First item` |
| **Links** | `[text](url)` | `[Click](https://...)` |
| **Images** | `![alt](url)` | `![logo](image.png)` |
| **Horizontal rule** | `---` or `***` | `---` |
| **Math (inline)** | `$formula$` | `$x^2$` |
| **Math (display)** | `$$formula$$` | `$$\int f(x)dx$$` |

### Link Click Handling

```rust
#[derive(Clone, Debug, DefaultNone)]
pub enum MarkdownAction {
    None,
    LinkNavigated(String),  // Contains the href URL
}

// In your widget's handle_event
fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
    self.view.handle_event(cx, event, scope);

    for action in cx.capture_actions(|cx| self.view.handle_event(cx, event, scope)) {
        if let MarkdownAction::LinkNavigated(url) = action.cast() {
            log!("User clicked link: {}", url);
            // Open URL, navigate, etc.
        }
    }
}
```

### Custom Code Block Widget

Enable custom code block rendering (e.g., with syntax highlighting):

```rust
<Markdown> {
    use_code_block_widget: true

    code_block = <View> {
        width: Fill
        height: Fit
        flow: Down
        padding: 10

        draw_bg: { color: #1E1E1E }

        // Custom code view goes here
        code_view = <CodeView> {
            // makepad-code-editor configuration
        }
    }
}
```

---

## 2. Html Widget

### Basic Usage (DSL)

```rust
live_design! {
    <Html> {
        width: Fill
        height: Fit
        body: "<h1>Hello World</h1><p>This is <b>bold</b> and <i>italic</i>.</p>"
    }
}
```

### Widget Structure

```rust
#[derive(Live, Widget)]
pub struct Html {
    #[deref] pub text_flow: TextFlow,
    #[live] pub body: ArcStringMut,            // HTML content
    #[live] ul_markers: Vec<String>,           // Bullet markers per nesting
    #[live] ol_markers: Vec<OrderedListType>,  // Numbered list types
    #[live] ol_separator: String,              // Separator after number
}
```

### Setting Content Dynamically

```rust
let html_ref = self.view.widget(id!(my_html)).as_html();
html_ref.set_text(cx, "<h1>Updated</h1><p>New <b>content</b></p>");
```

### DSL Properties

```rust
<Html> {
    width: Fill, height: Fit,

    // Content
    body: "<p>HTML content</p>"

    // List markers
    ul_markers: ["•", "-", "◦"],           // Bullets for nesting levels
    ol_markers: [Numbers, LowerAlpha, LowerRoman],
    ol_separator: ".",                      // "1." vs "1)"

    // Font
    font_size: 14.0,
    font_color: #FFFFFF,

    // Margins
    heading_margin: { top: 1.0, bottom: 0.1 }
    paragraph_margin: { top: 0.33, bottom: 0.33 }

    // Text styles
    draw_normal: { ... }
    draw_italic: { ... }
    draw_bold: { ... }
    draw_fixed: { ... }

    // Link template
    a = <HtmlLink> {
        hover_color: #00AAFF
        pressed_color: #0066CC
    }
}
```

### Supported HTML Tags

| Tag | Description | Attributes |
|-----|-------------|------------|
| `<h1>` - `<h6>` | Headings | - |
| `<p>` | Paragraph | - |
| `<b>`, `<strong>` | Bold | - |
| `<i>`, `<em>` | Italic | - |
| `<u>` | Underline | - |
| `<del>`, `<s>`, `<strike>` | Strikethrough | - |
| `<code>` | Inline code | - |
| `<pre>` | Preformatted | - |
| `<blockquote>` | Quote block | - |
| `<ul>` | Unordered list | - |
| `<ol>` | Ordered list | `start`, `type` |
| `<li>` | List item | `value` |
| `<a>` | Link | `href` |
| `<br>` | Line break | - |
| `<hr>`, `<sep>` | Separator | - |
| `<sub>` | Subscript | - |
| `<sup>` | Superscript | - |

### Ordered List Types

```rust
#[derive(Copy, Clone, Live)]
pub enum OrderedListType {
    Numbers,      // 1, 2, 3, ...
    UpperAlpha,   // A, B, C, ...
    LowerAlpha,   // a, b, c, ...
    UpperRoman,   // I, II, III, ...
    LowerRoman,   // i, ii, iii, ...
}
```

### Link Click Handling

```rust
#[derive(Debug, Clone, DefaultNone)]
pub enum HtmlLinkAction {
    Clicked { url: String, key_modifiers: KeyModifiers },
    SecondaryClicked { url: String, key_modifiers: KeyModifiers },
    None,
}

// In your widget's handle_event
for action in cx.capture_actions(|cx| self.view.handle_event(cx, event, scope)) {
    match action.cast::<HtmlLinkAction>() {
        HtmlLinkAction::Clicked { url, key_modifiers } => {
            log!("Link clicked: {} (modifiers: {:?})", url, key_modifiers);
        }
        HtmlLinkAction::SecondaryClicked { url, .. } => {
            log!("Right-click on link: {}", url);
        }
        _ => {}
    }
}
```

---

## 3. TextFlow Widget (Foundation)

TextFlow is the underlying rendering engine for both Markdown and Html.

### Key Methods

```rust
impl TextFlow {
    // Text rendering
    pub fn draw_text(&mut self, cx: &mut Cx2d, text: &str);

    // Blocks
    pub fn begin(&mut self, cx: &mut Cx2d, walk: Walk);
    pub fn end(&mut self, cx: &mut Cx2d);

    // Code blocks
    pub fn begin_code(&mut self, cx: &mut Cx2d);
    pub fn end_code(&mut self, cx: &mut Cx2d);

    // Quote blocks
    pub fn begin_quote(&mut self, cx: &mut Cx2d);
    pub fn end_quote(&mut self, cx: &mut Cx2d);

    // List items
    pub fn begin_list_item(&mut self, cx: &mut Cx2d, marker: &str, padding: f64);
    pub fn end_list_item(&mut self, cx: &mut Cx2d);

    // Separators
    pub fn sep(&mut self, cx: &mut Cx2d);

    // Font size manipulation
    pub fn push_size_rel_scale(&mut self, scale: f64);  // Relative to current
    pub fn push_size_abs_scale(&mut self, scale: f64);  // Relative to base
    pub fn pop_size(&mut self);
}
```

### Creating Custom Rich Text Widget

```rust
#[derive(Live, LiveHook, Widget)]
pub struct MyRichText {
    #[deref] text_flow: TextFlow,
    #[live] content: ArcStringMut,
}

impl Widget for MyRichText {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.text_flow.begin(cx, walk);

        // Parse and render your custom format
        for segment in parse_my_format(&self.content) {
            match segment {
                Segment::Text(t) => self.text_flow.draw_text(cx, &t),
                Segment::Bold(t) => {
                    self.text_flow.bold.inc();
                    self.text_flow.draw_text(cx, &t);
                    self.text_flow.bold.dec();
                }
                Segment::Code(t) => {
                    self.text_flow.begin_code(cx);
                    self.text_flow.draw_text(cx, &t);
                    self.text_flow.end_code(cx);
                }
            }
        }

        self.text_flow.end(cx);
        DrawStep::done()
    }
}
```

---

## 4. Production Pattern: AI Chat Markdown (Moly)

Real-world example from Moly AI chat application.

### Message Markdown Widget

```rust
live_design! {
    use makepad_code_editor::code_view::CodeView;

    pub MessageMarkdown = <Markdown> {
        padding: 0
        margin: 0
        paragraph_spacing: 16
        heading_base_scale: 1.6
        font_color: #000
        width: Fill, height: Fit
        font_size: 11.0

        // Custom code blocks with syntax highlighting
        use_code_block_widget: true

        code_block = <View> {
            width: Fill
            height: Fit
            flow: Down
            padding: 0

            // Header with language + copy button
            header = <View> {
                width: Fill
                height: Fit
                flow: Right
                padding: { left: 10, right: 10, top: 5, bottom: 5 }
                draw_bg: { color: #2D3748 }

                language_label = <Label> {
                    text: ""
                    draw_text: { color: #A0AEC0 }
                }

                <Filler> {}

                copy_code_button = <Button> {
                    text: "Copy"
                    draw_text: { color: #A0AEC0 }
                }
            }

            // Code content with syntax highlighting
            code_view = <CodeView> {
                editor: {
                    width: Fill
                    height: Fit
                    draw_bg: { color: #1D2330 }

                    // Syntax highlighting colors
                    token_colors: {
                        whitespace: #A8B5D1,
                        delimiter: #A8B5D1,
                        branch_keyword: #D2A6EF,  // purple
                        constant: #FFD9AF,
                        identifier: #A8B5D1,
                        number: #FFD9AF,
                        string: #58FFC7,           // cyan
                        function: #82AAFF,         // blue
                        typename: #FCF9C3,         // yellow
                        comment: #506686,          // gray
                    }
                }
            }
        }

        list_item_layout: { padding: { left: 10, right: 10, top: 6, bottom: 0 } }
        quote_layout: { padding: { top: 10, bottom: 10 } }
    }
}
```

### Code Copy Implementation

```rust
fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
    for action in cx.capture_actions(|cx| self.view.handle_event(cx, event, scope)) {
        // Find copy button action
        if let Some(wa) = action.as_widget_action() {
            if wa.widget().as_button().pressed(&action) {
                // Get code_view from the code_block
                let code_view = wa.widget().widget(id!(code_view));
                let text_to_copy = code_view.as_code_view().text();
                cx.copy_to_clipboard(&text_to_copy);
            }
        }
    }
}
```

### Dynamic Message Rendering

```rust
impl StandardMessageContent {
    fn set_content(&mut self, cx: &mut Cx, content: &MessageContent) {
        let markdown = self.view.widget(id!(markdown)).as_markdown();

        // Set main text content
        markdown.set_text(cx, &content.text);

        // Or with tool calls formatted as markdown
        if !content.tool_calls.is_empty() {
            let formatted = format!(
                "{}\n\n🔧 **Tool call:** `{}`",
                content.text,
                content.tool_calls[0].name
            );
            markdown.set_text(cx, &formatted);
        }
    }
}
```

---

## 5. Styling Reference

### Draw Block Shader Properties

```rust
draw_block: {
    // Separator line
    line_color: #333333
    sep_color: #444444

    // Quote styling
    quote_bg_color: #1A1A1A
    quote_fg_color: #CCCCCC

    // Code styling
    code_color: #1E1E1E

    // Border radius for blocks
    radius: 4.0
}
```

### Theme Integration

```rust
pub Markdown = <MarkdownBase> {
    // Use theme colors
    font_size: (THEME_FONT_SIZE_P)
    font_color: (THEME_COLOR_LABEL_INNER)

    draw_block: {
        line_color: (THEME_COLOR_LABEL_INNER)
        sep_color: (THEME_COLOR_SHADOW)
        quote_bg_color: (THEME_COLOR_BG_HIGHLIGHT)
        code_color: (THEME_COLOR_BG_HIGHLIGHT)
    }
}
```

---

## 6. Common Patterns

### Pattern: Streaming Markdown (SSE)

```rust
// In async handler
fn on_sse_chunk(&mut self, cx: &mut Cx, chunk: &str) {
    // Append to accumulated content
    self.content.push_str(chunk);

    // Update markdown widget
    let markdown = self.view.widget(id!(markdown)).as_markdown();
    markdown.set_text(cx, &self.content);

    // Redraw
    self.redraw(cx);
}
```

### Pattern: Link with External Browser

```rust
fn handle_link_click(&mut self, cx: &mut Cx, url: &str) {
    // Using robius-open for cross-platform
    if let Ok(uri) = robius_open::Uri::new(url) {
        let _ = uri.open();
    }
}
```

### Pattern: Citation Links with Preview

```rust
#[derive(Live, Widget)]
pub struct Citation {
    #[live] url: Option<String>,
    #[live] title: String,
    #[live] favicon: Option<String>,
}

impl Citation {
    fn set_url(&mut self, cx: &mut Cx, url: String) {
        self.url = Some(url.clone());

        // Parse host for initial display
        if let Ok(parsed) = url::Url::parse(&url) {
            self.title = parsed.host_str().unwrap_or("Link").to_string();
        }

        // Async fetch actual title/favicon
        self.fetch_metadata(cx, url);
    }
}
```

---

## 7. Migration Notes

### From HTML string to Markdown

```rust
// Before: HTML
<Html> { body: "<b>Bold</b> text" }

// After: Markdown
<Markdown> { body: "**Bold** text" }
```

### Handling Missing Typst Support

Makepad does not currently have native Typst support. For math/scientific content:

1. Use Markdown with math blocks: `$$\int f(x)dx$$`
2. Enable math rendering: `use_math_widget: true`
3. Or render Typst to images externally and embed as `<Image>`

---

## Summary

| Task | Widget | Method |
|------|--------|--------|
| Display markdown docs | `Markdown` | `set_text(cx, &str)` |
| Display HTML content | `Html` | `set_text(cx, &str)` |
| Handle link clicks | Both | `MarkdownAction::LinkNavigated` / `HtmlLinkAction::Clicked` |
| Syntax highlighting | Custom | `CodeView` from `makepad-code-editor` |
| Custom rich text | `TextFlow` | Build on base engine |
