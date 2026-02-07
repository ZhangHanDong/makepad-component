# Makepad Component

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/ZhangHanDong/makepad-component)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green.svg)](LICENSE)

**[中文](README_zh.md) | [日本語](README_ja.md)**

A modern UI component library for [Makepad](https://github.com/makepad/makepad), with a full **A2UI (Agent-to-UI)** protocol renderer that lets AI agents generate native, interactive UIs.

![Makepad Component Preview](asserts/mc1.png)

## Table of Contents

- [About Makepad](#about-makepad)
- [Components](#components)
- [A2UI Renderer](#a2ui-renderer)
- [Quick Start](#quick-start)
- [Running the Demos](#running-the-demos)
  - [Component Zoo](#component-zoo)
  - [A2UI Static Demo](#a2ui-static-demo)
  - [A2UI Bridge (LLM-powered UI)](#a2ui-bridge-llm-powered-ui)
  - [Watch Server (Live File Editing)](#watch-server-live-file-editing)
  - [Math Charts Demo](#math-charts-demo)
- [LLM Configuration](#llm-configuration)
- [A2UI App Types & Examples](#a2ui-app-types--examples)
- [Architecture](#architecture)
- [WebAssembly Build](#webassembly-build)
- [Claude Code Skills](#claude-code-skills)
- [Contributing](#contributing)
- [License](#license)

---

## About Makepad

[Makepad](https://github.com/makepad/makepad) is a next-generation UI framework written in Rust:

- **GPU-accelerated rendering** - Custom shader-based drawing with SDF (Signed Distance Field)
- **Cross-platform** - Desktop (Windows, macOS, Linux), Mobile (iOS, Android), Web (WebAssembly)
- **Live design** - Hot-reload DSL for rapid UI iteration
- **High performance** - Designed for demanding applications like IDEs and real-time tools

---

## Components

### Widget Library (v0.1.0)

| Component | Description |
|-----------|-------------|
| **Button** | Primary, Secondary, Danger, Ghost variants with sizes |
| **Checkbox** | With label and indeterminate state |
| **Switch** | Toggle switch with animations |
| **Radio** | Radio button groups |
| **Divider** | Horizontal/vertical separators |
| **Progress** | Linear progress bar |
| **Slider** | Single/Range mode, Vertical, Logarithmic, Disabled |
| **Badge** | Notification badges with variants |
| **Tooltip** | Four positions with edge detection and auto-flip |
| **Input** | Text input field |

### Screenshots

| Components | Slider Features |
|------------|-----------------|
| ![Components](asserts/mc1.png) | ![Slider](asserts/mc2.png) |

---

## A2UI Renderer

A complete **A2UI (Agent-to-UI)** protocol renderer enabling AI agents to generate interactive, native UIs:

- **Protocol Support** - Full A2UI v0.8 protocol (beginRendering, surfaceUpdate, dataModelUpdate)
- **Streaming** - Real-time SSE streaming for progressive UI updates
- **15 Component Types** - Text, Button, TextField, CheckBox, Slider, Image, Card, Row, Column, List, Tabs, Modal, Icon, Divider, MultipleChoice
- **29 Chart Types** - Bar, Line, Pie, Area, Scatter, Radar, Surface3D, and 22 more
- **Data Binding** - JSON Pointer path-based reactive data binding
- **Two-way Binding** - Interactive components sync back to data model
- **User Actions** - Action events with context resolution sent back to server

```
┌──────────────────────────────────────────────┐
│  AI Agent (LLM)                              │
│       │                                      │
│  A2UI JSON (declarative)                     │
│       ▼                                      │
│  ┌──────────────────────────────────┐        │
│  │  Makepad A2UI Renderer           │        │
│  │  - A2uiHost (SSE connection)     │        │
│  │  - A2uiMessageProcessor          │        │
│  │  - A2uiSurface (widget tree)     │        │
│  │  - makepad-plot (charts/3D)      │        │
│  └──────────────────────────────────┘        │
│       │                                      │
│  Native UI (GPU-accelerated)                 │
│       ▼                                      │
│  Desktop / Mobile / WebAssembly              │
└──────────────────────────────────────────────┘
```

---

## Quick Start

### Prerequisites

- **Rust** (stable toolchain): https://rustup.rs/
- **macOS/Linux/Windows** with OpenGL support

### Build & Run

```bash
# Clone the repository
git clone https://github.com/ZhangHanDong/makepad-component
cd makepad-component

# Run the component zoo (widget showcase)
cargo run -p component-zoo

# Run the A2UI demo
cargo run -p a2ui-demo
```

The A2UI demo starts with three mode buttons:
- **Product Catalog** - Static JSON product list with search, filters, cart
- **Math Charts** - Mathematical function visualizations (2D & 3D)
- **Live Editor** - Connect to a server for LLM-generated UIs

---

## Running the Demos

### Component Zoo

Standalone widget showcase with all available Makepad components:

```bash
cargo run -p component-zoo
```

### A2UI Static Demo

Product catalog with data-bound search, checkbox filters, slider, and add-to-cart buttons. No server needed:

```bash
cargo run -p a2ui-demo
# Click "Product Catalog"
```

### A2UI Bridge (LLM-powered UI)

Generate native UIs from natural language using **any OpenAI-compatible LLM** with tool-calling support. No code changes needed — configure everything via environment variables.

#### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `LLM_API_URL` | `https://api.moonshot.ai/v1/chat/completions` | Chat completions endpoint (full URL) |
| `LLM_MODEL` | `kimi-k2.5` | Model name |
| `LLM_API_KEY` | `not-needed` | API key (also reads `MOONSHOT_API_KEY`) |
| `LLM_PORT` | `8081` | Bridge server port |

#### Quick Start

**Terminal 1: Start the A2UI Bridge**
```bash
# Build once
cargo build --bin a2ui-bridge --features a2ui-bridge

# Run with any LLM provider (examples below)
LLM_API_URL="https://integrate.api.nvidia.com/v1/chat/completions" \
LLM_MODEL="minimaxai/minimax-m2.1" \
LLM_API_KEY="nvapi-your-key" \
LLM_PORT=8082 \
./target/debug/a2ui-bridge
```

**Terminal 2: Start the Makepad App**
```bash
cargo run -p a2ui-demo
# Click "Live Editor" — connects to localhost:8082 by default
```

**Terminal 3: Generate UIs via Chat**
```bash
# Login form
curl -X POST http://127.0.0.1:8082/chat \
  -H 'Content-Type: application/json' \
  -d '{"message": "Create a login form with username, password, and sign-in button"}'

# Music player
curl -X POST http://127.0.0.1:8082/chat \
  -d '{"message": "Create a music player with play/pause, next, prev buttons and volume slider"}'

# Health tracker with charts
curl -X POST http://127.0.0.1:8082/chat \
  -d '{"message": "Create a health dashboard with steps bar chart, heart rate line chart, and sleep pie chart"}'

# Reset conversation
curl -X POST http://127.0.0.1:8082/reset
```

#### Tested LLM Providers

| Provider | LLM_API_URL | LLM_MODEL | Notes |
|----------|-------------|-----------|-------|
| **NVIDIA NIM** | `https://integrate.api.nvidia.com/v1/chat/completions` | `minimaxai/minimax-m2.1` | Best quality, free tier available |
| **NVIDIA NIM** | `https://integrate.api.nvidia.com/v1/chat/completions` | `z-ai/glm4.7` | GLM 4.7 with reasoning |
| **Moonshot** | `https://api.moonshot.ai/v1/chat/completions` | `kimi-k2.5` | Default, requires `MOONSHOT_API_KEY` |

> **Tip:** The LLM must support **tool/function calling** for the bridge to work. Tested models: MiniMax M2.1, GLM 4.7, Kimi K2.5. Any OpenAI-compatible endpoint with tool calling works.

#### Bridge Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/chat` | POST | Send natural language `{"message": "..."}`, receive A2UI JSON |
| `/rpc` | POST | A2A protocol endpoint (returns latest UI as SSE stream) |
| `/live` | GET | Live updates via SSE (real-time push) |
| `/reset` | POST | Clear conversation history |
| `/status` | GET | Server health check (shows configured LLM URL and model) |
| `/inject` | POST | Inject raw A2UI JSON directly `{"a2ui": [...]}` |

#### Available Tool Functions

The bridge exposes 11 A2UI tools to the LLM:

| Tool | Description |
|------|-------------|
| `create_text` | Text labels with h1/h3/caption/body styles |
| `create_button` | Buttons with action bindings |
| `create_textfield` | Text input with data binding |
| `create_checkbox` | Boolean toggle |
| `create_slider` | Numeric range slider |
| `create_card` | Styled container |
| `create_column` | Vertical layout |
| `create_row` | Horizontal layout |
| `create_chart` | 13 chart types (bar, line, pie, area, scatter, radar, gauge, bubble, candlestick, heatmap, treemap, chord, sankey) |
| `set_data` | Set data model values |
| `render_ui` | Finalize and render (must call last) |

### Watch Server (Live File Editing)

Edit `ui_live.json` directly and see changes live in the app.

**Terminal 1: Start Watch Server (port 8080)**
```bash
cargo run --bin watch-server --features mock-server
```

**Terminal 2: Start the Makepad App**
```bash
cargo run -p a2ui-demo
# Click "Live Editor"
```

> **Note:** Change the app's server URL from `8082` to `8080` in `crates/a2ui-demo/src/app.rs` to match the watch-server port.

**Terminal 3: Edit ui_live.json**
```bash
# Edit the file — the watch server detects changes and streams to the app
vim ui_live.json
```

Or write A2UI JSON programmatically:
```bash
cat > ui_live.json << 'EOF'
[
  {"beginRendering": {"surfaceId": "main", "root": "root"}},
  {"surfaceUpdate": {
    "surfaceId": "main",
    "components": [
      {"id": "root", "component": {"Column": {"children": {"explicitList": ["title", "greeting"]}}}},
      {"id": "title", "component": {"Text": {"text": {"literalString": "Hello World"}, "usageHint": "h1"}}},
      {"id": "greeting", "component": {"Text": {"text": {"literalString": "Built with A2UI + Makepad"}}}}
    ]
  }}
]
EOF
```

### Math Charts Demo

Generates mathematical function visualizations (2D line charts and 3D surfaces):

```bash
# Generate math_test.json and ui_live.json
cargo run --bin math-charts

# Then view in the app
cargo run -p a2ui-demo
# Click "Math Charts" or "Live Editor"
```

Functions include Gaussian, Saddle, Mexican Hat, Damped Ripple, and more.

---

## LLM Configuration

The A2UI Bridge is fully configurable via environment variables — no code changes needed. See the [environment variables table](#environment-variables) above.

**Example: NVIDIA NIM (MiniMax M2.1)**
```bash
LLM_API_URL="https://integrate.api.nvidia.com/v1/chat/completions" \
LLM_MODEL="minimaxai/minimax-m2.1" \
LLM_API_KEY="nvapi-your-key" \
LLM_PORT=8082 \
./target/debug/a2ui-bridge
```

**Example: Moonshot Kimi K2.5 (default)**
```bash
LLM_API_KEY="sk-your-moonshot-key" \
./target/debug/a2ui-bridge
```

Any OpenAI-compatible chat completions endpoint with tool calling support will work.

---

## A2UI App Types & Examples

### 1. UI Apps (Forms, Lists, Dashboards)

Interactive forms with data binding, product catalogs, payment pages.

```json
[
  {"beginRendering": {"surfaceId": "main", "root": "root"}},
  {"surfaceUpdate": {
    "surfaceId": "main",
    "components": [
      {"id": "root", "component": {"Column": {"children": {"explicitList": ["title", "card"]}}}},
      {"id": "title", "component": {"Text": {"text": {"literalString": "My App"}, "usageHint": "h1"}}},
      {"id": "card", "component": {"Card": {"child": "card-content"}}},
      {"id": "card-content", "component": {"Column": {"children": {"explicitList": ["name-input", "submit-btn"]}}}},
      {"id": "name-input", "component": {"TextField": {"text": {"path": "/name"}, "placeholder": "Enter name"}}},
      {"id": "submit-btn-text", "component": {"Text": {"text": {"literalString": "Submit"}}}},
      {"id": "submit-btn", "component": {"Button": {"child": "submit-btn-text", "primary": true, "action": {"name": "submit"}}}}
    ]
  }},
  {"dataModelUpdate": {"surfaceId": "main", "path": "/", "contents": [
    {"key": "name", "valueString": ""}
  ]}}
]
```

### 2. Charts & Data Visualization

29 chart types covering business analytics, scientific data, and financial markets.

**2D Chart Types:**

| Type | Description | Data Format |
|------|-------------|-------------|
| Bar | Vertical/horizontal bars | `series[].values` = bar heights |
| Line | Connected data points | `series[].values` = y-values |
| Pie | Proportional segments | `series[0].values` = segment sizes |
| Area | Filled line chart | Same as Line |
| Scatter | X-Y point plot | `series[0]` = X, `series[1]` = Y |
| Radar | Multi-axis spider chart | `labels` = axes, `series[].values` = values per axis |
| Gauge | Single-value meter | `series[0].values[0]` = current value |
| Bubble | Sized scatter plot | `series[0]`=X, `[1]`=Y, `[2]`=Size |
| Candlestick | OHLC financial | 4 series: open, high, low, close |
| Heatmap | Color-coded matrix | `series` = rows, `labels` = columns |
| Treemap | Hierarchical rectangles | `series[0].values` = sizes |
| Chord | Relationship flows | `series[i].values[j]` = flow from i to j |
| Sankey | Flow diagram | `series[0]`=sources, `[1]`=targets, `[2]`=values |
| Histogram | Distribution bars | `series[0].values` = raw data |
| BoxPlot | Statistical summary | `series[0].values` = [min, Q1, median, Q3, max] |
| Donut | Pie with hole | Same as Pie |
| Stem | Stem-and-leaf | `series[0].values` = data points |
| Violin | Distribution shape | `series[0].values` = raw data |
| Polar | Polar coordinates | `series[0].values` = radii |
| Waterfall | Cumulative effect | `series[0].values` = changes |
| Funnel | Stage conversion | `series[0].values` = stage values |
| Step | Step function | `series[0].values` = y-values |

**3D Chart Types (Interactive with drag rotation):**

| Type | Description | Data Format |
|------|-------------|-------------|
| Surface3D | 3D surface mesh | `series` = rows of z-values (grid) |
| Scatter3D | 3D point cloud | `series[0]`=X, `[1]`=Y, `[2]`=Z |
| Line3D | 3D line path | `series[0]`=X, `[1]`=Y, `[2]`=Z |

**Example: Bar Chart**
```json
{"id": "sales-chart", "component": {
  "Chart": {
    "chartType": "bar",
    "title": "Monthly Sales",
    "width": 400.0, "height": 300.0,
    "labels": ["Jan", "Feb", "Mar", "Apr", "May"],
    "series": [
      {"name": "Revenue", "values": [120, 190, 150, 210, 180]},
      {"name": "Expenses", "values": [80, 100, 90, 120, 95]}
    ],
    "colors": ["#4CAF50", "#FF5722"]
  }
}}
```

**Example: 3D Surface**
```json
{"id": "surface", "component": {
  "Chart": {
    "chartType": "surface3d",
    "title": "Gaussian Surface",
    "width": 500.0, "height": 400.0,
    "series": [
      {"values": [0.1, 0.2, 0.5, 0.2, 0.1]},
      {"values": [0.2, 0.5, 0.8, 0.5, 0.2]},
      {"values": [0.5, 0.8, 1.0, 0.8, 0.5]},
      {"values": [0.2, 0.5, 0.8, 0.5, 0.2]},
      {"values": [0.1, 0.2, 0.5, 0.2, 0.1]}
    ]
  }
}}
```

### 3. Mathematical Visualizations

The `math-charts` binary generates famous mathematical functions:

```bash
cargo run --bin math-charts
```

Generates visualizations of:
- **2D Functions**: Sine/Cosine, Bessel, Damped oscillations
- **3D Surfaces**: Gaussian bell curve, Saddle point, Mexican hat (Ricker wavelet), Damped ripple

3D surfaces support interactive drag rotation and scroll zoom.

### 4. Financial Dashboards

Combine multiple chart types for financial analysis:

```bash
curl -X POST http://127.0.0.1:8082/chat \
  -d '{"message": "Create a stock dashboard with: candlestick chart for AAPL price history, pie chart for portfolio allocation, line chart for performance over time, and gauge for portfolio risk score"}'
```

### 5. Data-Bound Lists

Dynamic lists with template rendering from data model:

```json
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
}
```

Combined with a `dataModelUpdate` containing product data, this renders a scrollable list of product cards with images, names, prices, and action buttons.

---

## Architecture

### Workspace Crates

```
makepad-component/
├── crates/
│   ├── ui/                  # Core library: A2UI renderer + component widgets
│   │   └── src/a2ui/
│   │       ├── surface/     # A2uiSurface - renders component tree
│   │       ├── message.rs   # A2UI protocol types (serde JSON)
│   │       ├── host.rs      # SSE client for A2A servers
│   │       ├── processor.rs # Message → widget tree conversion
│   │       ├── chart_bridge/    # ChartComponent → makepad-plot bridge
│   │       ├── a2a_client.rs    # A2A protocol client (JSON-RPC + SSE)
│   │       └── sse.rs           # SSE streaming client
│   ├── component-zoo/       # Widget showcase demo app
│   ├── a2ui-demo/           # A2UI demo app + servers
│   │   └── src/
│   │       ├── main.rs              # Makepad GUI app
│   │       ├── a2ui_bridge.rs       # LLM → A2UI bridge server
│   │       ├── a2ui_bridge_impl/    # Bridge implementation modules
│   │       │   ├── server.rs        #   HTTP routing + LLM API client
│   │       │   ├── builder.rs       #   Tool calls → A2UI JSON builder
│   │       │   ├── tools.rs         #   Tool definitions for LLM
│   │       │   ├── types.rs         #   LLM response types
│   │       │   └── mureka.rs        #   Music generation (optional)
│   │       ├── watch_server.rs  # File-watching SSE server (port 8080)
│   │       └── math_charts.rs   # Math function chart generator
│   └── makepad-plot/        # Chart/plot library (29 chart types + 3D)
│       └── src/
│           ├── lib.rs
│           ├── plot/        # Chart widgets (LinePlot, BarPlot, Surface3D, etc.)
│           ├── elements.rs  # Drawing primitives
│           └── text.rs      # Plot text rendering
├── ui_live.json             # Live-editable A2UI JSON
├── chart_test.json          # Chart examples
└── math_test.json           # Math charts output
```

### Server Ports

| Server | Default Port | Feature Flag | Purpose |
|--------|-------------|-------------|---------|
| A2UI Bridge | 8082 (`LLM_PORT`) | `a2ui-bridge` | LLM chat → A2UI JSON |
| Watch Server | 8080 | `mock-server` | File watcher → SSE stream |
| Mock A2A Server | 8080 | `mock-server` | Static A2A responses |

> The Makepad app connects to `localhost:8082` by default. Set `LLM_PORT=8082` when starting the bridge to match.

### End-to-End Flow

```
┌──────────────────┐                    ┌───────────────────────────┐
│  Natural Language │   curl /chat      │  LLM with Tool Calling    │
│                  │ ─────────────────→ │                           │
│  "Create a       │                    │  MiniMax M2.1 (NVIDIA NIM) │
│   banking app"   │                    │    — or —                 │
│                  │                    │  GLM 4.7 (NVIDIA NIM)     │
└──────────────────┘                    │    — or —                 │
                                        │  Any OpenAI-compatible    │
                                        └─────────┬─────────────────┘
                                                  │ tool calls:
                                                  │ create_text, create_button,
                                                  │ create_card, create_chart...
                                                  ▼
                                        ┌───────────────────────────┐
                                        │  A2UI Bridge Server       │
                                        │  (tool call → JSON)       │
                                        │                           │
                                        │  Assembles flat adjacency │
                                        │  list of components       │
                                        └─────────┬─────────────────┘
                                                  │ serves via HTTP
                                                  │
                         ┌────────────────────────┼────────────────────────┐
                         │                        │                        │
                         ▼                        ▼                        ▼
                  POST /rpc               GET /live (SSE)          POST /chat
                  (full UI snapshot)      (real-time updates)      (new prompts)
                         │                        │
                         └────────┬───────────────┘
                                  │
                                  ▼
                    ┌───────────────────────────────┐
                    │  Makepad App                  │
                    │                               │
                    │  A2uiHost ──→ A2uiProcessor   │
                    │                    │          │
                    │              A2uiSurface      │
                    │              (widget tree)    │
                    │                    │          │
                    │         ┌──────────┼────────┐ │
                    │         ▼          ▼        ▼ │
                    │      Labels    Buttons   Charts│
                    │      Cards     Inputs    3D    │
                    │      Lists     Sliders   Plots │
                    │                               │
                    │  GPU-accelerated native render │
                    └───────────────────────────────┘
```

**Key points:**
- **A2UI is declarative JSON** — no code execution, safe across trust boundaries
- **Flat adjacency list** — LLM-friendly format, components reference each other by ID
- **Tool-use pattern** — LLM calls structured tools (`create_text`, `create_chart`, etc.), the bridge assembles valid A2UI JSON
- **Any OpenAI-compatible LLM works** — just needs tool/function calling support

### Data Flow

```
                          ┌──────────────────┐
 curl /chat ──────────────▶ A2UI Bridge      │──────▶ LLM API
                          │  (port 8082)     │◀──────  (tool_calls)
                          └───────┬──────────┘
                                  │ serves A2UI JSON
                                  │
                         ┌────────┼────────┐
                         ▼        ▼        ▼
                    POST /rpc  GET /live  POST /chat
                    (snapshot) (SSE push) (new prompts)
                         │        │
                         └───┬────┘
                             ▼
                      Makepad App (a2ui-demo)
                      (native GPU-accelerated UI)
```

### A2UI Protocol Messages

| Message | Direction | Purpose |
|---------|-----------|---------|
| `beginRendering` | Server → Client | Initialize surface with root component |
| `surfaceUpdate` | Server → Client | Add/update component tree (adjacency list) |
| `dataModelUpdate` | Server → Client | Update reactive data store |
| `deleteSurface` | Server → Client | Remove a UI surface |
| `userAction` | Client → Server | User interaction event with context |

---

## WebAssembly Build

```bash
# Install cargo-makepad (if not installed)
cargo install --force --git https://github.com/makepad/makepad.git --branch rik cargo-makepad

# Install wasm toolchain
cargo makepad wasm install-toolchain

# Build for web
cargo makepad wasm build -p component-zoo --release

# Serve locally
python3 serve_wasm.py 8080
# Open http://localhost:8080
```

---

## Claude Code Skills

This project includes Claude Code skills for Makepad development:

### makepad-screenshot

Automated screenshot debugging for Makepad GUI applications.

```
/screenshot              # Capture current running app
/screenshot a2ui-demo    # Capture specific app
```

See [skills/makepad-screenshot/SKILL.md](skills/makepad-screenshot/SKILL.md) for details.

---

## Installation (as library)

Add to your `Cargo.toml`:

```toml
[dependencies]
makepad-component = { git = "https://github.com/ZhangHanDong/makepad-component", branch = "main" }
```

```rust
use makepad_widgets::*;
use makepad_component::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use makepad_component::*;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                body = <View> {
                    flow: Down, spacing: 20, padding: 20
                    <MpButtonPrimary> { text: "Primary Button" }
                    <MpCheckbox> { text: "Check me" }
                    <MpSwitch> {}
                    <MpSlider> { value: 50.0, min: 0.0, max: 100.0 }
                }
            }
        }
    }
}
```

---

## AI-Assisted Development

This component library was built collaboratively with AI (Claude Code) using [makepad-skills](https://github.com/ZhangHanDong/makepad-skills).

---

## Contributing

> **Note:** This component library is still in early development and needs your help to grow!

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
