# Kimi A2UI Bridge: LLM-Powered UI Generation

## Overview

The Kimi A2UI Bridge enables natural language UI generation by leveraging Kimi K2.5's tool use (function calling) capabilities to produce declarative A2UI JSON that renders in real-time on Makepad.

```
┌─────────────┐     ┌──────────────────┐     ┌─────────────┐     ┌─────────────┐
│   User      │────▶│  Kimi Bridge     │────▶│ Watch       │────▶│  Makepad    │
│   Prompt    │     │  Server          │     │ Server      │     │  App        │
└─────────────┘     └──────────────────┘     └─────────────┘     └─────────────┘
                           │                        │
                           ▼                        │
                    ┌──────────────┐                │
                    │  Kimi K2.5   │                │
                    │  (Tool Use)  │                │
                    └──────────────┘                │
                           │                        │
                           ▼                        │
                    ┌──────────────┐                │
                    │  ui_live.json│◀───────────────┘
                    └──────────────┘
```

## Architecture

### 1. A2UI Protocol

A2UI (Agent-to-UI) is a declarative JSON protocol designed for LLM-friendly UI generation. Key design principles:

- **Adjacency List Model**: Flat structure with ID references instead of deep nesting
- **Data Binding**: JSON Pointer paths for reactive state management
- **Separation of Concerns**: Components, layout, and data are defined separately

#### Protocol Messages

```json
[
  {"beginRendering": {"root": "component-id", "surfaceId": "main"}},
  {"surfaceUpdate": {"components": [...], "surfaceId": "main"}},
  {"dataModelUpdate": {"path": "/", "contents": [...], "surfaceId": "main"}}
]
```

### 2. Tool Definitions

The bridge defines 10 tools that map directly to A2UI components:

| Tool | Description | Key Parameters |
|------|-------------|----------------|
| `create_text` | Static or dynamic text | id, text/dataPath, style |
| `create_button` | Clickable button | id, label, action, primary |
| `create_textfield` | Text input | id, dataPath, placeholder |
| `create_slider` | Numeric slider | id, dataPath, min, max, step |
| `create_checkbox` | Boolean toggle | id, dataPath, label |
| `create_row` | Horizontal layout | id, children[] |
| `create_column` | Vertical layout | id, children[] |
| `create_card` | Container with styling | id, child, title |
| `set_data` | Initialize data value | path, stringValue/numberValue/boolValue |
| `render_ui` | Finalize and render | rootId, title |

#### Tool Schema Example

```json
{
  "type": "function",
  "function": {
    "name": "create_slider",
    "description": "Create a slider for numeric value selection",
    "parameters": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string",
          "description": "Unique identifier for this slider"
        },
        "dataPath": {
          "type": "string",
          "description": "JSON pointer path for value binding (e.g., '/volume')"
        },
        "min": {
          "type": "number",
          "description": "Minimum value"
        },
        "max": {
          "type": "number",
          "description": "Maximum value"
        },
        "step": {
          "type": "number",
          "description": "Step increment (optional, default 1)"
        }
      },
      "required": ["id", "dataPath", "min", "max"]
    }
  }
}
```

### 3. Kimi Bridge Server

The bridge server (`kimi_bridge.rs`) handles:

#### HTTP Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/chat` | POST | Send user message, get UI generation |
| `/rpc` | POST | A2A protocol for direct Makepad communication |
| `/live` | GET | SSE stream for real-time updates |
| `/reset` | POST | Clear conversation history |
| `/status` | GET | Server health check |

#### Request Flow

1. **User sends message** → POST /chat with `{"message": "Create a login form"}`

2. **Bridge calls Kimi API** with:
   - System prompt explaining UI generation
   - Tool definitions
   - User message
   - `temperature: 1` (required for Kimi K2.5)

3. **Kimi returns tool calls**:
```json
{
  "choices": [{
    "message": {
      "tool_calls": [
        {"function": {"name": "create_text", "arguments": "{\"id\":\"title\",\"text\":\"Login\"}"}},
        {"function": {"name": "create_button", "arguments": "{\"id\":\"btn\",\"label\":\"Submit\"}"}}
      ]
    }
  }]
}
```

4. **A2uiBuilder converts to A2UI JSON**:
```json
[
  {"beginRendering": {"root": "login-form", "surfaceId": "main"}},
  {"surfaceUpdate": {
    "components": [
      {"id": "title", "component": {"Text": {"text": {"literalString": "Login"}}}},
      {"id": "btn", "component": {"Button": {"child": "btn-text", "action": {"name": "submit"}}}}
    ],
    "surfaceId": "main"
  }},
  {"dataModelUpdate": {"path": "/", "contents": [], "surfaceId": "main"}}
]
```

5. **JSON written to `ui_live.json`** and broadcast via SSE

6. **Watch server detects change** → Makepad app polls and re-renders

### 4. A2uiBuilder Implementation

The builder maintains state and converts tool calls:

```rust
struct A2uiBuilder {
    components: Vec<Value>,        // All created components
    data_contents: Vec<Value>,     // Initial data values
    root_id: Option<String>,       // Root component for rendering
}
```

#### Conversion Logic

**Text Component**:
```rust
fn create_text(&mut self, id: &str, text: Option<&str>, data_path: Option<&str>, style: Option<&str>) {
    let text_value = if let Some(path) = data_path {
        json!({"path": path})
    } else {
        json!({"literalString": text.unwrap_or("")})
    };

    self.components.push(json!({
        "id": id,
        "component": {
            "Text": {
                "text": text_value,
                "usageHint": style.unwrap_or("body")
            }
        }
    }));
}
```

**Button Component** (creates implicit text child):
```rust
fn create_button(&mut self, id: &str, label: &str, action: &str, primary: bool) {
    let text_id = format!("{}-text", id);
    self.create_text(&text_id, Some(label), None, None);

    self.components.push(json!({
        "id": id,
        "component": {
            "Button": {
                "child": text_id,
                "action": {"name": action, "context": []},
                "primary": primary
            }
        }
    }));
}
```

**Layout Components** (Row/Column):
```rust
fn create_row(&mut self, id: &str, children: &[String]) {
    self.components.push(json!({
        "id": id,
        "component": {
            "Row": {
                "children": {"explicitList": children}
            }
        }
    }));
}
```

### 5. Data Binding

A2UI uses JSON Pointer paths for reactive data binding:

```
/volume          → Root-level "volume" key
/login/username  → Nested path: login.username
/items/0/name    → Array access: items[0].name
```

**Setting Initial Data**:
```json
{"set_data": {"path": "/volume", "numberValue": 50}}
```

Converts to:
```json
{"dataModelUpdate": {
  "path": "/",
  "contents": [{"key": "volume", "valueNumber": 50}],
  "surfaceId": "main"
}}
```

**Component Binding**:
```json
{"create_slider": {"id": "vol", "dataPath": "/volume", "min": 0, "max": 100}}
```

Slider value automatically syncs with `/volume` data path.

## Usage

### Starting the Servers

```bash
# Terminal 1: Start the Kimi Bridge
export MOONSHOT_API_KEY="your-api-key"
cargo run --bin kimi-bridge --features kimi-bridge

# Terminal 2: Start the Watch Server
cargo run --bin watch-server --features mock-server

# Terminal 3: Start the Makepad App
cargo run --bin a2ui-demo
```

### Generating UI

```bash
# Create a login form
curl -X POST http://127.0.0.1:8081/chat \
  -H 'Content-Type: application/json' \
  -d '{"message": "Create a login form with username and password"}'

# Create a music player
curl -X POST http://127.0.0.1:8081/chat \
  -H 'Content-Type: application/json' \
  -d '{"message": "Create a music player with play/pause, volume slider"}'

# Reset conversation
curl -X POST http://127.0.0.1:8081/reset
```

### In the Makepad App

1. Click "Live Editor" button to enter live mode
2. App automatically polls for changes every second
3. UI updates in real-time as Kimi generates new components

## Example: Generated Music Player

**User Prompt**: "Create a music player with play/pause buttons, volume slider, and track info"

**Kimi Tool Calls**:
```
1. create_text(id="track-title", text="🎵 Currently Playing", style="h1")
2. create_text(id="artist-name", text="Artist - Song Title", style="body")
3. create_button(id="prev-btn", label="⏮️ Prev", action="previous")
4. create_button(id="play-btn", label="▶️ Play", action="play", primary=true)
5. create_button(id="pause-btn", label="⏸️ Pause", action="pause")
6. create_button(id="next-btn", label="⏭️ Next", action="next")
7. create_text(id="volume-label", text="🔊 Volume", style="body")
8. create_slider(id="volume-slider", dataPath="/volume", min=0, max=100, step=1)
9. create_text(id="volume-value", dataPath="/volumeDisplay", style="caption")
10. create_row(id="controls-row", children=["prev-btn","play-btn","pause-btn","next-btn"])
11. create_row(id="volume-row", children=["volume-label","volume-slider","volume-value"])
12. create_column(id="music-player", children=["track-title","artist-name","controls-row","volume-row"])
13. set_data(path="/volume", numberValue=50)
14. set_data(path="/volumeDisplay", stringValue="50%")
15. render_ui(rootId="music-player", title="🎧 Music Player")
```

**Generated A2UI JSON** (see `ui_live.json`):
- 12 component definitions in flat adjacency list
- 2 data model entries for volume state
- Proper ID references for layout hierarchy

## Design Decisions

### Why Tool Use Instead of Raw JSON?

1. **Structured Output**: Tools enforce schema validation
2. **Incremental Building**: LLM can think step-by-step
3. **Error Handling**: Invalid tool calls are easier to catch
4. **Flexibility**: Easy to add new component types
5. **Natural Reasoning**: LLM explains its UI decisions

### Why Adjacency List Model?

1. **LLM-Friendly**: No deep nesting to track
2. **Order Independent**: Components can be defined in any order
3. **Reusable**: Same component can be referenced multiple times
4. **Flat Structure**: Easier for LLMs to generate correctly

### Why JSON Pointer for Data Binding?

1. **Standard Format**: RFC 6901 specification
2. **Flexible Paths**: Supports nested and array access
3. **Bidirectional**: Components both read and write data
4. **Reactive**: Changes propagate automatically

## Future Enhancements

1. **Streaming Generation**: Show components as they're created
2. **Component Library**: Pre-built templates for common UIs
3. **Style System**: Theme and styling tools
4. **Validation**: Type checking for data bindings
5. **Action Handlers**: Define what happens on button clicks
6. **Conditional Rendering**: Show/hide based on data values

## API Reference

### Kimi API Configuration

```json
{
  "model": "kimi-k2.5",
  "temperature": 1,
  "max_tokens": 32768,
  "top_p": 0.95,
  "tools": [...],
  "tool_choice": "auto"
}
```

**Note**: Kimi K2.5 requires `temperature: 1` for tool use.

### System Prompt

```
You are a UI generation assistant. Create user interfaces by calling the provided tools.

Guidelines:
1. Create components with unique IDs
2. Use descriptive IDs (e.g., "login-btn" not "btn1")
3. Build hierarchy with Row/Column containers
4. Initialize data with set_data before using dataPath
5. Always call render_ui at the end with the root container ID

Available components: Text, Button, TextField, Slider, Checkbox, Row, Column, Card
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| "temperature must be 1" | Set `temperature: 1` in API request |
| UI not updating | Check watch-server is running, verify ui_live.json path |
| Tool calls not working | Ensure tools array is properly formatted |
| Empty response | Check MOONSHOT_API_KEY environment variable |
| Components not rendering | Verify render_ui was called with correct rootId |
