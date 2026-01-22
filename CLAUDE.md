# Makepad Skills - Claude Instructions

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

- **Makepad**: `/path/to/makepad` - Framework source
- **Robrix**: `/path/to/robrix` - Matrix client example
- **Moly**: `/path/to/moly` - AI chat example
- **MolyKit**: `/path/to/moly/moly-kit` - AI chat toolkit
