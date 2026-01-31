# Makepad Skills - Claude Instructions

---

## PROJECT GOAL: Makepad A2UI Renderer

### Objective

Build an **A2UI Renderer for Makepad** that enables AI agents to generate rich, interactive UIs rendered natively with Makepad widgets. This makes Makepad a first-class A2UI client alongside Lit, Angular, and Flutter (GenUI).

### What is A2UI?

**A2UI (Agent-to-UI)** is an open-source declarative JSON protocol for AI agents to generate interactive UIs:
- **Declarative JSON**: Safe to execute across trust boundaries (no code execution)
- **LLM-Optimized**: Flat adjacency list format easy for LLMs to generate
- **Cross-Platform**: One response renders on Web, Mobile, Desktop
- **Streaming**: Progressive UI updates as content generates

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    Makepad Application                           │
├─────────────────────────────────────────────────────────────────┤
│  A2uiHost ←→ ContentGenerator ←→ LLM/A2A Server                 │
│         ↓                                                        │
│  A2uiMessageProcessor (Rust)                                     │
│         ↓                                                        │
│  A2uiSurface → DataModel → Makepad Widgets                      │
└─────────────────────────────────────────────────────────────────┘
```

### A2UI Protocol Messages

| Message Type | Direction | Purpose |
|--------------|-----------|---------|
| `beginRendering` | Server→Client | Initialize a new UI surface |
| `surfaceUpdate` | Server→Client | Add or update component tree |
| `dataModelUpdate` | Server→Client | Update reactive data store |
| `deleteSurface` | Server→Client | Remove a UI surface |
| `userAction` | Client→Server | User interaction event |

---

### TARGET DEMO: 购物商城示例

最终目标是实现一个与 A2UI 示例项目（`/Users/zhangalex/Work/Projects/fw/A2UI/samples/agent/adk/`）相同的购物商城 Demo。

#### 示例 JSON 输入（产品列表）

```json
[
  {
    "beginRendering": {
      "surfaceId": "product-catalog",
      "root": "root-column",
      "styles": { "primaryColor": "#007BFF", "font": "Roboto" }
    }
  },
  {
    "surfaceUpdate": {
      "surfaceId": "product-catalog",
      "components": [
        {
          "id": "root-column",
          "component": {
            "Column": {
              "children": { "explicitList": ["header", "product-list"] }
            }
          }
        },
        {
          "id": "header",
          "component": {
            "Text": {
              "text": { "literalString": "Products" },
              "usageHint": "h1"
            }
          }
        },
        {
          "id": "product-list",
          "component": {
            "List": {
              "direction": "vertical",
              "children": {
                "template": {
                  "componentId": "product-card",
                  "dataBinding": "/products"
                }
              }
            }
          }
        },
        {
          "id": "product-card",
          "component": {
            "Card": { "child": "card-row" }
          }
        },
        {
          "id": "card-row",
          "component": {
            "Row": {
              "children": { "explicitList": ["product-image", "product-info", "add-btn"] },
              "alignment": "center"
            }
          }
        },
        {
          "id": "product-image",
          "component": {
            "Image": {
              "url": { "path": "imageUrl" },
              "fit": "cover"
            }
          }
        },
        {
          "id": "product-info",
          "component": {
            "Column": {
              "children": { "explicitList": ["product-name", "product-price"] }
            }
          }
        },
        {
          "id": "product-name",
          "component": {
            "Text": {
              "text": { "path": "name" },
              "usageHint": "h3"
            }
          }
        },
        {
          "id": "product-price",
          "component": {
            "Text": { "text": { "path": "price" } }
          }
        },
        {
          "id": "add-btn-text",
          "component": {
            "Text": { "text": { "literalString": "Add to Cart" } }
          }
        },
        {
          "id": "add-btn",
          "component": {
            "Button": {
              "child": "add-btn-text",
              "primary": true,
              "action": {
                "name": "addToCart",
                "context": [
                  { "key": "productId", "value": { "path": "id" } },
                  { "key": "quantity", "value": { "literalNumber": 1 } }
                ]
              }
            }
          }
        }
      ]
    }
  },
  {
    "dataModelUpdate": {
      "surfaceId": "product-catalog",
      "path": "/",
      "contents": [
        {
          "key": "products",
          "valueMap": [
            {
              "key": "p1",
              "valueMap": [
                { "key": "id", "valueString": "SKU001" },
                { "key": "name", "valueString": "Premium Headphones" },
                { "key": "price", "valueString": "$99.99" },
                { "key": "imageUrl", "valueString": "https://example.com/headphones.jpg" }
              ]
            },
            {
              "key": "p2",
              "valueMap": [
                { "key": "id", "valueString": "SKU002" },
                { "key": "name", "valueString": "Wireless Mouse" },
                { "key": "price", "valueString": "$49.99" },
                { "key": "imageUrl", "valueString": "https://example.com/mouse.jpg" }
              ]
            },
            {
              "key": "p3",
              "valueMap": [
                { "key": "id", "valueString": "SKU003" },
                { "key": "name", "valueString": "Mechanical Keyboard" },
                { "key": "price", "valueString": "$129.99" },
                { "key": "imageUrl", "valueString": "https://example.com/keyboard.jpg" }
              ]
            }
          ]
        }
      ]
    }
  }
]
```

#### 预期 Makepad 渲染效果

```
┌─────────────────────────────────────────────────────┐
│                     Products                         │  ← h1 标题
├─────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────┐ │
│ │ [Image]  │ Premium Headphones    │ [Add to Cart]│ │  ← Card + Row
│ │          │ $99.99                │              │ │
│ └─────────────────────────────────────────────────┘ │
│ ┌─────────────────────────────────────────────────┐ │
│ │ [Image]  │ Wireless Mouse        │ [Add to Cart]│ │
│ │          │ $49.99                │              │ │
│ └─────────────────────────────────────────────────┘ │
│ ┌─────────────────────────────────────────────────┐ │
│ │ [Image]  │ Mechanical Keyboard   │ [Add to Cart]│ │
│ │          │ $129.99               │              │ │
│ └─────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
```

#### 关键功能点

1. **动态列表渲染** - `List` 组件使用 `template` 绑定 `/products` 数据
2. **数据绑定** - `{ "path": "name" }` 自动从 DataModel 获取值
3. **组件嵌套** - Card → Row → [Image, Column, Button]
4. **用户交互** - Button 的 `action` 触发 `userAction` 消息返回服务器
5. **样式提示** - `usageHint: "h1"` / `"h3"` 控制文本样式

#### A2UI 示例项目参考

| 示例 | 路径 | 说明 |
|------|------|------|
| Rizzcharts | `/fw/A2UI/samples/agent/adk/rizzcharts/` | 电商仪表板（图表+地图） |
| Contact List | `/fw/A2UI/samples/agent/adk/contact_multiple_surfaces/` | 产品列表卡片模式 |
| Standard Catalog | `/fw/A2UI/specification/v0_8/json/standard_catalog_definition.json` | 组件 Schema 定义 |

---

### A2UI Adjacency List Model

Instead of deeply nested JSON, A2UI uses flat adjacency lists (LLM-friendly):

```json
{
  "updateComponents": {
    "surfaceId": "main",
    "components": [
      {"id": "root", "component": {"Column": {"children": ["title", "btn"]}}},
      {"id": "title", "component": {"Text": {"text": {"literalString": "Hello"}}}},
      {"id": "btn", "component": {"Button": {"child": "btn-text", "action": {"name": "click"}}}}
    ]
  }
}
```

### Component Mapping (A2UI → Makepad)

| A2UI Component | Makepad Widget | Notes |
|----------------|----------------|-------|
| Column | `View` (flow: Down) | Vertical layout |
| Row | `View` (flow: Right) | Horizontal layout |
| Text | `Label` | Display text with usageHint styles |
| Button | `Button` | With action binding |
| TextField | `TextInput` | Two-way data binding |
| Image | `Image` | With fit modes |
| List | `PortalList` | Scrollable with templates |
| Card | `View` + styling | Container with elevation |
| Checkbox | `CheckBox` | Boolean toggle |
| Slider | `Slider` | Range input |
| Modal | `View` + overlay | Dialog overlay |

### Data Binding

A2UI uses JSON Pointer paths for reactive data binding:

```json
// Static value
{"literalString": "Fixed text"}

// Data-bound value (reacts to DataModel changes)
{"path": "/user/name"}
```

**Makepad Implementation:**
```rust
// DataModel stores values by path
pub struct DataModel {
    data: HashMap<String, Value>,  // JSON Pointer path → Value
}

// Widgets subscribe to paths
impl A2uiText {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope) {
        // Get value from DataModel via path
        let text = self.resolve_string_value(cx, &self.text);
        self.label.set_text(&text);
        self.label.draw_walk(cx, scope, walk);
    }
}
```

### Implementation Phases

#### Phase 1: Core Infrastructure
- [ ] `A2uiMessage` enum (Rust types for protocol messages)
- [ ] `DataModel` with JSON Pointer path access
- [ ] `A2uiMessageProcessor` to update DataModel and component tree
- [ ] `ComponentRegistry` for A2UI → Makepad widget mapping

#### Phase 2: Standard Components
- [ ] Layout: Column, Row, List, Card
- [ ] Display: Text, Image, Icon, Divider
- [ ] Interactive: Button, TextField, Checkbox, Slider
- [ ] Container: Modal, Tabs

#### Phase 3: Data Binding
- [ ] `StringValue` resolver (literalString | path)
- [ ] `NumberValue` resolver
- [ ] `BooleanValue` resolver
- [ ] Two-way binding for input widgets
- [ ] Template-based children (dynamic lists)

#### Phase 4: Actions & Events
- [ ] `userAction` message generation
- [ ] Action context with data model values
- [ ] Button click, form submit, etc.

#### Phase 5: Integration
- [ ] SSE/WebSocket streaming support
- [ ] A2A server connection
- [ ] Example demo application

### Reference Implementations

| Platform | Package | Location |
|----------|---------|----------|
| Flutter | GenUI | `/Users/zhangalex/Work/Projects/fw/genui` |
| Web (Lit) | A2UI Lit | `/Users/zhangalex/Work/Projects/fw/A2UI/renderers/lit` |
| Web (Angular) | A2UI Angular | `/Users/zhangalex/Work/Projects/fw/A2UI/renderers/angular` |
| Specification | A2UI Spec | `/Users/zhangalex/Work/Projects/fw/A2UI/specification/v0_9` |

### Skills to Use

When implementing, load these skills:

| Task | Skills to Load |
|------|----------------|
| Makepad widgets | `makepad-widgets`, `makepad-layout`, `makepad-dsl` |
| Event handling | `makepad-event-action` |
| Async/HTTP | `robius-app-architecture` |
| A2UI protocol | `a2ui-protocol`, `a2ui-components`, `a2ui-data-binding` |
| GenUI reference | `genui-a2ui-integration`, `genui-data-binding` |
| Rust patterns | `m06-error-handling`, `m02-resource` |

### Makepad Source

- **Makepad Framework**: `/Users/zhangalex/Work/Projects/fw/makepad`
- **This Project**: `/Users/zhangalex/Work/Projects/fw/makepad-component`

---

## CRITICAL: Hook-Based Skill Loading

**IMPORTANT:** When you see a message starting with `[makepad-skills]` in the conversation, you MUST:

1. **Read the routing instruction** - e.g., `[makepad-skills] Routing to: makepad-widgets makepad-layout`
2. **Immediately call the Skill tool** for EACH skill listed before doing anything else
3. **Do not skip this step** - the skills contain essential Makepad knowledge

Example:
```
[makepad-skills] Routing to: makepad-widgets makepad-layout
```
→ Call `Skill(makepad-widgets)` then `Skill(makepad-layout)` FIRST, then answer the question.

---

## Skill Routing

For Makepad/Robius/MolyKit questions, use **context detection** and **skill dependencies** to load multiple related skills.

### Context Detection (Load Skill Bundles)

When user intent matches these contexts, load the entire skill bundle:

| Context | Trigger Keywords | Load These Skills |
|---------|------------------|-------------------|
| **Full App Development** | "build app", "create app", "从零", "完整应用", "app architecture" | makepad-basics, makepad-dsl, makepad-layout, makepad-widgets, makepad-event-action, robius-app-architecture |
| **UI Design** | "ui design", "界面设计", "design ui" | makepad-dsl, makepad-layout, makepad-widgets, makepad-animation, makepad-shaders |
| **Widget/Component Creation** | "create widget", "创建组件", "自定义组件", "custom component" | makepad-widgets, makepad-dsl, makepad-layout, makepad-animation, makepad-shaders, makepad-font, makepad-event-action |
| **Production Patterns** | "best practice", "robrix pattern", "实际项目", "production" | robius-app-architecture, robius-widget-patterns, robius-state-management, robius-event-action |

### Skill Dependencies (Auto-Load Related Skills)

When loading a skill, automatically include its dependencies:

| Primary Skill | Also Load |
|---------------|-----------|
| makepad-widgets | makepad-layout, makepad-dsl |
| makepad-animation | makepad-shaders |
| makepad-shaders | makepad-widgets |
| makepad-font | makepad-widgets |
| robius-app-architecture | makepad-basics, makepad-event-action |
| robius-widget-patterns | makepad-widgets, makepad-layout |
| robius-event-action | makepad-event-action |

### Single Skill Keywords (Fallback)

For specific questions, match keywords to individual skills:

| Keywords | Skill |
|----------|-------|
| getting started, `live_design!`, `app_main!` | makepad-basics |
| DSL syntax, inheritance, `<Widget>`, `Foo = { }` | makepad-dsl |
| layout, Flow, Walk, padding, center, align | makepad-layout |
| View, Button, Label, widget | makepad-widgets |
| event, action, Hit, FingerDown, handle_event | makepad-event-action |
| animator, state, transition, hover | makepad-animation |
| shader, draw_bg, Sdf2d, gradient, glow | makepad-shaders |
| platform, macOS, Android, iOS, WASM | makepad-platform |
| font, text, glyph, typography | makepad-font |
| splash, script, cx.eval | makepad-splash |
| Tokio, async, submit_async_request | robius-app-architecture |
| apply_over, modal, collapsible, pageflip | robius-widget-patterns |
| custom action, MatchEvent, post_action | robius-event-action |
| AppState, persistence, Scope::with_data | robius-state-management |
| Matrix SDK, sliding sync, MatrixRequest | robius-matrix-integration |
| BotClient, OpenAI, SSE streaming | molykit |
| deploy, package, APK, IPA | makepad-deployment |
| troubleshoot, error, debug | makepad-reference |

### Extended Skills

**Note:** Production patterns are integrated into robius-* skills:
- Widget patterns (modal, collapsible, drag-drop) → `robius-widget-patterns/_base/`
- State patterns (theme switching, state machine) → `robius-state-management/_base/`
- Async patterns (streaming, tokio) → `robius-app-architecture/_base/`

## Usage Examples

### Full App Development (Bundle)
```
User: "我想从零开发一个 Makepad 应用"
-> Detect: Full app context
-> Load: makepad-basics, makepad-dsl, makepad-layout, makepad-widgets,
         makepad-event-action, robius-app-architecture
-> Answer with complete app structure, widgets, events, and async patterns
```

### Widget Creation (Bundle)
```
User: "帮我创建一个自定义按钮组件"
-> Detect: Widget creation context
-> Load: makepad-widgets, makepad-dsl, makepad-layout, makepad-animation,
         makepad-shaders, makepad-font, makepad-event-action
-> Answer with widget structure, styling, animations, and event handling
```

### Simple Question (Single + Dependencies)
```
User: "如何设置字体大小"
-> Match: makepad-font
-> Auto-load dependency: makepad-widgets
-> Load: makepad-font, makepad-widgets
-> Answer with text_style, font_size, and widget context
```

### Production App (Bundle)
```
User: "参考 Robrix 的最佳实践"
-> Detect: Production context
-> Load: robius-app-architecture, robius-widget-patterns,
         robius-state-management, robius-event-action
         + dependencies: makepad-basics, makepad-widgets, makepad-layout, makepad-event-action
-> Answer with production-ready patterns from Robrix/Moly codebases
```

## Key Patterns

### Makepad Widget Definition
```rust
#[derive(Live, LiveHook, Widget)]
pub struct MyWidget {
    #[deref] view: View,
    #[live] property: f64,
    #[rust] internal_state: State,
    #[animator] animator: Animator,
}
```

### Robius Async Pattern
```rust
// UI -> Async
submit_async_request(MatrixRequest::SendMessage { ... });

// Async -> UI
Cx::post_action(MessageSentAction { ... });
SignalToUI::set_ui_signal();
```

### MolyKit Cross-Platform Async
```rust
// Platform-agnostic spawning
spawn(async move {
    let result = fetch_data().await;
    Cx::post_action(DataReady(result));
    SignalToUI::set_ui_signal();
});
```

## Default Project Settings

When creating Makepad projects:

```toml
[package]
edition = "2024"

[dependencies]
makepad-widgets = { git = "https://github.com/makepad/makepad", branch = "dev" }

[features]
default = []
nightly = ["makepad-widgets/nightly"]
```

## Source Codebases

For deeper reference, check these codebases:

- **Makepad**: `/Users/zhangalex/Work/Projects/fw/makepad` - Framework source
- **A2UI**: `/Users/zhangalex/Work/Projects/fw/A2UI` - A2UI protocol and renderers
- **GenUI**: `/Users/zhangalex/Work/Projects/fw/genui` - Flutter A2UI renderer
- **Robrix**: `/Users/zhangalex/Work/Projects/fw/robrix` - Matrix client example
- **Moly**: `/Users/zhangalex/Work/Projects/fw/moly` - AI chat example
