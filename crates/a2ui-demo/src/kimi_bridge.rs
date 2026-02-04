//! Kimi A2UI Bridge Server
//!
//! Connects Kimi K2.5 LLM with A2UI Makepad renderer via tool use.
//!
//! Architecture:
//! 1. User sends chat message via POST /chat
//! 2. Server calls Kimi K2.5 with A2UI component tools
//! 3. Kimi returns tool_calls to build UI
//! 4. Server converts tool_calls to A2UI JSON
//! 5. Streams A2UI JSON to connected Makepad clients via SSE

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::RwLock;

const KIMI_API_URL: &str = "https://api.moonshot.ai/v1/chat/completions";

// ============================================================================
// A2UI Component Tools Definition
// ============================================================================

fn get_a2ui_tools() -> Value {
    json!([
        {
            "type": "function",
            "function": {
                "name": "create_text",
                "description": "Create a text/label component to display static or dynamic text",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID (e.g., 'title', 'label-1')"},
                        "text": {"type": "string", "description": "Static text to display"},
                        "dataPath": {"type": "string", "description": "JSON pointer for dynamic text binding (e.g., '/user/name')"},
                        "style": {"type": "string", "enum": ["h1", "h3", "caption", "body"], "description": "Text style: h1=large title, h3=subtitle, caption=small, body=normal"}
                    },
                    "required": ["id"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_button",
                "description": "Create a clickable button that triggers an action",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "label": {"type": "string", "description": "Button text label"},
                        "action": {"type": "string", "description": "Action name triggered on click (e.g., 'submit', 'cancel')"},
                        "primary": {"type": "boolean", "description": "If true, button is highlighted as primary action"}
                    },
                    "required": ["id", "label", "action"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_textfield",
                "description": "Create a text input field for user input",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "dataPath": {"type": "string", "description": "JSON pointer for data binding (e.g., '/form/email')"},
                        "placeholder": {"type": "string", "description": "Placeholder text shown when empty"}
                    },
                    "required": ["id", "dataPath"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_checkbox",
                "description": "Create a checkbox toggle for boolean values",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "label": {"type": "string", "description": "Label text next to checkbox"},
                        "dataPath": {"type": "string", "description": "JSON pointer for boolean binding (e.g., '/settings/darkMode')"}
                    },
                    "required": ["id", "label", "dataPath"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_slider",
                "description": "Create a slider for numeric value selection",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "dataPath": {"type": "string", "description": "JSON pointer for numeric binding (e.g., '/volume')"},
                        "min": {"type": "number", "description": "Minimum value"},
                        "max": {"type": "number", "description": "Maximum value"},
                        "step": {"type": "number", "description": "Step increment (default: 1)"}
                    },
                    "required": ["id", "dataPath", "min", "max"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_card",
                "description": "Create a card container with visual styling (elevation, border)",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "childId": {"type": "string", "description": "ID of the child component inside the card"}
                    },
                    "required": ["id", "childId"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_column",
                "description": "Create a vertical layout container (stacks children top to bottom)",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "children": {"type": "array", "items": {"type": "string"}, "description": "Array of child component IDs in order"}
                    },
                    "required": ["id", "children"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_row",
                "description": "Create a horizontal layout container (arranges children left to right)",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "children": {"type": "array", "items": {"type": "string"}, "description": "Array of child component IDs in order"}
                    },
                    "required": ["id", "children"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "set_data",
                "description": "Set initial data value in the data model",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {"type": "string", "description": "JSON pointer path (e.g., '/volume', '/user/name')"},
                        "stringValue": {"type": "string", "description": "String value to set"},
                        "numberValue": {"type": "number", "description": "Number value to set"},
                        "booleanValue": {"type": "boolean", "description": "Boolean value to set"}
                    },
                    "required": ["path"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_chart",
                "description": "Create a chart. Types: bar, line, pie, area (filled line), scatter (series[0]=X,series[1]=Y), radar (labels=axes, values per axis), gauge (series[0].values[0]=value, maxValue=max), bubble (series[0]=X,[1]=Y,[2]=Size), candlestick (4 series: open,high,low,close), heatmap (series=rows, labels=columns), treemap (series[0]=sizes), chord (labels=entities, series=flow matrix rows: series[i].values[j]=flow from i to j), sankey (labels=node names, series[0]=source indices, series[1]=target indices, series[2]=flow values).",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "chartType": {"type": "string", "enum": ["bar", "line", "pie", "area", "scatter", "radar", "gauge", "bubble", "candlestick", "heatmap", "treemap", "chord", "sankey"], "description": "Chart type"},
                        "title": {"type": "string", "description": "Chart title displayed above the chart"},
                        "labels": {"type": "array", "items": {"type": "string"}, "description": "Category labels / axis names / column headers"},
                        "values": {"type": "array", "items": {"type": "number"}, "description": "Data values for a single series"},
                        "series": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "name": {"type": "string", "description": "Series name (for legend)"},
                                    "values": {"type": "array", "items": {"type": "number"}, "description": "Data values"}
                                },
                                "required": ["values"]
                            },
                            "description": "Multiple data series (alternative to 'values' for multi-series charts)"
                        },
                        "colors": {"type": "array", "items": {"type": "string"}, "description": "Color palette as hex strings"},
                        "width": {"type": "number", "description": "Chart width in pixels (default: 400)"},
                        "height": {"type": "number", "description": "Chart height in pixels (default: 300)"},
                        "maxValue": {"type": "number", "description": "Max value for gauge chart (default: 100)"}
                    },
                    "required": ["id", "chartType", "labels"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "render_ui",
                "description": "Finalize and render the UI with the specified root component. Call this LAST after creating all components.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "rootId": {"type": "string", "description": "ID of the root component (usually a column or row)"},
                        "title": {"type": "string", "description": "Optional title for the UI surface"}
                    },
                    "required": ["rootId"]
                }
            }
        }
    ])
}

// ============================================================================
// Kimi API Types
// ============================================================================

#[derive(Debug, Deserialize)]
struct KimiResponse {
    choices: Vec<KimiChoice>,
}

#[derive(Debug, Deserialize)]
struct KimiChoice {
    message: KimiMessage,
}

#[derive(Debug, Deserialize)]
struct KimiMessage {
    content: Option<String>,
    reasoning_content: Option<String>,
    tool_calls: Option<Vec<KimiToolCall>>,
}

#[derive(Debug, Deserialize)]
struct KimiToolCall {
    id: String,
    function: KimiFunctionCall,
}

#[derive(Debug, Deserialize)]
struct KimiFunctionCall {
    name: String,
    arguments: String,
}

// ============================================================================
// A2UI Builder - Converts tool calls to A2UI JSON
// ============================================================================

struct A2uiBuilder {
    components: Vec<Value>,
    data_contents: Vec<Value>,
    root_id: Option<String>,
}

impl A2uiBuilder {
    fn new() -> Self {
        A2uiBuilder {
            components: Vec::new(),
            data_contents: Vec::new(),
            root_id: None,
        }
    }

    fn process_tool_call(&mut self, name: &str, args: &Value) {
        match name {
            "create_text" => self.create_text(args),
            "create_button" => self.create_button(args),
            "create_textfield" => self.create_textfield(args),
            "create_checkbox" => self.create_checkbox(args),
            "create_slider" => self.create_slider(args),
            "create_card" => self.create_card(args),
            "create_column" => self.create_column(args),
            "create_row" => self.create_row(args),
            "create_chart" => self.create_chart(args),
            "set_data" => self.set_data(args),
            "render_ui" => self.render_ui(args),
            _ => eprintln!("Unknown tool: {}", name),
        }
    }

    fn create_text(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("text");

        let text_value = if let Some(data_path) = args["dataPath"].as_str() {
            json!({"path": data_path})
        } else if let Some(text) = args["text"].as_str() {
            json!({"literalString": text})
        } else {
            json!({"literalString": ""})
        };

        let mut component = json!({
            "Text": {
                "text": text_value
            }
        });

        if let Some(style) = args["style"].as_str() {
            component["Text"]["usageHint"] = json!(style);
        }

        self.components.push(json!({
            "id": id,
            "component": component
        }));
    }

    fn create_button(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("button");
        let label = args["label"].as_str().unwrap_or("Button");
        let action = args["action"].as_str().unwrap_or("click");
        let primary = args["primary"].as_bool().unwrap_or(false);

        // Create button text component
        let text_id = format!("{}-text", id);
        self.components.push(json!({
            "id": text_id,
            "component": {
                "Text": {
                    "text": {"literalString": label}
                }
            }
        }));

        // Create button
        self.components.push(json!({
            "id": id,
            "component": {
                "Button": {
                    "child": text_id,
                    "primary": primary,
                    "action": {
                        "name": action,
                        "context": []
                    }
                }
            }
        }));
    }

    fn create_textfield(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("textfield");
        let data_path = args["dataPath"].as_str().unwrap_or("/input");
        let placeholder = args["placeholder"].as_str().unwrap_or("");

        self.components.push(json!({
            "id": id,
            "component": {
                "TextField": {
                    "text": {"path": data_path},
                    "placeholder": {"literalString": placeholder}
                }
            }
        }));
    }

    fn create_checkbox(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("checkbox");
        let label = args["label"].as_str().unwrap_or("Option");
        let data_path = args["dataPath"].as_str().unwrap_or("/checked");

        self.components.push(json!({
            "id": id,
            "component": {
                "CheckBox": {
                    "label": {"literalString": label},
                    "value": {"path": data_path}
                }
            }
        }));
    }

    fn create_slider(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("slider");
        let data_path = args["dataPath"].as_str().unwrap_or("/value");
        let min = args["min"].as_f64().unwrap_or(0.0);
        let max = args["max"].as_f64().unwrap_or(100.0);
        let step = args["step"].as_f64().unwrap_or(1.0);

        self.components.push(json!({
            "id": id,
            "component": {
                "Slider": {
                    "value": {"path": data_path},
                    "min": min,
                    "max": max,
                    "step": step
                }
            }
        }));
    }

    fn create_card(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("card");
        let child_id = args["childId"].as_str().unwrap_or("card-content");

        self.components.push(json!({
            "id": id,
            "component": {
                "Card": {
                    "child": child_id
                }
            }
        }));
    }

    fn create_column(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("column");
        let children: Vec<String> = args["children"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        self.components.push(json!({
            "id": id,
            "component": {
                "Column": {
                    "children": {"explicitList": children}
                }
            }
        }));
    }

    fn create_row(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("row");
        let children: Vec<String> = args["children"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        self.components.push(json!({
            "id": id,
            "component": {
                "Row": {
                    "children": {"explicitList": children}
                }
            }
        }));
    }

    fn create_chart(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("chart");
        let chart_type = args["chartType"].as_str().unwrap_or("bar");
        let title = args["title"].as_str();
        let width = args["width"].as_f64().unwrap_or(400.0);
        let height = args["height"].as_f64().unwrap_or(300.0);

        // Parse labels
        let labels: Vec<String> = args["labels"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        // Parse series - either from "series" array or single "values" array
        let series: Vec<Value> = if let Some(series_arr) = args["series"].as_array() {
            series_arr.iter().map(|s| {
                let name = s["name"].as_str().map(|n| json!(n));
                let values: Vec<f64> = s["values"]
                    .as_array()
                    .map(|arr| arr.iter().filter_map(|v| v.as_f64()).collect())
                    .unwrap_or_default();
                let mut obj = json!({"values": values});
                if let Some(n) = name {
                    obj["name"] = n;
                }
                obj
            }).collect()
        } else if let Some(values_arr) = args["values"].as_array() {
            let values: Vec<f64> = values_arr.iter().filter_map(|v| v.as_f64()).collect();
            vec![json!({"values": values})]
        } else {
            vec![json!({"values": []})]
        };

        // Parse colors
        let colors: Vec<String> = args["colors"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        let mut component = json!({
            "Chart": {
                "chartType": chart_type,
                "labels": labels,
                "series": series,
                "width": width,
                "height": height
            }
        });

        if let Some(t) = title {
            component["Chart"]["title"] = json!({"literalString": t});
        }

        if !colors.is_empty() {
            component["Chart"]["colors"] = json!(colors);
        }

        if let Some(max_val) = args["maxValue"].as_f64() {
            component["Chart"]["maxValue"] = json!(max_val);
        }

        self.components.push(json!({
            "id": id,
            "component": component
        }));
    }

    fn set_data(&mut self, args: &Value) {
        let path = args["path"].as_str().unwrap_or("/");

        // Parse the path to build nested structure
        let parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        if parts.is_empty() || parts[0].is_empty() {
            return;
        }

        let value = if let Some(s) = args["stringValue"].as_str() {
            json!({"valueString": s})
        } else if let Some(n) = args["numberValue"].as_f64() {
            json!({"valueNumber": n})
        } else if let Some(b) = args["booleanValue"].as_bool() {
            json!({"valueBoolean": b})
        } else if let Some(n) = args["value"].as_f64() {
            // Fallback for simple "value" field
            json!({"valueNumber": n})
        } else if let Some(s) = args["value"].as_str() {
            json!({"valueString": s})
        } else if let Some(b) = args["value"].as_bool() {
            json!({"valueBoolean": b})
        } else {
            json!({"valueString": ""})
        };

        // For now, store as flat key-value (simplified)
        let key = parts.last().unwrap_or(&"");
        let mut content = json!({"key": key});

        // Merge value fields
        if let Some(obj) = value.as_object() {
            for (k, v) in obj {
                content[k] = v.clone();
            }
        }

        self.data_contents.push(content);
    }

    fn render_ui(&mut self, args: &Value) {
        if let Some(root_id) = args["rootId"].as_str() {
            self.root_id = Some(root_id.to_string());
        }
    }

    fn build_a2ui_json(&self) -> Value {
        let root = self.root_id.as_deref().unwrap_or("root");
        let mut components = self.components.clone();

        // Check if root component exists; if not, auto-create it as a Column
        // containing all top-level components (those not referenced as children)
        let root_exists = components.iter().any(|c| {
            c["id"].as_str() == Some(root)
        });

        if !root_exists {
            // Collect all IDs that are referenced as children by other components
            let mut child_ids = std::collections::HashSet::new();
            for comp in &components {
                let c = &comp["component"];
                // Column children
                if let Some(kids) = c["Column"]["children"]["explicitList"].as_array() {
                    for kid in kids {
                        if let Some(id) = kid.as_str() { child_ids.insert(id.to_string()); }
                    }
                }
                // Row children
                if let Some(kids) = c["Row"]["children"]["explicitList"].as_array() {
                    for kid in kids {
                        if let Some(id) = kid.as_str() { child_ids.insert(id.to_string()); }
                    }
                }
                // Card child
                if let Some(id) = c["Card"]["child"].as_str() { child_ids.insert(id.to_string()); }
                // Button child
                if let Some(id) = c["Button"]["child"].as_str() { child_ids.insert(id.to_string()); }
            }

            // Top-level = components whose IDs are not referenced as children
            let top_level: Vec<String> = components.iter()
                .filter_map(|c| {
                    let id = c["id"].as_str()?;
                    if !child_ids.contains(id) { Some(id.to_string()) } else { None }
                })
                .collect();

            eprintln!("[A2uiBuilder] Root '{}' not found, auto-creating Column with {} top-level children", root, top_level.len());

            components.push(json!({
                "id": root,
                "component": {
                    "Column": {
                        "children": { "explicitList": top_level }
                    }
                }
            }));
        }

        json!([
            {
                "beginRendering": {
                    "surfaceId": "main",
                    "root": root
                }
            },
            {
                "surfaceUpdate": {
                    "surfaceId": "main",
                    "components": components
                }
            },
            {
                "dataModelUpdate": {
                    "surfaceId": "main",
                    "path": "/",
                    "contents": self.data_contents
                }
            }
        ])
    }
}

// ============================================================================
// Server State
// ============================================================================

struct ServerState {
    api_key: String,
    tx: broadcast::Sender<String>,
    conversation: RwLock<Vec<Value>>,
    latest_a2ui: RwLock<Option<Value>>,
}

// ============================================================================
// Kimi API Client
// ============================================================================

async fn call_kimi(api_key: &str, messages: Vec<Value>) -> Result<KimiResponse, String> {
    let client = reqwest::Client::new();

    let request_body = json!({
        "model": "kimi-k2.5",
        "messages": messages,
        "tools": get_a2ui_tools(),
        "temperature": 1,
        "max_tokens": 8192,
        "stream": false
    });

    let response = client
        .post(KIMI_API_URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = response.status();
    let body = response.text().await.map_err(|e| format!("Failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!("API error ({}): {}", status, body));
    }

    serde_json::from_str(&body).map_err(|e| format!("Failed to parse response: {} - Body: {}", e, body))
}

// ============================================================================
// HTTP Handlers
// ============================================================================

async fn handle_request(
    req: Request<Incoming>,
    state: Arc<ServerState>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    println!("[Kimi Bridge] {} {}", method, path);

    match (method, path.as_str()) {
        // Chat endpoint - send message to Kimi
        (Method::POST, "/chat") => {
            let body_bytes = http_body_util::BodyExt::collect(req.into_body())
                .await
                .map(|b| b.to_bytes())
                .unwrap_or_default();

            let body_str = String::from_utf8_lossy(&body_bytes);

            #[derive(Deserialize)]
            struct ChatRequest {
                message: String,
            }

            let chat_req: ChatRequest = match serde_json::from_str(&body_str) {
                Ok(r) => r,
                Err(e) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .header("Content-Type", "application/json")
                        .body(Full::new(Bytes::from(json!({"error": format!("Invalid JSON: {}", e)}).to_string())))
                        .unwrap());
                }
            };

            println!("[Kimi Bridge] User message: {}", chat_req.message);

            // Build messages with system prompt
            let system_prompt = r#"You are an A2UI generator assistant. Your job is to create user interfaces by calling the provided tools.

IMPORTANT RULES:
1. Create components using the tools (create_text, create_button, create_slider, etc.)
2. Use create_column for vertical layouts, create_row for horizontal layouts
3. Use create_card to wrap sections in styled containers
4. Set initial data values with set_data for any bound components
5. ALWAYS call render_ui as the LAST step with the root component ID
6. Use descriptive IDs like "title", "volume-slider", "submit-btn"
7. For sliders/checkboxes, always set initial data with set_data
8. Use emojis in text labels to make the UI visually appealing

Example flow for "create a volume control":
1. create_text(id="volume-label", text="🔊 Volume", style="body")
2. create_slider(id="volume-slider", dataPath="/volume", min=0, max=100, step=1)
3. create_text(id="volume-value", dataPath="/volumeDisplay", style="caption")
4. create_row(id="volume-row", children=["volume-label", "volume-slider", "volume-value"])
5. set_data(path="/volume", numberValue=50)
6. set_data(path="/volumeDisplay", stringValue="50%")
7. render_ui(rootId="volume-row")"#;

            let mut messages = vec![
                json!({"role": "system", "content": system_prompt}),
            ];

            // Add conversation history
            {
                let history = state.conversation.read().await;
                messages.extend(history.clone());
            }

            // Add new user message
            messages.push(json!({"role": "user", "content": chat_req.message}));

            // Call Kimi API
            match call_kimi(&state.api_key, messages.clone()).await {
                Ok(response) => {
                    if let Some(choice) = response.choices.first() {
                        // Log reasoning if present
                        if let Some(reasoning) = &choice.message.reasoning_content {
                            println!("[Kimi Bridge] Reasoning: {}", reasoning);
                        }

                        // Process tool calls
                        if let Some(tool_calls) = &choice.message.tool_calls {
                            println!("[Kimi Bridge] Received {} tool calls", tool_calls.len());

                            let mut builder = A2uiBuilder::new();

                            for tc in tool_calls {
                                let args: Value = serde_json::from_str(&tc.function.arguments)
                                    .unwrap_or(json!({}));
                                println!("[Kimi Bridge] Tool: {}({})", tc.function.name, tc.function.arguments);
                                builder.process_tool_call(&tc.function.name, &args);
                            }

                            let a2ui_json = builder.build_a2ui_json();
                            let a2ui_str = serde_json::to_string_pretty(&a2ui_json).unwrap();

                            println!("[Kimi Bridge] Generated A2UI JSON, broadcasting...");

                            // Write to ui_live.json for watch-server
                            if let Err(e) = std::fs::write("ui_live.json", &a2ui_str) {
                                eprintln!("[Kimi Bridge] Failed to write ui_live.json: {}", e);
                            } else {
                                println!("[Kimi Bridge] Written to ui_live.json");
                            }

                            // Store latest A2UI for /rpc endpoint
                            {
                                let mut latest = state.latest_a2ui.write().await;
                                *latest = Some(a2ui_json.clone());
                            }

                            // Broadcast to connected clients
                            let _ = state.tx.send(a2ui_str.clone());

                            // Save to conversation
                            {
                                let mut history = state.conversation.write().await;
                                history.push(json!({"role": "user", "content": chat_req.message}));
                                history.push(json!({"role": "assistant", "content": format!("Generated UI with {} components", tool_calls.len())}));
                            }

                            return Ok(Response::builder()
                                .status(StatusCode::OK)
                                .header("Content-Type", "application/json")
                                .header("Access-Control-Allow-Origin", "*")
                                .body(Full::new(Bytes::from(json!({
                                    "status": "success",
                                    "components": tool_calls.len(),
                                    "a2ui": a2ui_json
                                }).to_string())))
                                .unwrap());
                        }

                        // Text response (no tool calls)
                        if let Some(content) = &choice.message.content {
                            return Ok(Response::builder()
                                .status(StatusCode::OK)
                                .header("Content-Type", "application/json")
                                .header("Access-Control-Allow-Origin", "*")
                                .body(Full::new(Bytes::from(json!({
                                    "status": "text",
                                    "message": content
                                }).to_string())))
                                .unwrap());
                        }
                    }

                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .header("Content-Type", "application/json")
                        .body(Full::new(Bytes::from(json!({"error": "Empty response from Kimi"}).to_string())))
                        .unwrap())
                }
                Err(e) => {
                    eprintln!("[Kimi Bridge] Error: {}", e);
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .header("Content-Type", "application/json")
                        .body(Full::new(Bytes::from(json!({"error": e}).to_string())))
                        .unwrap())
                }
            }
        }

        // SSE endpoint for Makepad client (A2A protocol compatible)
        (Method::POST, "/rpc") => {
            // Return latest generated UI or welcome message
            let ui_to_send = {
                let latest = state.latest_a2ui.read().await;
                if let Some(ref a2ui) = *latest {
                    a2ui.clone()
                } else {
                    // Default welcome UI
                    json!([
                        {"beginRendering": {"surfaceId": "main", "root": "welcome"}},
                        {"surfaceUpdate": {"surfaceId": "main", "components": [
                            {"id": "welcome", "component": {"Column": {"children": {"explicitList": ["title", "subtitle"]}}}},
                            {"id": "title", "component": {"Text": {"text": {"literalString": "🤖 Kimi A2UI Bridge"}, "usageHint": "h1"}}},
                            {"id": "subtitle", "component": {"Text": {"text": {"literalString": "Send a message to /chat to generate UI"}, "usageHint": "caption"}}}
                        ]}},
                        {"dataModelUpdate": {"surfaceId": "main", "path": "/", "contents": []}}
                    ])
                }
            };

            // Format as A2A SSE response with JSON-RPC wrapper
            let mut response_body = String::new();

            // Send task started first
            let task_start = json!({
                "jsonrpc": "2.0",
                "result": {
                    "kind": "task",
                    "id": "kimi-task",
                    "contextId": "kimi-ctx",
                    "status": {"state": "running"}
                }
            });
            response_body.push_str(&format!("data: {}\n\n", task_start));

            // Send each A2UI message wrapped in JSON-RPC event format
            for msg in ui_to_send.as_array().unwrap() {
                let wrapped = json!({
                    "jsonrpc": "2.0",
                    "result": {
                        "kind": "event",
                        "taskId": "kimi-task",
                        "data": msg
                    }
                });
                response_body.push_str(&format!("data: {}\n\n", wrapped));
            }

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/event-stream")
                .header("Cache-Control", "no-cache")
                .header("Access-Control-Allow-Origin", "*")
                .body(Full::new(Bytes::from(response_body)))
                .unwrap())
        }

        // Live SSE endpoint for real-time updates
        (Method::GET, "/live") => {
            let mut rx = state.tx.subscribe();

            let sse_body = match tokio::time::timeout(
                tokio::time::Duration::from_secs(30),
                rx.recv()
            ).await {
                Ok(Ok(content)) => {
                    // Parse and format as SSE
                    if let Ok(messages) = serde_json::from_str::<Vec<Value>>(&content) {
                        let mut response = String::new();
                        for msg in messages {
                            response.push_str(&format!("data: {}\n\n", msg));
                        }
                        response
                    } else {
                        format!("data: {}\n\n", content)
                    }
                }
                _ => {
                    // Timeout - send keepalive
                    "data: {\"keepalive\": true}\n\n".to_string()
                }
            };

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/event-stream")
                .header("Cache-Control", "no-cache")
                .header("Access-Control-Allow-Origin", "*")
                .body(Full::new(Bytes::from(sse_body)))
                .unwrap())
        }

        // Reset conversation
        (Method::POST, "/reset") => {
            let mut history = state.conversation.write().await;
            history.clear();

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Full::new(Bytes::from(json!({"status": "conversation reset"}).to_string())))
                .unwrap())
        }

        // Status endpoint
        (Method::GET, "/status") => {
            let history = state.conversation.read().await;
            let status = json!({
                "status": "running",
                "model": "kimi-k2.5",
                "conversation_turns": history.len() / 2,
                "endpoints": {
                    "POST /chat": "Send message to generate UI",
                    "POST /rpc": "A2A protocol endpoint (initial load)",
                    "GET /live": "SSE for real-time updates",
                    "POST /reset": "Reset conversation"
                }
            });

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Full::new(Bytes::from(status.to_string())))
                .unwrap())
        }

        // CORS preflight
        (Method::OPTIONS, _) => {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
                .header("Access-Control-Allow-Headers", "Content-Type")
                .body(Full::new(Bytes::new()))
                .unwrap())
        }

        // 404
        _ => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .body(Full::new(Bytes::from(json!({"error": "Not found"}).to_string())))
                .unwrap())
        }
    }
}

// ============================================================================
// Main
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Get API key from environment
    let api_key = std::env::var("MOONSHOT_API_KEY")
        .expect("MOONSHOT_API_KEY environment variable not set");

    let (tx, _rx) = broadcast::channel::<String>(16);

    let state = Arc::new(ServerState {
        api_key,
        tx,
        conversation: RwLock::new(Vec::new()),
        latest_a2ui: RwLock::new(None),
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    let listener = TcpListener::bind(addr).await?;

    println!("===========================================");
    println!("  Kimi A2UI Bridge Server");
    println!("===========================================");
    println!();
    println!("Server:   http://127.0.0.1:8081");
    println!("Model:    kimi-k2.5 (with tool use)");
    println!();
    println!("Endpoints:");
    println!("  POST /chat   - Send message to generate UI");
    println!("  POST /rpc    - A2A protocol (for Makepad)");
    println!("  GET  /live   - Live updates (SSE)");
    println!("  POST /reset  - Reset conversation");
    println!("  GET  /status - Server status");
    println!();
    println!("Example:");
    println!("  curl -X POST http://127.0.0.1:8081/chat \\");
    println!("    -H 'Content-Type: application/json' \\");
    println!("    -d '{{\"message\": \"Create a login form\"}}'");
    println!();
    println!("Press Ctrl+C to stop");
    println!();

    loop {
        let (stream, remote_addr) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let state = state.clone();

        println!("[Server] Connection from {}", remote_addr);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| {
                    handle_request(req, state.clone())
                }))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
