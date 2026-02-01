# A2UI Protocol & Makepad Renderer Guide

## Table of Contents

1. [What is A2UI](#what-is-a2ui)
2. [Protocol Overview](#protocol-overview)
3. [Message Types](#message-types)
4. [Component System](#component-system)
5. [Data Binding](#data-binding)
6. [Makepad Renderer Implementation](#makepad-renderer-implementation)
7. [Demo Application](#demo-application)
8. [Quick Start](#quick-start)

---

## What is A2UI

**A2UI (Agent-to-UI)** is an open-source declarative JSON protocol designed for AI agents to generate interactive user interfaces.

### Core Features

| Feature | Description |
|---------|-------------|
| **Declarative JSON** | Pure data description, no code execution, safe across trust boundaries |
| **LLM Optimized** | Flat adjacency list format, easy for large language models to generate |
| **Cross-Platform** | Single response renders on Web, Mobile, and Desktop |
| **Streaming** | Progressive UI updates as content generates in real-time |

### Why A2UI?

Traditional AI agents can only return text or Markdown, limiting user experience. A2UI enables AI agents to:

- Generate complete form interfaces
- Create interactive product listings
- Build payment flow pages
- Update UI state in real-time

```
┌─────────────────────────────────────────────────────────────┐
│                       AI Agent                               │
│                          │                                   │
│                     A2UI JSON                                │
│                          │                                   │
│          ┌───────────────┼───────────────┐                  │
│          ▼               ▼               ▼                  │
│      ┌───────┐      ┌───────┐      ┌───────────┐           │
│      │  Web  │      │ Mobile│      │  Desktop  │           │
│      │ (Lit) │      │(Flutter)│    │ (Makepad) │           │
│      └───────┘      └───────┘      └───────────┘           │
└─────────────────────────────────────────────────────────────┘
```

---

## Protocol Overview

### Adjacency List Model

A2UI uses a flat adjacency list structure instead of deeply nested JSON. This design is more LLM-friendly:

```json
{
  "surfaceUpdate": {
    "surfaceId": "main",
    "components": [
      {"id": "root", "component": {"Column": {"children": {"explicitList": ["title", "btn"]}}}},
      {"id": "title", "component": {"Text": {"text": {"literalString": "Hello"}}}},
      {"id": "btn", "component": {"Button": {"child": "btn-text", "action": {"name": "click"}}}}
    ]
  }
}
```

### Advantages

1. **Incremental Updates** - Only send changed components
2. **Simple References** - Reference other components by ID
3. **Easy Generation** - LLMs don't need to track nesting levels
4. **Fast Parsing** - Renderers can quickly look up components

---

## Message Types

A2UI defines 5 core message types:

### 1. beginRendering - Initialize Surface

Creates a new UI surface and sets the root component.

```json
{
  "beginRendering": {
    "surfaceId": "main",
    "root": "root-column",
    "styles": {
      "primaryColor": "#007BFF",
      "font": "Roboto"
    }
  }
}
```

### 2. surfaceUpdate - Update Components

Adds or updates components in the component tree.

```json
{
  "surfaceUpdate": {
    "surfaceId": "main",
    "components": [
      {
        "id": "greeting",
        "component": {
          "Text": {
            "text": {"literalString": "Welcome"},
            "usageHint": "h1"
          }
        }
      }
    ]
  }
}
```

### 3. dataModelUpdate - Update Data

Updates the data model, causing data-bound components to refresh automatically.

```json
{
  "dataModelUpdate": {
    "surfaceId": "main",
    "path": "/user",
    "contents": [
      {"key": "name", "valueString": "John"},
      {"key": "balance", "valueNumber": 1000.50},
      {"key": "vip", "valueBoolean": true}
    ]
  }
}
```

### 4. deleteSurface - Delete Surface

Removes an entire UI surface.

```json
{
  "deleteSurface": {
    "surfaceId": "main"
  }
}
```

### 5. userAction - User Action (Client → Server)

User-triggered action events sent back to the server.

```json
{
  "userAction": {
    "surfaceId": "main",
    "componentId": "pay-btn",
    "action": {
      "name": "confirmPayment",
      "context": {
        "amount": 344.75,
        "method": "alipay"
      }
    }
  }
}
```

---

## Component System

A2UI defines a standard component catalog in four categories:

### Layout Components

| Component | Description | Makepad Mapping |
|-----------|-------------|-----------------|
| Column | Vertical arrangement | `View` (flow: Down) |
| Row | Horizontal arrangement | `View` (flow: Right) |
| List | Scrollable list | `PortalList` |
| Card | Card container | `View` + rounded border |

### Display Components

| Component | Description | Makepad Mapping |
|-----------|-------------|-----------------|
| Text | Text display | `Label` / DrawText |
| Image | Image display | `Image` / DrawQuad |
| Icon | Icon | Custom drawing |
| Divider | Separator line | DrawQuad |

### Interactive Components

| Component | Description | Makepad Mapping |
|-----------|-------------|-----------------|
| Button | Button | Custom DrawQuad + events |
| TextField | Text input | Custom input component |
| CheckBox | Checkbox | Custom drawing + events |
| Slider | Slider | Custom track + thumb |

### Container Components

| Component | Description | Makepad Mapping |
|-----------|-------------|-----------------|
| Modal | Modal dialog | Overlay View |
| Tabs | Tab panel | Custom switch component |

### Component Examples

#### Text Component

```json
{
  "id": "title",
  "component": {
    "Text": {
      "text": {"literalString": "Product List"},
      "usageHint": "h1"
    }
  }
}
```

`usageHint` controls styling:
- `h1` - 28px heading
- `h2` - 22px subheading
- `h3` - 18px small heading
- `body` - 14px body text
- `caption` - 12px caption

#### Button Component

```json
{
  "id": "submit-btn",
  "component": {
    "Button": {
      "child": "btn-text",
      "primary": true,
      "action": {
        "name": "submitForm",
        "context": [
          {"key": "formId", "value": {"literalString": "checkout"}}
        ]
      }
    }
  }
}
```

#### CheckBox Component

```json
{
  "id": "agree-checkbox",
  "component": {
    "CheckBox": {
      "value": {"path": "/form/agreed"},
      "label": {"literalString": "I agree to the Terms of Service"}
    }
  }
}
```

---

## Data Binding

A2UI uses JSON Pointer paths for reactive data binding.

### Value Types

Every bindable value supports two forms:

#### Literal Values

```json
{"literalString": "Fixed text"}
{"literalNumber": 42}
{"literalBoolean": true}
```

#### Path Binding

```json
{"path": "/user/name"}
{"path": "/cart/total"}
{"path": "/settings/darkMode"}
```

### How It Works

```
┌─────────────────────────────────────────────────────────────┐
│                      DataModel                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  /user/name: "John"                                  │    │
│  │  /cart/items: [{...}, {...}]                        │    │
│  │  /cart/total: 299.99                                │    │
│  └─────────────────────────────────────────────────────┘    │
│                           │                                  │
│                    On data change                            │
│                           ▼                                  │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  Text { text: {"path": "/user/name"} }              │    │
│  │       └──────► Automatically displays "John"         │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

### Two-Way Binding

Interactive components (TextField, CheckBox, Slider) support two-way binding:

1. **Display** - Read value from DataModel
2. **Update** - Write back to DataModel on user action

```rust
// When CheckBox is clicked
A2uiSurfaceAction::DataModelChanged {
    surface_id: "main",
    path: "/payment/alipay",  // Binding path
    value: true,              // New value
}
```

---

## Makepad Renderer Implementation

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Makepad Application                       │
├─────────────────────────────────────────────────────────────┤
│  A2uiHost ←→ A2aClient ←→ SSE Stream ←→ A2A Server         │
│         ↓                                                    │
│  A2uiMessageProcessor                                        │
│         ↓                                                    │
│  ┌─────────────┐    ┌─────────────┐                         │
│  │   Surface   │    │  DataModel  │                         │
│  │ (Component  │    │ (Data       │                         │
│  │    Tree)    │    │   Store)    │                         │
│  └─────────────┘    └─────────────┘                         │
│         ↓                  ↓                                 │
│  A2uiSurface Widget (Rendering + Event Handling)            │
└─────────────────────────────────────────────────────────────┘
```

### Core Modules

#### 1. A2uiHost - Connection Management

```rust
pub struct A2uiHost {
    config: A2uiHostConfig,
    client: Option<A2aClient>,
    stream: Option<A2aEventStream>,
}

// Usage
let config = A2uiHostConfig {
    url: "http://localhost:8080/rpc".to_string(),
    auth_token: None,
};
let mut host = A2uiHost::new(config);
host.connect("Show payment page")?;
```

#### 2. A2uiMessageProcessor - Message Processing

```rust
pub struct A2uiMessageProcessor {
    surfaces: HashMap<String, Surface>,
    data_models: HashMap<String, DataModel>,
    catalog: ComponentCatalog,
}

// Process messages
let events = processor.process_message(a2ui_message);
```

#### 3. A2uiSurface - Rendering Widget

```rust
#[derive(Live, LiveHook, Widget)]
pub struct A2uiSurface {
    // Draw primitives
    draw_bg: DrawQuad,
    draw_text: DrawText,
    draw_card: DrawColor,
    draw_button: DrawColor,
    draw_checkbox: DrawA2uiCheckBox,

    // State
    processor: Option<A2uiMessageProcessor>,
    button_areas: Vec<Area>,
    checkbox_areas: Vec<Area>,
    // ...
}
```

### Rendering Flow

```rust
fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
    // 1. Begin drawing background
    self.draw_bg.begin(cx, walk, self.layout);

    // 2. Get Surface and DataModel
    let surface = self.processor.get_surface("main");
    let data_model = self.processor.get_data_model("main");

    // 3. Recursively render from root component
    self.render_component(cx, scope, surface, data_model, &surface.root);

    // 4. End drawing
    self.draw_bg.end(cx);
    DrawStep::done()
}
```

### Event Handling

```rust
fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
    // Handle CheckBox clicks
    for (idx, area) in self.checkbox_areas.iter().enumerate() {
        match event.hits(cx, *area) {
            Hit::FingerDown(_) => {
                self.hovered_checkbox_idx = Some(idx);
            }
            Hit::FingerUp(fe) if fe.is_over => {
                // Toggle checked state
                let new_value = !current_value;
                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    A2uiSurfaceAction::DataModelChanged {
                        surface_id,
                        path: binding_path,
                        value: serde_json::Value::Bool(new_value),
                    },
                );
            }
            _ => {}
        }
    }
}
```

---

## Demo Application

### Overview

The `a2ui-demo` application demonstrates two modes:

| Mode | Feature | Data Source |
|------|---------|-------------|
| **Static Mode** | Product Catalog | Local JSON |
| **Streaming Mode** | Payment Checkout | Mock Server |

### Static Mode - Product Catalog

Features:
- Product list (with images)
- Filters (search, checkboxes, slider)
- Add to cart buttons

```
┌─────────────────────────────────────────┐
│  🛒 Product Catalog                      │
├─────────────────────────────────────────┤
│  ┌─────────────────────────────────┐    │
│  │ Filters                          │    │
│  │ Search: [____________]           │    │
│  │ ☑ In Stock Only  ☐ On Sale      │    │
│  │ Max Price: ────●──── $150       │    │
│  └─────────────────────────────────┘    │
│                                          │
│  ┌─────────────────────────────────┐    │
│  │ [IMG] Premium Headphones         │    │
│  │       $99.99      [Add to Cart]  │    │
│  └─────────────────────────────────┘    │
│  ┌─────────────────────────────────┐    │
│  │ [IMG] Wireless Mouse             │    │
│  │       $49.99      [Add to Cart]  │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
```

### Streaming Mode - Payment Checkout

Features:
- Order summary
- Payment method selection (single-select)
- Price breakdown
- Confirm/Cancel buttons

```
┌─────────────────────────────────────────┐
│  💳 Payment Checkout                     │
├─────────────────────────────────────────┤
│  ┌─────────────────────────────────┐    │
│  │ 📦 Order Items                   │    │
│  │ 🎧 Premium Headphones    $99.99  │    │
│  │ 🖱️ Wireless Mouse x2     $79.98  │    │
│  │ ⌨️ Mechanical Keyboard  $129.99  │    │
│  └─────────────────────────────────┘    │
│                                          │
│  ┌─────────────────────────────────┐    │
│  │ 💰 Payment Method                │    │
│  │ ◉ 💳 Credit Card                 │    │
│  │ ○ 🅿️ PayPal                      │    │
│  │ ○ [Alipay] Alipay                │    │
│  │ ○ [WeChat] WeChat Pay            │    │
│  └─────────────────────────────────┘    │
│                                          │
│  ┌─────────────────────────────────┐    │
│  │ 📊 Order Summary                 │    │
│  │ Subtotal:              $309.96   │    │
│  │ Shipping:                $9.99   │    │
│  │ Tax:                    $24.80   │    │
│  │ Total:                 $344.75   │    │
│  └─────────────────────────────────┘    │
│                                          │
│  [❌ Cancel]        [✅ Confirm & Pay]   │
└─────────────────────────────────────────┘
```

### Radio Button Behavior Implementation

Payment methods use CheckBox components but implement radio (single-select) behavior:

```rust
// streaming_demo.rs / app.rs
A2uiSurfaceAction::DataModelChanged { surface_id, path, value } => {
    let payment_methods = [
        "/payment/creditCard",
        "/payment/paypal",
        "/payment/alipay",
        "/payment/wechat",
    ];

    if payment_methods.contains(&path.as_str()) {
        if value == serde_json::Value::Bool(true) {
            // When one is selected, deselect all others
            for method in &payment_methods {
                if *method != path {
                    data_model.set(method, serde_json::Value::Bool(false));
                }
            }
        }
    }

    data_model.set(&path, value);
}
```

---

## Quick Start

### Running the Demo

#### 1. Start the Mock Server

```bash
cargo run -p a2ui-demo --bin mock-a2a-server --features mock-server
```

Output:
```
===========================================
  Mock A2A Server - Payment Page Demo
===========================================
Listening on http://127.0.0.1:8080/rpc
```

#### 2. Start the Demo Application

```bash
cargo run -p a2ui-demo
```

#### 3. Usage Instructions

- Click **"🛒 Product Catalog"** - Load static product list
- Click **"💳 Payment Checkout"** - Connect to server, stream payment page

### Project Structure

```
crates/
├── ui/src/a2ui/
│   ├── mod.rs              # Module exports
│   ├── message.rs          # A2UI message type definitions
│   ├── value.rs            # Value types (StringValue, NumberValue, BooleanValue)
│   ├── data_model.rs       # DataModel implementation
│   ├── processor.rs        # A2uiMessageProcessor
│   ├── surface.rs          # A2uiSurface Widget
│   ├── host.rs             # A2uiHost connection management
│   ├── a2a_client.rs       # A2A JSON-RPC client
│   └── sse.rs              # SSE streaming transport
│
└── a2ui-demo/src/
    ├── app.rs              # Main demo application
    ├── mock_server.rs      # Mock A2A server
    └── streaming_demo.rs   # Standalone streaming demo
```

### Custom Server Implementation

Implementing your own A2A server requires:

1. Receive JSON-RPC requests
2. Return SSE streaming responses
3. Send A2UI messages

```python
# Pseudo-code example
@app.post("/rpc")
async def handle_rpc(request):
    async def generate():
        # Send beginRendering
        yield f"data: {json.dumps(begin_rendering_msg)}\n\n"

        # Send surfaceUpdate
        yield f"data: {json.dumps(surface_update_msg)}\n\n"

        # Send dataModelUpdate
        yield f"data: {json.dumps(data_model_msg)}\n\n"

    return StreamingResponse(generate(), media_type="text/event-stream")
```

---

## Reference Resources

- **A2UI Specification**: `/fw/A2UI/specification/v0_9/`
- **Lit Renderer**: `/fw/A2UI/renderers/lit/`
- **Flutter Renderer (GenUI)**: `/fw/genui/`
- **Makepad Framework**: `/fw/makepad/`

---

*Document Version: 1.0 | Last Updated: 2026-02*
