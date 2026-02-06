//! A2UI Demo Application
//!
//! Demonstrates the A2UI protocol rendering with:
//! - Static mode: Load product catalog JSON data directly
//! - Streaming mode: Connect to A2A server for payment checkout UI

use makepad_component::a2ui::*;
use makepad_widgets::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use makepad_component::theme::colors::*;
    use makepad_component::a2ui::surface::*;

    // Main Application
    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                show_bg: true
                width: Fill
                height: Fill

                draw_bg: {
                    fn pixel(self) -> vec4 {
                        return #1a1a2e;
                    }
                }

                body = <View> {
                    width: Fill
                    height: Fill
                    flow: Down
                    padding: 20.0
                    spacing: 16.0

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

                    // Control buttons row
                    <View> {
                        width: Fill
                        height: Fit
                        flow: Right
                        spacing: 10.0

                        // Load static data button
                        load_btn = <Button> {
                            text: "🛒 Product Catalog"
                            draw_text: { color: #FFFFFF }
                            draw_bg: { color: #0066CC }
                        }

                        // Math charts demo button
                        math_btn = <Button> {
                            text: "Math Charts"
                            draw_text: { color: #FFFFFF }
                            draw_bg: { color: #AA6600 }
                        }

                        // Connect to server button
                        connect_btn = <Button> {
                            text: "🎨 Live Editor"
                            draw_text: { color: #FFFFFF }
                            draw_bg: { color: #00AA66 }
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
                    <ScrollYView> {
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
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_component::live_design(cx);
    }
}

impl App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle "Load Static Data" button click
        if self.ui.button(ids!(load_btn)).clicked(&actions) {
            self.load_a2ui_data(cx);
        }

        // Handle "Math Charts" button click
        if self.ui.button(ids!(math_btn)).clicked(&actions) {
            self.load_math_charts(cx);
        }

        // Handle "Connect to Server" button click
        if self.ui.button(ids!(connect_btn)).clicked(&actions) {
            self.connect_to_server(cx);
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

        // Clear surface BEFORE connecting - this ensures a fresh start
        // The BeginRendering message will create a new surface
        let surface_ref = self.ui.widget(ids!(a2ui_surface));
        if let Some(mut surface) = surface_ref.borrow_mut::<A2uiSurface>() {
            surface.clear();
        }

        // Update title for streaming mode
        self.ui.label(ids!(title_label)).set_text(cx, "🎨 Live A2UI Editor");

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
            }
            Err(e) => {
                self.ui.label(ids!(status_label)).set_text(cx, &format!("❌ Connection failed: {}", e));
            }
        }

        self.ui.redraw(cx);
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
