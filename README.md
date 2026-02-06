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
  - [Kimi Bridge (LLM-powered UI)](#kimi-bridge-llm-powered-ui)
  - [Watch Server (Live File Editing)](#watch-server-live-file-editing)
  - [Math Charts Demo](#math-charts-demo)
- [LLM Configuration](#llm-configuration)
  - [Kimi Cloud (K2.5)](#kimi-cloud-k25)
  - [Local Qwen3 (via Ollama/vLLM)](#local-qwen3-via-ollamavllm)
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

### Kimi Bridge (LLM-powered UI)

Generate native UIs from natural language using Kimi K2.5's tool-calling API.

**Terminal 1: Start Kimi Bridge Server (port 8081)**
```bash
export MOONSHOT_API_KEY="sk-your-api-key"
cargo run --bin kimi-bridge --features kimi-bridge
```

**Terminal 2: Start the Makepad App**
```bash
cargo run -p a2ui-demo
# Click "Live Editor"
```

> **Note:** The app connects to `localhost:8082` by default. To use kimi-bridge directly, change the URL in `crates/a2ui-demo/src/app.rs` (line ~295) from port `8082` to `8081`.

**Terminal 3: Generate UIs via Chat**
```bash
# Login form
curl -X POST http://127.0.0.1:8081/chat \
  -H 'Content-Type: application/json' \
  -d '{"message": "Create a login form with username, password, and sign-in button"}'

# Music player
curl -X POST http://127.0.0.1:8081/chat \
  -d '{"message": "Create a music player with play/pause, next, prev buttons and volume slider"}'

# Stock watchlist
curl -X POST http://127.0.0.1:8081/chat \
  -d '{"message": "Create a stock watchlist with AAPL, GOOGL, TSLA prices and buy/sell buttons"}'

# Health tracker with charts
curl -X POST http://127.0.0.1:8081/chat \
  -d '{"message": "Create a health dashboard with steps bar chart, heart rate line chart, and sleep pie chart"}'
```

#### Kimi Bridge Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/chat` | POST | Send natural language, receive A2UI JSON |
| `/rpc` | POST | A2A protocol endpoint (returns latest UI as SSE) |
| `/live` | GET | Live updates via SSE |
| `/reset` | POST | Clear conversation history |
| `/status` | GET | Server health check |

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

### Kimi Cloud (K2.5)

The default configuration uses Kimi K2.5 via the Moonshot API:

```bash
# Get your API key from https://platform.moonshot.cn/
export MOONSHOT_API_KEY="sk-your-api-key"

cargo run --bin kimi-bridge --features kimi-bridge
```

The bridge calls `https://api.moonshot.ai/v1/chat/completions` with model `kimi-k2.5`.

### Local Qwen3 (via Ollama/vLLM)

To use a local model instead of Kimi Cloud, modify the API URL and model name in `crates/a2ui-demo/src/kimi_bridge.rs`:

**Step 1: Start a local OpenAI-compatible server**

```bash
# Option A: Ollama
ollama pull qwen3:32b
ollama serve  # Serves on http://localhost:11434

# Option B: vLLM
pip install vllm
vllm serve Qwen/Qwen3-32B --port 8000
```

**Step 2: Update the bridge configuration**

Edit `crates/a2ui-demo/src/kimi_bridge.rs`:

```rust
// Change from:
const KIMI_API_URL: &str = "https://api.moonshot.ai/v1/chat/completions";

// To (Ollama):
const KIMI_API_URL: &str = "http://localhost:11434/v1/chat/completions";

// Or (vLLM):
const KIMI_API_URL: &str = "http://localhost:8000/v1/chat/completions";
```

Also update the model name in the request body (search for `"model": "kimi-k2.5"` and change to your model name, e.g., `"qwen3:32b"`).

**Step 3: Run without the API key**

```bash
export MOONSHOT_API_KEY="not-needed"  # Still required as env var
cargo run --bin kimi-bridge --features kimi-bridge
```

> **Tip:** The LLM must support **tool/function calling** for the bridge to work. Qwen3-32B+ and Llama 3.1-70B+ support this well.

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
curl -X POST http://127.0.0.1:8081/chat \
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
│   │       ├── surface.rs   # A2uiSurface - renders component tree
│   │       ├── message.rs   # A2UI protocol types (serde JSON)
│   │       ├── host.rs      # SSE client for A2A servers
│   │       ├── processor.rs # Message → widget tree conversion
│   │       └── chart_bridge.rs  # ChartComponent → makepad-plot bridge
│   ├── component-zoo/       # Widget showcase demo app
│   ├── a2ui-demo/           # A2UI demo app + servers
│   │   └── src/
│   │       ├── main.rs          # Makepad GUI app
│   │       ├── kimi_bridge.rs   # Kimi LLM → A2UI bridge (port 8081)
│   │       ├── watch_server.rs  # File-watching SSE server (port 8080)
│   │       ├── mock_server.rs   # Mock A2A server (port 8080)
│   │       ├── math_charts.rs   # Math function chart generator
│   │       └── fft_demo.rs      # FFT visualization
│   └── makepad-plot/        # Chart/plot library (29 chart types + 3D)
│       └── src/
│           ├── lib.rs
│           ├── plot.rs      # All chart widgets (LinePlot, BarPlot, Surface3D, etc.)
│           ├── elements.rs  # Drawing primitives
│           └── text.rs      # Plot text rendering
├── ui_live.json             # Live-editable A2UI JSON
├── chart_test.json          # Chart examples
└── math_test.json           # Math charts output
```

### Server Ports

| Server | Port | Feature Flag | Purpose |
|--------|------|-------------|---------|
| Kimi Bridge | 8081 | `kimi-bridge` | LLM chat → A2UI JSON |
| Watch Server | 8080 | `mock-server` | File watcher → SSE stream |
| Mock A2A Server | 8080 | `mock-server` | Static A2A responses |

### Data Flow

```
                          ┌──────────────────┐
 curl /chat ──────────────▶ Kimi Bridge:8081 │──────▶ Kimi K2.5 API
                          │  (tool calls)    │◀──────  (tool_calls)
                          └───────┬──────────┘
                                  │ writes
                                  ▼
                          ┌──────────────────┐
                          │  ui_live.json    │
                          └───────┬──────────┘
                                  │ watched by
                                  ▼
                          ┌──────────────────┐
                          │ Watch Server:8080│──── SSE /rpc ──▶ Makepad App
                          └──────────────────┘                (native UI)
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
