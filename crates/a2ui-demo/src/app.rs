//! A2UI Demo Application
//!
//! Demonstrates the A2UI protocol rendering with:
//! - Static mode: Load product catalog JSON data directly
//! - Streaming mode: Connect to A2A server for payment checkout UI

use makepad_component::a2ui::*;
use makepad_component::widgets::button::MpButtonAction;
use makepad_widgets::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// ============================================================================
// Theme System
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Theme {
    #[default]
    DarkPurple,
    Light,
    Soft,
}

impl Theme {
    fn from_index(index: usize) -> Self {
        match index {
            0 => Theme::DarkPurple,
            1 => Theme::Light,
            2 => Theme::Soft,
            _ => Theme::DarkPurple,
        }
    }

    fn to_index(self) -> usize {
        match self {
            Theme::DarkPurple => 0,
            Theme::Light => 1,
            Theme::Soft => 2,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Theme::DarkPurple => "Dark Purple",
            Theme::Light => "Cloud White",
            Theme::Soft => "Soft Gray",
        }
    }
}

struct ThemeColors {
    bg_primary: Vec4,
    bg_surface: Vec4,
    text_primary: Vec4,
    text_secondary: Vec4,
    accent: Vec4,
    accent_secondary: Vec4,
    status_color: Vec4,
}

impl Theme {
    fn colors(self) -> ThemeColors {
        match self {
            Theme::DarkPurple => ThemeColors {
                bg_primary: vec4(0.102, 0.102, 0.180, 1.0),      // #1a1a2e
                bg_surface: vec4(0.133, 0.133, 0.267, 1.0),      // #222244
                text_primary: vec4(1.0, 1.0, 1.0, 1.0),          // #FFFFFF
                text_secondary: vec4(0.533, 0.533, 0.533, 1.0),  // #888888
                accent: vec4(0.0, 0.4, 0.8, 1.0),                // #0066CC
                accent_secondary: vec4(0.0, 0.667, 0.4, 1.0),    // #00AA66
                status_color: vec4(0.298, 0.686, 0.314, 1.0),    // #4CAF50
            },
            Theme::Light => ThemeColors {
                bg_primary: vec4(0.961, 0.961, 0.969, 1.0),      // #f5f5f7 (iOS-like)
                bg_surface: vec4(1.0, 1.0, 1.0, 1.0),            // #FFFFFF
                text_primary: vec4(0.11, 0.11, 0.118, 1.0),      // #1c1c1e
                text_secondary: vec4(0.557, 0.557, 0.576, 1.0),  // #8e8e93
                accent: vec4(0.0, 0.478, 1.0, 1.0),              // #007AFF (iOS blue)
                accent_secondary: vec4(0.204, 0.78, 0.349, 1.0), // #34C759 (iOS green)
                status_color: vec4(0.204, 0.78, 0.349, 1.0),     // #34C759
            },
            Theme::Soft => ThemeColors {
                // Soft Gray - mid-tone between dark and light
                bg_primary: vec4(0.435, 0.455, 0.490, 1.0),      // #6f7479 (medium gray-blue)
                bg_surface: vec4(0.533, 0.553, 0.588, 1.0),      // #888d96 (lighter gray)
                text_primary: vec4(1.0, 1.0, 1.0, 1.0),          // #FFFFFF
                text_secondary: vec4(0.85, 0.85, 0.88, 1.0),     // #d9d9e0 (light gray)
                accent: vec4(0.0, 0.4, 0.8, 1.0),                // #0066CC (same blue as Dark Purple)
                accent_secondary: vec4(0.0, 0.667, 0.4, 1.0),    // #00AA66 (same green as Dark Purple)
                status_color: vec4(0.298, 0.686, 0.314, 1.0),    // #4CAF50 (same green)
            },
        }
    }

    /// Get A2UI surface theme colors for this theme
    fn a2ui_colors(self) -> A2uiThemeColors {
        match self {
            Theme::DarkPurple => A2uiThemeColors::dark_purple(),
            Theme::Light => A2uiThemeColors::light(),
            Theme::Soft => A2uiThemeColors::soft(),
        }
    }
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use makepad_component::theme::colors::*;
    use makepad_component::a2ui::surface::*;
    use makepad_component::widgets::dropdown::*;
    use makepad_component::widgets::button::*;

    // Main Application
    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                show_bg: true
                width: Fill
                height: Fill

                body = <View> {
                    width: Fill
                    height: Fill
                    flow: Down
                    padding: 20.0
                    spacing: 16.0
                    show_bg: true
                    draw_bg: { color: #1a1a2e }

                    // Header row: Title on left, Theme dropdown on right
                    header_row = <View> {
                        width: Fill
                        height: Fit
                        flow: Right
                        align: { y: 0.5 }

                        // Title and description column
                        <View> {
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 4.0

                            // Title - changes based on mode
                            title_label = <Label> {
                                text: "A2UI Demo"
                                draw_text: {
                                    text_style: <THEME_FONT_BOLD> { font_size: 24.0 }
                                    color: #FFFFFF
                                }
                            }

                            // Description
                            desc_label = <Label> {
                                text: "Static: Product Catalog | Streaming: Payment Checkout"
                                draw_text: {
                                    text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                                    color: #888888
                                }
                            }
                        }

                        // Theme dropdown in top-right corner
                        theme_dropdown = <MpDropdownSmall> {
                            width: Fit
                            height: Fit
                            labels: ["Dark Purple", "Cloud White", "Soft Gray"]
                            selected_item: 0
                        }
                    }

                    // Control buttons row
                    controls_row = <View> {
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 10.0

                        // Load static data button
                        load_btn = <MpButton> {
                            text: "🛒 Product Catalog"
                            draw_text: { color: #FFFFFF }
                            draw_bg: {
                                color: #0066CC
                                color_hover: #0055AA
                                color_pressed: #004488
                            }
                        }

                        // Math charts demo button
                        math_btn = <Button> {
                            text: "Math Charts"
                            draw_text: { color: #FFFFFF }
                            draw_bg: { color: #AA6600 }
                        }

                        // Connect to server button
                        connect_btn = <MpButton> {
                            text: "🎨 Live Editor"
                            draw_text: { color: #FFFFFF }
                            draw_bg: {
                                color: #00AA66
                                color_hover: #009955
                                color_pressed: #008844
                            }
                        }

                        // Server URL input
                        server_url = <Label> {
                            text: "localhost:8081"
                            draw_text: { color: #666666 }
                        }
                    }

                    // Status label - green color for visibility
                    status_label = <Label> {
                        text: "Select a demo mode above"
                        draw_text: {
                            color: #4CAF50
                            text_style: { font_size: 16.0 }
                        }
                    }

                    // A2UI Surface container with scroll
                    surface_container = <ScrollYView> {
                        width: Fill
                        height: Fill
                        show_bg: true
                        draw_bg: { color: #222244 }

                        <View> {
                            width: Fill
                            height: Fit
                            padding: 16.0

                            a2ui_surface = <A2uiSurface> {
                                width: Fill
                                height: Fit
                            }
                        }
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,

    #[rust]
    loaded: bool,

    #[rust]
    host: Option<A2uiHost>,

    /// Live SSE connection for real-time streaming updates
    #[rust]
    live_host: Option<A2uiHost>,

    #[rust]
    is_streaming: bool,

    #[rust]
    live_mode: bool,

    #[rust]
    last_poll_time: f64,

    #[rust]
    last_content_hash: u64,

    #[rust]
    poll_timer: Timer,
    current_theme: Theme,

    /// Currently playing audio URL (None = not playing)
    #[rust]
    playing_audio_component_id: Option<String>,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_component::live_design(cx);
    }
}

impl App {
    /// Apply the current theme colors to all UI elements
    fn apply_theme(&mut self, cx: &mut Cx) {
        let colors = self.current_theme.colors();

        // Apply body background (main container)
        self.ui.view(ids!(body)).apply_over(cx, live! {
            draw_bg: { color: (colors.bg_primary) }
        });

        // Apply header row background (in case it needs distinction)
        self.ui.view(ids!(header_row)).apply_over(cx, live! {
            draw_bg: { color: (colors.bg_primary) }
        });

        // Apply controls row background
        self.ui.view(ids!(controls_row)).apply_over(cx, live! {
            draw_bg: { color: (colors.bg_primary) }
        });

        // Apply title color
        self.ui.label(ids!(title_label)).apply_over(cx, live! {
            draw_text: { color: (colors.text_primary) }
        });

        // Apply description color
        self.ui.label(ids!(desc_label)).apply_over(cx, live! {
            draw_text: { color: (colors.text_secondary) }
        });

        // Apply button colors - keep text white for contrast
        let white = vec4(1.0, 1.0, 1.0, 1.0);

        // Calculate hover/pressed colors (slightly darker versions)
        let accent_hover = vec4(
            colors.accent.x * 0.85,
            colors.accent.y * 0.85,
            colors.accent.z * 0.85,
            1.0
        );
        let accent_pressed = vec4(
            colors.accent.x * 0.7,
            colors.accent.y * 0.7,
            colors.accent.z * 0.7,
            1.0
        );
        let secondary_hover = vec4(
            colors.accent_secondary.x * 0.85,
            colors.accent_secondary.y * 0.85,
            colors.accent_secondary.z * 0.85,
            1.0
        );
        let secondary_pressed = vec4(
            colors.accent_secondary.x * 0.7,
            colors.accent_secondary.y * 0.7,
            colors.accent_secondary.z * 0.7,
            1.0
        );

        self.ui.button(ids!(load_btn)).apply_over(cx, live! {
            draw_bg: {
                color: (colors.accent)
                color_hover: (accent_hover)
                color_pressed: (accent_pressed)
            }
            draw_text: { color: (white) }
        });

        self.ui.button(ids!(connect_btn)).apply_over(cx, live! {
            draw_bg: {
                color: (colors.accent_secondary)
                color_hover: (secondary_hover)
                color_pressed: (secondary_pressed)
            }
            draw_text: { color: (white) }
        });

        // Apply server URL label color
        self.ui.label(ids!(server_url)).apply_over(cx, live! {
            draw_text: { color: (colors.text_secondary) }
        });

        // Apply status label color
        self.ui.label(ids!(status_label)).apply_over(cx, live! {
            draw_text: { color: (colors.status_color) }
        });

        // Apply surface container background
        self.ui.view(ids!(surface_container)).apply_over(cx, live! {
            draw_bg: { color: (colors.bg_surface) }
        });

        // Apply theme-appropriate dropdown styling
        let is_light = self.current_theme == Theme::Light;
        let dropdown_text = if is_light {
            vec4(0.04, 0.04, 0.04, 1.0)  // dark text
        } else {
            vec4(1.0, 1.0, 1.0, 1.0)     // white text
        };
        let dropdown_bg = if is_light {
            vec4(1.0, 1.0, 1.0, 1.0)     // white bg
        } else {
            vec4(0.2, 0.2, 0.33, 1.0)    // dark purple bg
        };
        let dropdown_border = if is_light {
            vec4(0.83, 0.83, 0.83, 1.0)  // light border
        } else {
            vec4(0.33, 0.33, 0.47, 1.0)  // dark border
        };

        self.ui.drop_down(ids!(theme_dropdown)).apply_over(cx, live! {
            draw_text: { color: (dropdown_text) }
            draw_bg: {
                color: (dropdown_bg)
                border_color: (dropdown_border)
            }
        });

        // Apply theme to A2UI surface content
        let surface_ref = self.ui.widget(ids!(a2ui_surface));
        if let Some(mut surface) = surface_ref.borrow_mut::<A2uiSurface>() {
            let a2ui_colors = self.current_theme.a2ui_colors();
            surface.set_theme_colors(cx, &a2ui_colors);
        }

        self.ui.redraw(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle theme dropdown selection
        if let Some(index) = self.ui.drop_down(ids!(theme_dropdown)).selected(&actions) {
            let new_theme = Theme::from_index(index);
            if new_theme != self.current_theme {
                self.current_theme = new_theme;
                self.apply_theme(cx);
                log!("Theme changed to: {:?}", self.current_theme);
            }
        }

        // Handle "Math Charts" button click
        if self.ui.button(ids!(math_btn)).clicked(&actions) {
            self.load_math_charts(cx);
        }

        // Handle "Connect to Server" button click
        if self.ui.button(ids!(connect_btn)).clicked(&actions) {
            self.connect_to_server(cx);
        // Handle "Load Static Data" button click (MpButton)
        let load_btn_ref = self.ui.widget(ids!(load_btn));
        if let Some(item) = actions.find_widget_action(load_btn_ref.widget_uid()) {
            if matches!(item.cast::<MpButtonAction>(), MpButtonAction::Clicked) {
                self.load_a2ui_data(cx);
            }
        }

        // Handle "Connect to Server" button click (MpButton)
        let connect_btn_ref = self.ui.widget(ids!(connect_btn));
        if let Some(item) = actions.find_widget_action(connect_btn_ref.widget_uid()) {
            if matches!(item.cast::<MpButtonAction>(), MpButtonAction::Clicked) {
                self.connect_to_server(cx);
            }
        }

        // Handle A2UI surface actions
        let surface_ref = self.ui.widget(ids!(a2ui_surface));
        if let Some(item) = actions.find_widget_action(surface_ref.widget_uid()) {
            match item.cast::<A2uiSurfaceAction>() {
                A2uiSurfaceAction::UserAction(user_action) => {
                    // If connected to server, forward the action
                    if let Some(host) = &mut self.host {
                        if let Err(e) = host.send_action(&user_action) {
                            log!("Failed to send action to server: {}", e);
                        }
                        // Handle payment actions
                        match user_action.action.name.as_str() {
                            "confirmPayment" => {
                                self.ui.label(ids!(status_label)).set_text(
                                    cx,
                                    "✅ Processing payment...",
                                );
                            }
                            "cancelPayment" => {
                                self.ui.label(ids!(status_label)).set_text(
                                    cx,
                                    "❌ Payment cancelled",
                                );
                            }
                            _ => {
                                self.ui.label(ids!(status_label)).set_text(
                                    cx,
                                    &format!("📤 Action: {}", user_action.action.name),
                                );
                            }
                        }
                    } else {
                        // Handle locally (static mode)
                        if user_action.action.name == "addToCart" {
                            if let Some(product_id) = user_action.action.context.get("productId") {
                                self.ui.label(ids!(status_label)).set_text(
                                    cx,
                                    &format!("🛒 Added {} to cart!", product_id),
                                );
                            }
                        } else {
                            self.ui.label(ids!(status_label)).set_text(
                                cx,
                                &format!("🎯 Action: {}", user_action.action.name),
                            );
                        }
                    }
                    self.ui.redraw(cx);
                }
                A2uiSurfaceAction::PlayAudio { component_id, url, title } => {
                    // Toggle: if same component is playing, stop it
                    if self.playing_audio_component_id.as_ref() == Some(&component_id) {
                        // Stop playing
                        #[cfg(target_os = "macos")]
                        {
                            let _ = std::process::Command::new("pkill")
                                .args(["-9", "afplay"])
                                .status();
                        }
                        self.playing_audio_component_id = None;

                        // Update surface state for button display
                        let surface = self.ui.a2ui_surface(ids!(a2ui_surface));
                        surface.set_playing_component(None);

                        self.ui.label(ids!(status_label)).set_text(
                            cx,
                            &format!("⏹ Stopped: {}", title),
                        );
                        log!("Stopped: {}", title);
                    } else {
                        // Stop any current playback first
                        #[cfg(target_os = "macos")]
                        {
                            let _ = std::process::Command::new("pkill")
                                .args(["-9", "afplay"])
                                .status();
                        }

                        log!("PlayAudio: {} - {}", title, url);
                        self.playing_audio_component_id = Some(component_id.clone());

                        // Update surface state for button display
                        let surface = self.ui.a2ui_surface(ids!(a2ui_surface));
                        surface.set_playing_component(Some(component_id.clone()));

                        self.ui.label(ids!(status_label)).set_text(
                            cx,
                            &format!("🎵 Playing: {}", title),
                        );

                        // Download to resources directory and play
                        let title_clone = title.clone();
                        let url_clone = url.clone();
                        std::thread::spawn(move || {
                            // Determine file extension from URL
                            let ext = if url_clone.contains(".mp4") { "mp4" }
                                else if url_clone.contains(".mp3") { "mp3" }
                                else { "mp3" };

                            let filename = format!("audio_{}.{}",
                                title_clone.chars().filter(|c| c.is_alphanumeric()).collect::<String>(),
                                ext
                            );
                            let download_path = format!("crates/a2ui-demo/resources/{}", filename);

                            log!("Downloading {} to {}", url_clone, download_path);

                            // Download using curl
                            let status = std::process::Command::new("curl")
                                .args(["-L", "-o", &download_path, &url_clone])
                                .status();

                            match status {
                                Ok(s) if s.success() => {
                                    log!("Download complete: {}", download_path);

                                    // Play with system player
                                    #[cfg(target_os = "macos")]
                                    {
                                        let _ = std::process::Command::new("afplay")
                                            .arg(&download_path)
                                            .spawn();
                                    }
                                    #[cfg(target_os = "linux")]
                                    {
                                        let _ = std::process::Command::new("aplay")
                                            .arg(&download_path)
                                            .spawn();
                                    }
                                    #[cfg(target_os = "windows")]
                                    {
                                        let _ = std::process::Command::new("cmd")
                                            .args(["/C", "start", "", &download_path])
                                            .spawn();
                                    }
                                }
                                _ => {
                                    log!("Download failed");
                                }
                            }
                        });
                    }
                    self.ui.redraw(cx);
                }
                A2uiSurfaceAction::DataModelChanged { surface_id, path, value } => {
                    // Update the data model with the new value
                    if let Some(mut surface) = surface_ref.borrow_mut::<A2uiSurface>() {
                        if let Some(processor) = surface.processor_mut() {
                            if let Some(data_model) = processor.get_data_model_mut(&surface_id) {
                                // Radio button behavior for payment methods (streaming mode)
                                let payment_methods = [
                                    "/payment/creditCard",
                                    "/payment/paypal",
                                    "/payment/alipay",
                                    "/payment/wechat",
                                ];

                                if payment_methods.contains(&path.as_str()) {
                                    // If setting to true, deselect all others first
                                    if value == serde_json::Value::Bool(true) {
                                        for method in &payment_methods {
                                            if *method != path {
                                                data_model.set(method, serde_json::Value::Bool(false));
                                            }
                                        }
                                    }
                                }

                                data_model.set(&path, value.clone());

                                // Computed value: when maxPrice changes, update maxPriceDisplay
                                if path == "/filters/maxPrice" {
                                    if let Some(price) = value.as_f64() {
                                        let display = format!("${:.0}", price);
                                        data_model.set("/filters/maxPriceDisplay", serde_json::Value::String(display));
                                    }
                                }
                            }
                        }
                    }
                    // Update status to show the change
                    self.ui.label(ids!(status_label)).set_text(
                        cx,
                        &format!("📝 Updated {}", path),
                    );
                    self.ui.redraw(cx);
                }
                _ => {}
            }
        }
    }

    fn connect_to_server(&mut self, cx: &mut Cx) {
        // Always disconnect first to allow reconnection
        if self.host.is_some() {
            log!("connect_to_server: Clearing existing host");
            self.host = None;
        }
        if self.live_host.is_some() {
            log!("connect_to_server: Clearing existing live_host");
            self.live_host = None;
        }

        // Clear surface BEFORE connecting - this ensures a fresh start
        // The BeginRendering message will create a new surface
        let surface_ref = self.ui.widget(ids!(a2ui_surface));
        if let Some(mut surface) = surface_ref.borrow_mut::<A2uiSurface>() {
            surface.clear();
        }

        // Update title for streaming mode
        self.ui.label(ids!(title_label)).set_text(cx, "🎨 Live A2UI Editor");

        // Connect to /rpc for initial UI load
        let config = A2uiHostConfig {
            url: "http://localhost:8082/rpc".to_string(),
            auth_token: None,
        };

        let mut host = A2uiHost::new(config);

        match host.connect("Live mode") {
            Ok(()) => {
                self.ui.label(ids!(status_label)).set_text(cx, "🔗 Connecting to live server...");
                self.host = Some(host);
                self.is_streaming = true;
                self.live_mode = true;
                self.last_poll_time = cx.seconds_since_app_start();
                self.loaded = false;

                // Also connect to /live for real-time streaming updates
                self.connect_live_stream(cx);
            }
            Err(e) => {
                self.ui.label(ids!(status_label)).set_text(cx, &format!("❌ Connection failed: {}", e));
            }
        }

        self.ui.redraw(cx);
    }

    fn connect_live_stream(&mut self, _cx: &mut Cx) {
        // Connect to /live SSE endpoint for real-time component updates (using GET)
        let live_config = A2uiHostConfig {
            url: "http://localhost:8081/live".to_string(),
            auth_token: None,
        };

        let mut live_host = A2uiHost::new(live_config);

        // Use connect_sse for GET-based SSE connection
        match live_host.connect_sse() {
            Ok(()) => {
                log!("🔴 Connected to /live SSE for real-time streaming");
                self.live_host = Some(live_host);
            }
            Err(e) => {
                log!("Failed to connect to /live: {}", e);
            }
        }
    }

    fn reconnect_live(&mut self, cx: &mut Cx) {
        // Reconnect to get updates (don't clear surface - we want incremental updates)
        let config = A2uiHostConfig {
            url: "http://localhost:8082/rpc".to_string(),
            auth_token: None,
        };

        let mut host = A2uiHost::new(config);

        match host.connect("Live poll") {
            Ok(()) => {
                self.host = Some(host);
                self.is_streaming = true;
            }
            Err(_) => {
                // Silent retry on failure
            }
        }
    }

    fn disconnect(&mut self, cx: &mut Cx) {
        self.host = None;
        self.is_streaming = false;
        self.ui.label(ids!(status_label)).set_text(cx, "🔌 Disconnected from server");
        self.ui.redraw(cx);
    }

    fn poll_host(&mut self, cx: &mut Cx) {
        let Some(host) = &mut self.host else {
            return;
        };

        let events = host.poll_all();
        if events.is_empty() {
            return;
        }

        // Collect all messages first, then hash the batch to detect duplicates
        let mut messages: Vec<A2uiMessage> = Vec::new();
        let mut had_error = false;
        let mut error_msg = String::new();
        let mut had_disconnect = false;
        let mut task_state = None;

        for event in events {
            match event {
                A2uiHostEvent::Connected => {}
                A2uiHostEvent::Message(msg) => {
                    messages.push(msg);
                }
                A2uiHostEvent::TaskStatus { task_id: _, state } => {
                    task_state = Some(state);
                }
                A2uiHostEvent::Error(e) => {
                    had_error = true;
                    error_msg = e;
                }
                A2uiHostEvent::Disconnected => {
                    had_disconnect = true;
                }
            }
        }

        // Hash the entire batch of messages to detect duplicates across reconnections
        let mut needs_redraw = false;

        if !messages.is_empty() {
            let batch_hash = {
                let mut hasher = DefaultHasher::new();
                for msg in &messages {
                    format!("{:?}", msg).hash(&mut hasher);
                }
                hasher.finish()
            };

            if batch_hash != self.last_content_hash {
                self.last_content_hash = batch_hash;

                let surface_ref = self.ui.widget(ids!(a2ui_surface));
                for msg in messages {
                    log!("Received A2uiMessage: {:?}", msg);
                    if let Some(mut surface) = surface_ref.borrow_mut::<A2uiSurface>() {
                        let events = surface.process_message(msg);
                        log!("Processed streaming message, {} events", events.len());
                        for event in &events {
                            log!("  Event: {:?}", event);
                        }
                    }
                }
                if self.live_mode {
                    self.ui.label(ids!(status_label)).set_text(cx, "🎨 UI Updated from ui_live.json");
                    self.loaded = true;
                    self.poll_timer = Timer::default(); // Stop polling — no data, no refresh
                } else {
                    self.ui.label(ids!(status_label)).set_text(cx, "💳 Streaming payment UI...");
                }
                needs_redraw = true;
            }
        }

        if let Some(state) = task_state {
            if !self.live_mode {
                if state == "completed" {
                    self.ui.label(ids!(status_label)).set_text(cx, "✅ Payment page ready");
                } else {
                    self.ui.label(ids!(status_label)).set_text(cx, &format!("💳 {}", state));
                }
                needs_redraw = true;
            }
        }

        if had_error {
            self.ui.label(ids!(status_label)).set_text(cx, &format!("❌ Error: {}", error_msg));
            needs_redraw = true;
        }

        if had_disconnect {
            self.host = None;
            self.is_streaming = false;
            if !self.live_mode {
                self.ui.label(ids!(status_label)).set_text(cx, "⚫ Disconnected from server");
                needs_redraw = true;
            }
        }

        if needs_redraw {
            self.ui.redraw(cx);
        }
    }

    /// Poll for real-time streaming updates from /live SSE endpoint
    fn poll_live_host(&mut self, cx: &mut Cx) {
        let Some(live_host) = &mut self.live_host else {
            return;
        };

        let events = live_host.poll_all();
        if events.is_empty() {
            return;
        }

        let surface_ref = self.ui.widget(ids!(a2ui_surface));
        let mut needs_redraw = false;

        for event in events {
            match event {
                A2uiHostEvent::Connected => {
                    log!("Live stream connected - ready for real-time updates");
                }
                A2uiHostEvent::Message(msg) => {
                    log!("🔴 LIVE: Received streaming component: {:?}", msg);
                    if let Some(mut surface) = surface_ref.borrow_mut::<A2uiSurface>() {
                        let events = surface.process_message(msg);
                        log!("🔴 LIVE: Processed {} events", events.len());
                    }
                    self.ui.label(ids!(status_label)).set_text(cx, "🔴 Streaming component...");
                    needs_redraw = true;
                }
                A2uiHostEvent::TaskStatus { task_id: _, state } => {
                    log!("Live stream task status: {}", state);
                }
                A2uiHostEvent::Error(e) => {
                    log!("Live stream error: {}", e);
                }
                A2uiHostEvent::Disconnected => {
                    log!("Live stream disconnected, will reconnect...");
                    self.live_host = None;
                }
            }
        }

        if needs_redraw {
            self.ui.redraw(cx);
        }
    }

    fn load_a2ui_data(&mut self, cx: &mut Cx) {
        // Disconnect from server if connected
        if self.host.is_some() {
            self.disconnect(cx);
        }
        self.live_mode = false;

        // Clear the surface before loading new data
        let surface_ref = self.ui.widget(ids!(a2ui_surface));
        if let Some(mut surface) = surface_ref.borrow_mut::<A2uiSurface>() {
            surface.clear();
        }

        // Update title for static mode
        self.ui.label(ids!(title_label)).set_text(cx, "🛒 Product Catalog");

        // Sample A2UI JSON for a product catalog
        let a2ui_json = get_sample_product_catalog();

        // Get the A2uiSurface widget ref and process the JSON
        let surface_ref = self.ui.widget(ids!(a2ui_surface));
        let result = {
            if let Some(mut surface) = surface_ref.borrow_mut::<A2uiSurface>() {
                match surface.process_json(&a2ui_json) {
                    Ok(events) => {
                        log!("A2UI Events: {} events processed", events.len());
                        for event in &events {
                            log!("  - {:?}", event);
                        }
                        Some(events.len())
                    }
                    Err(e) => {
                        log!("Error parsing A2UI JSON: {}", e);
                        None
                    }
                }
            } else {
                log!("Could not borrow A2uiSurface");
                None
            }
        };

        // Update status label - use emoji to highlight static data mode
        if let Some(count) = result {
            self.ui.label(ids!(status_label))
                .set_text(cx, &format!("🟢 Static Mode | {} events loaded", count));
            self.loaded = true;
        } else {
            self.ui.label(ids!(status_label))
                .set_text(cx, "🔴 Error loading A2UI data");
        }

        self.ui.redraw(cx);
    }

    fn load_math_charts(&mut self, cx: &mut Cx) {
        // Disconnect from server if connected
        if self.host.is_some() {
            self.disconnect(cx);
        }
        self.live_mode = false;

        // Clear the surface before loading
        let surface_ref = self.ui.widget(ids!(a2ui_surface));
        if let Some(mut surface) = surface_ref.borrow_mut::<A2uiSurface>() {
            surface.clear();
        }

        self.ui.label(ids!(title_label)).set_text(cx, "Famous Mathematical Functions");

        // Try to load math_test.json from current directory
        let json_str = match std::fs::read_to_string("math_test.json") {
            Ok(s) => s,
            Err(e) => {
                self.ui.label(ids!(status_label))
                    .set_text(cx, &format!("Error: math_test.json not found ({}). Run: cargo run -p a2ui-demo --bin math-charts", e));
                self.ui.redraw(cx);
                return;
            }
        };

        let surface_ref = self.ui.widget(ids!(a2ui_surface));
        let result = {
            if let Some(mut surface) = surface_ref.borrow_mut::<A2uiSurface>() {
                match surface.process_json(&json_str) {
                    Ok(events) => {
                        log!("Math charts: {} events processed", events.len());
                        Some(events.len())
                    }
                    Err(e) => {
                        log!("Error parsing math_test.json: {}", e);
                        None
                    }
                }
            } else {
                None
            }
        };

        if let Some(count) = result {
            self.ui.label(ids!(status_label))
                .set_text(cx, &format!("Math Demo | {} events | Chebyshev, Fourier, Rosenbrock, Himmelblau, Legendre, Rastrigin", count));
            self.loaded = true;
        } else {
            self.ui.label(ids!(status_label))
                .set_text(cx, "Error loading math charts data");
        }

        self.ui.redraw(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Auto-load math charts on startup if math_test.json exists
        if let Event::Startup = event {
            if std::path::Path::new("math_test.json").exists() {
                self.load_math_charts(cx);
            } else {
                self.connect_to_server(cx);
            }
            // Start interval timer for polling instead of continuous frame requests
            self.poll_timer = cx.start_interval(1.0);
        }

        // Only poll on timer ticks — no polling on mouse/keyboard/paint events
        if self.poll_timer.is_event(event).is_some() {
            if self.host.is_some() {
                self.poll_host(cx);
            } else if self.live_mode && !self.loaded {
                // Only reconnect if we haven't loaded data yet
                self.reconnect_live(cx);
            }
        // Apply theme and auto-connect to live server on startup
        if let Event::Startup = event {
            self.apply_theme(cx);
            self.connect_to_server(cx);
        }

        // Poll for streaming messages when connected
        if self.host.is_some() {
            self.poll_host(cx);
        }

        // Poll for real-time streaming updates from /live
        if self.live_host.is_some() {
            self.poll_live_host(cx);
        }

        // Live mode: keep the event loop running for polling
        if self.live_mode {
            if self.host.is_none() {
                // Reconnect periodically to get updates
                let current_time = cx.seconds_since_app_start();
                if current_time - self.last_poll_time > 0.2 {
                    self.last_poll_time = current_time;
                    self.reconnect_live(cx);
                }
            }
            // Reconnect live stream if disconnected
            if self.live_host.is_none() {
                self.connect_live_stream(cx);
            }
            // Always request next frame to keep polling loop active
            cx.new_next_frame();
        }

        // Capture actions from UI event handling
        let actions = cx.capture_actions(|cx| {
            self.ui.handle_event(cx, event, &mut Scope::empty());
        });

        // Handle captured actions
        self.handle_actions(cx, &actions);
    }
}

/// Get sample A2UI JSON for a product catalog with form inputs
fn get_sample_product_catalog() -> String {
    r##"[
        {
            "beginRendering": {
                "surfaceId": "main",
                "root": "root-column"
            }
        },
        {
            "surfaceUpdate": {
                "surfaceId": "main",
                "components": [
                    {
                        "id": "root-column",
                        "component": {
                            "Column": {
                                "children": {
                                    "explicitList": ["header", "filters-section", "product-list"]
                                }
                            }
                        }
                    },
                    {
                        "id": "header",
                        "component": {
                            "Text": {
                                "text": {"literalString": "Products"},
                                "usageHint": "h1"
                            }
                        }
                    },
                    {
                        "id": "filters-section",
                        "component": {
                            "Card": {
                                "child": "filters-content"
                            }
                        }
                    },
                    {
                        "id": "filters-content",
                        "component": {
                            "Column": {
                                "children": {
                                    "explicitList": ["filters-title", "search-row", "options-row", "price-row"]
                                }
                            }
                        }
                    },
                    {
                        "id": "filters-title",
                        "component": {
                            "Text": {
                                "text": {"literalString": "Filters"},
                                "usageHint": "h3"
                            }
                        }
                    },
                    {
                        "id": "search-row",
                        "component": {
                            "Row": {
                                "children": {
                                    "explicitList": ["search-label", "search-input"]
                                }
                            }
                        }
                    },
                    {
                        "id": "search-label",
                        "component": {
                            "Text": {
                                "text": {"literalString": "Search:"}
                            }
                        }
                    },
                    {
                        "id": "search-input",
                        "component": {
                            "TextField": {
                                "text": {"path": "/filters/search"},
                                "placeholder": {"literalString": "Enter product name..."}
                            }
                        }
                    },
                    {
                        "id": "options-row",
                        "component": {
                            "Row": {
                                "children": {
                                    "explicitList": ["in-stock-checkbox", "on-sale-checkbox"]
                                }
                            }
                        }
                    },
                    {
                        "id": "in-stock-checkbox",
                        "component": {
                            "CheckBox": {
                                "value": {"path": "/filters/inStock"},
                                "label": {"literalString": "In Stock Only"}
                            }
                        }
                    },
                    {
                        "id": "on-sale-checkbox",
                        "component": {
                            "CheckBox": {
                                "value": {"path": "/filters/onSale"},
                                "label": {"literalString": "On Sale"}
                            }
                        }
                    },
                    {
                        "id": "price-row",
                        "component": {
                            "Row": {
                                "children": {
                                    "explicitList": ["price-label", "price-slider", "price-value"]
                                }
                            }
                        }
                    },
                    {
                        "id": "price-label",
                        "component": {
                            "Text": {
                                "text": {"literalString": "Max Price:"}
                            }
                        }
                    },
                    {
                        "id": "price-slider",
                        "component": {
                            "Slider": {
                                "value": {"path": "/filters/maxPrice"},
                                "min": 0,
                                "max": 200,
                                "step": 10
                            }
                        }
                    },
                    {
                        "id": "price-value",
                        "component": {
                            "Text": {
                                "text": {"path": "/filters/maxPriceDisplay"}
                            }
                        }
                    },
                    {
                        "id": "product-list",
                        "component": {
                            "Column": {
                                "children": {
                                    "explicitList": ["product-1", "product-2", "product-3"]
                                }
                            }
                        }
                    },
                    {
                        "id": "product-1",
                        "component": {
                            "Card": {
                                "child": "product-1-content"
                            }
                        }
                    },
                    {
                        "id": "product-1-content",
                        "component": {
                            "Row": {
                                "children": {
                                    "explicitList": ["product-1-image", "product-1-info", "product-1-btn"]
                                }
                            }
                        }
                    },
                    {
                        "id": "product-1-image",
                        "component": {
                            "Image": {
                                "url": {"literalString": "https://example.com/headphones.jpg"},
                                "usageHint": "smallFeature"
                            }
                        }
                    },
                    {
                        "id": "product-1-info",
                        "component": {
                            "Column": {
                                "children": {
                                    "explicitList": ["product-1-name", "product-1-price"]
                                }
                            }
                        }
                    },
                    {
                        "id": "product-1-name",
                        "component": {
                            "Text": {
                                "text": {"literalString": "Premium Headphones"},
                                "usageHint": "h3"
                            }
                        }
                    },
                    {
                        "id": "product-1-price",
                        "component": {
                            "Text": {
                                "text": {"literalString": "$99.99"}
                            }
                        }
                    },
                    {
                        "id": "product-1-btn",
                        "component": {
                            "Button": {
                                "child": "product-1-btn-text",
                                "primary": true,
                                "action": {
                                    "name": "addToCart",
                                    "context": [
                                        {"key": "productId", "value": {"literalString": "SKU001"}}
                                    ]
                                }
                            }
                        }
                    },
                    {
                        "id": "product-1-btn-text",
                        "component": {
                            "Text": {
                                "text": {"literalString": "Add to Cart"}
                            }
                        }
                    },
                    {
                        "id": "product-2",
                        "component": {
                            "Card": {
                                "child": "product-2-content"
                            }
                        }
                    },
                    {
                        "id": "product-2-content",
                        "component": {
                            "Row": {
                                "children": {
                                    "explicitList": ["product-2-image", "product-2-info", "product-2-btn"]
                                }
                            }
                        }
                    },
                    {
                        "id": "product-2-image",
                        "component": {
                            "Image": {
                                "url": {"literalString": "https://example.com/mouse.jpg"},
                                "usageHint": "smallFeature"
                            }
                        }
                    },
                    {
                        "id": "product-2-info",
                        "component": {
                            "Column": {
                                "children": {
                                    "explicitList": ["product-2-name", "product-2-price"]
                                }
                            }
                        }
                    },
                    {
                        "id": "product-2-name",
                        "component": {
                            "Text": {
                                "text": {"literalString": "Wireless Mouse"},
                                "usageHint": "h3"
                            }
                        }
                    },
                    {
                        "id": "product-2-price",
                        "component": {
                            "Text": {
                                "text": {"literalString": "$49.99"}
                            }
                        }
                    },
                    {
                        "id": "product-2-btn",
                        "component": {
                            "Button": {
                                "child": "product-2-btn-text",
                                "primary": true,
                                "action": {
                                    "name": "addToCart",
                                    "context": [
                                        {"key": "productId", "value": {"literalString": "SKU002"}}
                                    ]
                                }
                            }
                        }
                    },
                    {
                        "id": "product-2-btn-text",
                        "component": {
                            "Text": {
                                "text": {"literalString": "Add to Cart"}
                            }
                        }
                    },
                    {
                        "id": "product-3",
                        "component": {
                            "Card": {
                                "child": "product-3-content"
                            }
                        }
                    },
                    {
                        "id": "product-3-content",
                        "component": {
                            "Row": {
                                "children": {
                                    "explicitList": ["product-3-image", "product-3-info", "product-3-btn"]
                                }
                            }
                        }
                    },
                    {
                        "id": "product-3-image",
                        "component": {
                            "Image": {
                                "url": {"literalString": "https://example.com/keyboard.jpg"},
                                "usageHint": "smallFeature"
                            }
                        }
                    },
                    {
                        "id": "product-3-info",
                        "component": {
                            "Column": {
                                "children": {
                                    "explicitList": ["product-3-name", "product-3-price"]
                                }
                            }
                        }
                    },
                    {
                        "id": "product-3-name",
                        "component": {
                            "Text": {
                                "text": {"literalString": "Mechanical Keyboard"},
                                "usageHint": "h3"
                            }
                        }
                    },
                    {
                        "id": "product-3-price",
                        "component": {
                            "Text": {
                                "text": {"literalString": "$129.99"}
                            }
                        }
                    },
                    {
                        "id": "product-3-btn",
                        "component": {
                            "Button": {
                                "child": "product-3-btn-text",
                                "primary": true,
                                "action": {
                                    "name": "addToCart",
                                    "context": [
                                        {"key": "productId", "value": {"literalString": "SKU003"}}
                                    ]
                                }
                            }
                        }
                    },
                    {
                        "id": "product-3-btn-text",
                        "component": {
                            "Text": {
                                "text": {"literalString": "Add to Cart"}
                            }
                        }
                    }
                ]
            }
        },
        {
            "dataModelUpdate": {
                "surfaceId": "main",
                "path": "/",
                "contents": [
                    {
                        "key": "filters",
                        "valueMap": [
                            {"key": "search", "valueString": ""},
                            {"key": "inStock", "valueBoolean": true},
                            {"key": "onSale", "valueBoolean": false},
                            {"key": "maxPrice", "valueNumber": 150},
                            {"key": "maxPriceDisplay", "valueString": "$150"}
                        ]
                    }
                ]
            }
        }
    ]"##.to_string()
}
