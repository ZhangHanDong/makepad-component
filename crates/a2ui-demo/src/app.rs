//! A2UI Demo Application
//!
//! Demonstrates the A2UI protocol rendering with:
//! - Static mode: Load JSON data directly
//! - Streaming mode: Connect to A2A server for real-time UI updates

use makepad_component::a2ui::*;
use makepad_widgets::*;

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

                    // Title
                    <Label> {
                        text: "A2UI Demo - Product Catalog"
                        draw_text: {
                            text_style: <THEME_FONT_BOLD> { font_size: 24.0 }
                            color: #FFFFFF
                        }
                    }

                    // Description
                    <Label> {
                        text: "This demo shows A2UI protocol rendering in Makepad"
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
                            text: "Load Static Data"
                            draw_text: { color: #FFFFFF }
                            draw_bg: { color: #0066CC }
                        }

                        // Connect to server button
                        connect_btn = <Button> {
                            text: "Connect to Server"
                            draw_text: { color: #FFFFFF }
                            draw_bg: { color: #00AA66 }
                        }

                        // Disconnect button
                        disconnect_btn = <Button> {
                            text: "Disconnect"
                            draw_text: { color: #FFFFFF }
                            draw_bg: { color: #CC3333 }
                            visible: false
                        }

                        // Server URL input
                        server_url = <Label> {
                            text: "http://localhost:8080/rpc"
                            draw_text: { color: #888888 }
                        }
                    }

                    // Status label
                    status_label = <Label> {
                        text: "Click a button to load A2UI data or connect to server"
                        draw_text: { color: #888888 }
                    }

                    // A2UI Surface container
                    <View> {
                        width: Fill
                        height: Fill
                        show_bg: true
                        draw_bg: {
                            color: #222244
                        }
                        padding: 16.0

                        a2ui_surface = <A2uiSurface> {
                            width: Fill
                            height: Fill
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

        // Handle "Connect to Server" button click
        if self.ui.button(ids!(connect_btn)).clicked(&actions) {
            self.connect_to_server(cx);
        }

        // Handle "Disconnect" button click
        if self.ui.button(ids!(disconnect_btn)).clicked(&actions) {
            self.disconnect(cx);
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
                        self.ui.label(ids!(status_label)).set_text(
                            cx,
                            &format!("📤 Sent action: {}", user_action.action.name),
                        );
                    } else {
                        // Handle locally (static mode)
                        if user_action.action.name == "addToCart" {
                            if let Some(product_id) = user_action.action.context.get("productId") {
                                self.ui.label(ids!(status_label)).set_text(
                                    cx,
                                    &format!("🛒 Added product {} to cart!", product_id),
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
                        &format!("📝 Updated {}: {}", path, value),
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

        let config = A2uiHostConfig {
            url: "http://localhost:8080/rpc".to_string(),
            auth_token: None,
        };

        let mut host = A2uiHost::new(config);

        match host.connect("Show me a product catalog UI") {
            Ok(()) => {
                self.ui.label(ids!(status_label)).set_text(cx, "🔗 Connecting to server...");
                self.host = Some(host);
                self.is_streaming = true;
                self.loaded = false; // Reset loaded flag so static data can be reloaded
            }
            Err(e) => {
                self.ui.label(ids!(status_label)).set_text(cx, &format!("❌ Connection failed: {}", e));
            }
        }

        self.ui.redraw(cx);
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

        let surface_ref = self.ui.widget(ids!(a2ui_surface));

        for event in events {
            match event {
                A2uiHostEvent::Connected => {
                    self.ui.label(ids!(status_label)).set_text(cx, "✅ Connected! Receiving UI...");
                }
                A2uiHostEvent::Message(msg) => {
                    log!("Received A2uiMessage: {:?}", msg);
                    if let Some(mut surface) = surface_ref.borrow_mut::<A2uiSurface>() {
                        let events = surface.process_message(msg);
                        log!("Processed streaming message, {} events", events.len());
                        for event in &events {
                            log!("  Event: {:?}", event);
                        }
                    } else {
                        log!("ERROR: Could not borrow A2uiSurface!");
                    }
                    self.ui.label(ids!(status_label)).set_text(cx, "📥 Receiving UI updates...");
                }
                A2uiHostEvent::TaskStatus { task_id, state } => {
                    self.ui.label(ids!(status_label)).set_text(
                        cx,
                        &format!("📋 Task {}: {}", task_id, state),
                    );
                }
                A2uiHostEvent::Error(e) => {
                    self.ui.label(ids!(status_label)).set_text(cx, &format!("❌ Error: {}", e));
                }
                A2uiHostEvent::Disconnected => {
                    self.ui.label(ids!(status_label)).set_text(cx, "🔌 Server disconnected");
                    self.host = None;
                    self.is_streaming = false;
                }
            }
        }

        self.ui.redraw(cx);
    }

    fn load_a2ui_data(&mut self, cx: &mut Cx) {
        // Disconnect from server if connected
        if self.host.is_some() {
            self.disconnect(cx);
        }

        // Clear the surface before loading new data
        let surface_ref = self.ui.widget(ids!(a2ui_surface));
        if let Some(mut surface) = surface_ref.borrow_mut::<A2uiSurface>() {
            surface.clear();
        }

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

        // Update status label
        if let Some(count) = result {
            self.ui.label(ids!(status_label))
                .set_text(cx, &format!("📦 Loaded static data! {} events processed.", count));
            self.loaded = true;
        } else {
            self.ui.label(ids!(status_label))
                .set_text(cx, "❌ Error loading A2UI data");
        }

        self.ui.redraw(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Auto-load static data on startup
        if let Event::Startup = event {
            self.load_a2ui_data(cx);
        }

        // Poll for streaming messages when connected
        if self.host.is_some() {
            self.poll_host(cx);
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
