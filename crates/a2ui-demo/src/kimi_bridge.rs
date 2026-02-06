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
#[cfg(feature = "mureka")]
const MUREKA_API_URL: &str = "https://api.mureka.ai";

// ============================================================================
// Mureka API Client (optional feature)
// ============================================================================

#[cfg(feature = "mureka")]
#[derive(Debug, Clone)]
struct MurekaClient {
    api_key: String,
    client: reqwest::Client,
}

#[cfg(feature = "mureka")]
#[derive(Debug, Deserialize)]
struct MurekaGenerateResponse {
    job_id: String,
}

#[cfg(feature = "mureka")]
#[derive(Debug, Deserialize)]
struct MurekaJobStatus {
    status: String,  // "pending", "processing", "completed", "failed"
    #[serde(default)]
    songs: Vec<MurekaSong>,
}

#[cfg(feature = "mureka")]
#[derive(Debug, Deserialize, Clone)]
struct MurekaSong {
    #[allow(dead_code)]
    id: String,
    #[serde(default)]
    audio_url: Option<String>,
    #[serde(default)]
    title: Option<String>,
}

#[cfg(feature = "mureka")]
impl MurekaClient {
    fn new(api_key: String) -> Self {
        MurekaClient {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    async fn generate_music(&self, prompt: &str, instrumental: bool) -> Result<String, String> {
        let body = if instrumental {
            json!({
                "description": prompt,
                "instrumental": true
            })
        } else {
            json!({
                "lyrics": prompt
            })
        };

        let response = self.client
            .post(format!("{}/v1/song/generate", MUREKA_API_URL))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Mureka request failed: {}", e))?;

        let status = response.status();
        let body = response.text().await.map_err(|e| format!("Failed to read Mureka response: {}", e))?;

        if !status.is_success() {
            return Err(format!("Mureka API error ({}): {}", status, body));
        }

        let result: MurekaGenerateResponse = serde_json::from_str(&body)
            .map_err(|e| format!("Failed to parse Mureka response: {} - Body: {}", e, body))?;

        Ok(result.job_id)
    }

    async fn poll_job(&self, job_id: &str) -> Result<MurekaJobStatus, String> {
        let response = self.client
            .get(format!("{}/v1/song/generate/jobs/{}", MUREKA_API_URL, job_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| format!("Mureka poll failed: {}", e))?;

        let status = response.status();
        let body = response.text().await.map_err(|e| format!("Failed to read Mureka poll response: {}", e))?;

        if !status.is_success() {
            return Err(format!("Mureka poll error ({}): {}", status, body));
        }

        let result: MurekaJobStatus = serde_json::from_str(&body)
            .map_err(|e| format!("Failed to parse Mureka poll response: {} - Body: {}", e, body))?;

        Ok(result)
    }

    /// Wait for music generation to complete (with timeout)
    async fn wait_for_completion(&self, job_id: &str, max_attempts: u32) -> Result<Vec<MurekaSong>, String> {
        for attempt in 0..max_attempts {
            let status = self.poll_job(job_id).await?;

            match status.status.as_str() {
                "completed" => {
                    return Ok(status.songs);
                }
                "failed" => {
                    return Err("Music generation failed".to_string());
                }
                _ => {
                    // Still processing, wait and retry
                    println!("[Mureka] Job {} status: {} (attempt {}/{})", job_id, status.status, attempt + 1, max_attempts);
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                }
            }
        }

        Err("Music generation timed out".to_string())
    }
}

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
        },
        {
            "type": "function",
            "function": {
                "name": "generate_music",
                "description": "Generate AI music using Mureka API. Returns a job ID that will be polled for completion. The music generation takes about 45 seconds.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "prompt": {"type": "string", "description": "Description of the music to generate (e.g., 'relaxing piano melody', 'upbeat electronic dance')"},
                        "instrumental": {"type": "boolean", "description": "If true, generate instrumental music without lyrics. Default true."}
                    },
                    "required": ["prompt"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "create_audio_player",
                "description": "Create an audio player component to play music. Use this after generate_music returns an audio URL.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string", "description": "Unique component ID"},
                        "url": {"type": "string", "description": "Audio URL to play"},
                        "title": {"type": "string", "description": "Song title"},
                        "artist": {"type": "string", "description": "Artist name (optional)"}
                    },
                    "required": ["id", "url", "title"]
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
    #[cfg(feature = "mureka")]
    /// Pending music generation requests (prompt, instrumental)
    pending_music: Vec<(String, bool)>,
    #[cfg(feature = "mureka")]
    /// Generated audio URLs from Mureka
    generated_audio: Vec<MurekaSong>,
}

impl A2uiBuilder {
    fn new() -> Self {
        A2uiBuilder {
            components: Vec::new(),
            data_contents: Vec::new(),
            root_id: None,
            #[cfg(feature = "mureka")]
            pending_music: Vec::new(),
            #[cfg(feature = "mureka")]
            generated_audio: Vec::new(),
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
            #[cfg(feature = "mureka")]
            "generate_music" => self.generate_music(args),
            "create_audio_player" => self.create_audio_player(args),
            _ => eprintln!("Unknown tool: {}", name),
        }
    }

    #[cfg(feature = "mureka")]
    fn generate_music(&mut self, args: &Value) {
        let prompt = args["prompt"].as_str().unwrap_or("relaxing music").to_string();
        let instrumental = args["instrumental"].as_bool().unwrap_or(true);
        self.pending_music.push((prompt, instrumental));
    }

    fn create_audio_player(&mut self, args: &Value) {
        let id = args["id"].as_str().unwrap_or("audio-player");
        let url = args["url"].as_str().unwrap_or("");
        let title = args["title"].as_str().unwrap_or("Audio");
        let artist = args["artist"].as_str();

        let mut audio_player = json!({
            "url": {"literalString": url},
            "title": {"literalString": title}
        });

        if let Some(artist_name) = artist {
            audio_player["artist"] = json!({"literalString": artist_name});
        }

        self.components.push(json!({
            "id": id,
            "component": {
                "AudioPlayer": audio_player
            }
        }));
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

    #[cfg(feature = "mureka")]
    fn has_pending_music(&self) -> bool {
        !self.pending_music.is_empty()
    }

    #[cfg(feature = "mureka")]
    fn get_pending_music(&self) -> Vec<(String, bool)> {
        self.pending_music.clone()
    }

    #[cfg(feature = "mureka")]
    fn set_generated_audio(&mut self, songs: Vec<MurekaSong>) {
        self.generated_audio = songs;
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
    #[cfg(feature = "mureka")]
    mureka_client: Option<MurekaClient>,
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

/// Streaming version of call_kimi - broadcasts components as they arrive
async fn call_kimi_stream(
    api_key: &str,
    messages: Vec<Value>,
    tx: &broadcast::Sender<String>,
) -> Result<KimiResponse, String> {
    use futures_util::StreamExt;

    let client = reqwest::Client::new();

    let request_body = json!({
        "model": "kimi-k2.5",
        "messages": messages,
        "tools": get_a2ui_tools(),
        "temperature": 1,
        "max_tokens": 8192,
        "stream": true
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
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("API error ({}): {}", status, body));
    }

    // Accumulate tool calls from stream
    let mut tool_calls: HashMap<i64, (String, String, String)> = HashMap::new(); // index -> (id, name, arguments)
    let mut processed_indices: std::collections::HashSet<i64> = std::collections::HashSet::new();
    let mut sent_begin = false;
    let mut accumulated_components: Vec<Value> = Vec::new(); // For ui_live.json updates

    // Clear ui_live.json at start of new stream
    let _ = std::fs::write("ui_live.json", "[]");

    let mut stream = response.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Stream error: {}", e))?;
        buffer.push_str(&String::from_utf8_lossy(&chunk));

        // Process complete SSE lines
        while let Some(pos) = buffer.find("\n\n") {
            let line = buffer[..pos].to_string();
            buffer = buffer[pos + 2..].to_string();

            if line.starts_with("data: ") {
                let data = &line[6..];
                if data == "[DONE]" {
                    continue;
                }

                if let Ok(chunk_json) = serde_json::from_str::<Value>(data) {
                    // Extract delta tool_calls
                    if let Some(choices) = chunk_json.get("choices").and_then(|c| c.as_array()) {
                        for choice in choices {
                            if let Some(delta) = choice.get("delta") {
                                if let Some(calls) = delta.get("tool_calls").and_then(|t| t.as_array()) {
                                    for call in calls {
                                        let index = call.get("index").and_then(|i| i.as_i64()).unwrap_or(0);
                                        let id = call.get("id").and_then(|s| s.as_str()).unwrap_or("");
                                        let func = call.get("function");
                                        let name = func.and_then(|f| f.get("name")).and_then(|n| n.as_str()).unwrap_or("");
                                        let args_chunk = func.and_then(|f| f.get("arguments")).and_then(|a| a.as_str()).unwrap_or("");

                                        let entry = tool_calls.entry(index).or_insert_with(|| (String::new(), String::new(), String::new()));

                                        if !id.is_empty() {
                                            entry.0 = id.to_string();
                                        }
                                        if !name.is_empty() {
                                            entry.1 = name.to_string();
                                        }
                                        entry.2.push_str(args_chunk);

                                        // Try to parse complete arguments and send component immediately
                                        if !entry.1.is_empty() && !entry.2.is_empty() && !processed_indices.contains(&index) {
                                            if let Ok(args) = serde_json::from_str::<Value>(&entry.2) {
                                                // Mark as processed
                                                processed_indices.insert(index);

                                                // Send beginRendering on first component
                                                if !sent_begin {
                                                    sent_begin = true;
                                                    let begin_msg = json!([
                                                        {"beginRendering": {"surfaceId": "main", "root": "streaming-root"}}
                                                    ]);
                                                    let _ = tx.send(begin_msg.to_string());
                                                    println!("[Stream] Sent beginRendering");
                                                }

                                                // Build component JSON directly based on tool name
                                                let component = build_component_json(&entry.1, &args);
                                                if let Some(comp) = component {
                                                    let update_msg = json!([
                                                        {"surfaceUpdate": {"surfaceId": "main", "components": [comp.clone()]}}
                                                    ]);
                                                    let _ = tx.send(update_msg.to_string());
                                                    println!("[Stream] Sent component: {}", entry.1);

                                                    // Accumulate and write to ui_live.json for /rpc polling
                                                    accumulated_components.push(comp);
                                                    let a2ui = json!([
                                                        {"beginRendering": {"surfaceId": "main", "root": "streaming-root"}},
                                                        {"surfaceUpdate": {"surfaceId": "main", "components": accumulated_components}},
                                                        {"dataModelUpdate": {"surfaceId": "main", "path": "/", "contents": []}}
                                                    ]);
                                                    let _ = std::fs::write("ui_live.json", serde_json::to_string(&a2ui).unwrap_or_default());
                                                    println!("[Stream] Updated ui_live.json ({} components)", accumulated_components.len());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Build final response from accumulated tool calls
    // Note: Components have already been streamed, this is for compatibility
    let tool_call_list: Vec<KimiToolCall> = tool_calls.into_iter()
        .filter(|(_, (_, name, _))| !name.is_empty())
        .map(|(_, (id, name, args))| KimiToolCall {
            id,
            function: KimiFunctionCall { name, arguments: args },
        })
        .collect();

    Ok(KimiResponse {
        choices: vec![KimiChoice {
            message: KimiMessage {
                content: None,
                reasoning_content: None,
                tool_calls: if tool_call_list.is_empty() { None } else { Some(tool_call_list) },
            },
        }],
    })
}

/// Build component JSON directly from tool name and args (for streaming)
fn build_component_json(name: &str, args: &Value) -> Option<Value> {
    let id = args.get("id").and_then(|v| v.as_str()).unwrap_or("comp");

    match name {
        "create_text" => {
            let text = args.get("text").and_then(|v| v.as_str()).unwrap_or("");
            let usage_hint = args.get("usage_hint").and_then(|v| v.as_str());
            let mut text_obj = json!({
                "text": {"literalString": text}
            });
            if let Some(hint) = usage_hint {
                text_obj["usageHint"] = json!(hint);
            }
            Some(json!({"id": id, "component": {"Text": text_obj}}))
        }
        "create_audio_player" => {
            let url = args.get("url").and_then(|v| v.as_str()).unwrap_or("");
            let title = args.get("title").and_then(|v| v.as_str()).unwrap_or("Audio");
            let artist = args.get("artist").and_then(|v| v.as_str());
            let mut audio_obj = json!({
                "url": {"literalString": url},
                "title": {"literalString": title}
            });
            if let Some(a) = artist {
                audio_obj["artist"] = json!({"literalString": a});
            }
            Some(json!({"id": id, "component": {"AudioPlayer": audio_obj}}))
        }
        "create_column" => {
            let children = args.get("children").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
                .unwrap_or_default();
            Some(json!({"id": id, "component": {"Column": {"children": {"explicitList": children}}}}))
        }
        "create_row" => {
            let children = args.get("children").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
                .unwrap_or_default();
            Some(json!({"id": id, "component": {"Row": {"children": {"explicitList": children}}}}))
        }
        "create_button" => {
            let label = args.get("label").and_then(|v| v.as_str()).unwrap_or("Button");
            let label_id = format!("{}-label", id);
            Some(json!({"id": id, "component": {"Button": {"child": label_id, "primary": true}}}))
        }
        "create_card" => {
            let child = args.get("child").and_then(|v| v.as_str()).unwrap_or("");
            Some(json!({"id": id, "component": {"Card": {"child": child}}}))
        }
        "render_ui" => {
            // This is the final layout - handled separately
            None
        }
        _ => None
    }
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
7. render_ui(rootId="volume-row")

MUSIC GENERATION:
When the user asks you to generate music (e.g., "生成一首轻松的钢琴曲", "create relaxing music"):
1. First call generate_music(prompt="description of the music", instrumental=true/false)
2. The system will wait for Mureka AI to generate the music (~45 seconds)
3. The audio URL will be provided to you automatically
4. Then create an audio player: create_audio_player(id="player", url="<audio_url>", title="Song Title")
5. Wrap it in a nice UI with a title and card
6. Call render_ui() at the end

Example for music generation:
1. create_text(id="title", text="🎵 AI Generated Music", style="h1")
2. generate_music(prompt="relaxing piano melody with soft ambient sounds", instrumental=true)
3. create_audio_player(id="player", url="<will be filled>", title="Relaxing Piano")
4. create_column(id="root", children=["title", "player"])
5. render_ui(rootId="root")"#;

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

            // Call Kimi API with streaming (broadcasts components as they arrive)
            match call_kimi_stream(&state.api_key, messages.clone(), &state.tx).await {
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

                            // Handle pending music generation (only with mureka feature)
                            #[cfg(feature = "mureka")]
                            if builder.has_pending_music() {
                                if let Some(ref mureka) = state.mureka_client {
                                    println!("[Kimi Bridge] Processing music generation requests...");

                                    for (prompt, instrumental) in builder.get_pending_music() {
                                        println!("[Kimi Bridge] Generating music: '{}' (instrumental: {})", prompt, instrumental);

                                        match mureka.generate_music(&prompt, instrumental).await {
                                            Ok(job_id) => {
                                                println!("[Kimi Bridge] Mureka job started: {}", job_id);
                                                println!("[Kimi Bridge] Waiting for music generation (this may take ~45 seconds)...");

                                                // Poll for completion (max 20 attempts = ~60 seconds)
                                                match mureka.wait_for_completion(&job_id, 20).await {
                                                    Ok(songs) => {
                                                        println!("[Kimi Bridge] Music generated! {} songs available", songs.len());
                                                        builder.set_generated_audio(songs.clone());

                                                        // Update audio player components with real URLs
                                                        if let Some(song) = songs.first() {
                                                            if let Some(url) = &song.audio_url {
                                                                println!("[Kimi Bridge] Audio URL: {}", url);
                                                                // Find and update AudioPlayer components
                                                                for comp in &mut builder.components {
                                                                    if let Some(audio_player) = comp.get_mut("component")
                                                                        .and_then(|c| c.get_mut("AudioPlayer"))
                                                                    {
                                                                        audio_player["url"] = json!({"literalString": url});
                                                                        if let Some(title) = &song.title {
                                                                            audio_player["title"] = json!({"literalString": title});
                                                                        }
                                                                        audio_player["artist"] = json!({"literalString": "Mureka AI"});
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    Err(e) => {
                                                        eprintln!("[Kimi Bridge] Music generation failed: {}", e);
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!("[Kimi Bridge] Failed to start music generation: {}", e);
                                            }
                                        }
                                    }
                                } else {
                                    println!("[Kimi Bridge] Warning: Music generation requested but MUREKA_API_KEY not set");
                                }
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
            // Read from ui_live.json for real-time streaming updates
            let ui_to_send = if let Ok(content) = std::fs::read_to_string("ui_live.json") {
                if let Ok(json) = serde_json::from_str::<Value>(&content) {
                    json
                } else {
                    // Fallback to latest_a2ui
                    let latest = state.latest_a2ui.read().await;
                    latest.clone().unwrap_or_else(|| json!([]))
                }
            } else {
                // Fallback to latest_a2ui or default welcome
                let latest = state.latest_a2ui.read().await;
                if let Some(ref a2ui) = *latest {
                    a2ui.clone()
                } else {
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

        // Live SSE endpoint for real-time streaming updates
        (Method::GET, "/live") => {
            let mut rx = state.tx.subscribe();
            let mut sse_body = String::new();

            // Keep receiving messages until timeout or channel closes
            loop {
                match tokio::time::timeout(
                    tokio::time::Duration::from_secs(60),
                    rx.recv()
                ).await {
                    Ok(Ok(content)) => {
                        // Parse and format as SSE
                        if let Ok(messages) = serde_json::from_str::<Vec<Value>>(&content) {
                            for msg in messages {
                                sse_body.push_str(&format!("data: {}\n\n", msg));
                            }
                        } else {
                            sse_body.push_str(&format!("data: {}\n\n", content));
                        }
                        println!("[Live SSE] Sent streaming update");
                    }
                    Ok(Err(_)) => {
                        // Channel closed
                        break;
                    }
                    Err(_) => {
                        // Timeout - send keepalive and continue
                        sse_body.push_str("data: {\"keepalive\": true}\n\n");
                        break; // Exit after timeout for now
                    }
                }
            }

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

    // Optional: Get Mureka API key for music generation (only with mureka feature)
    #[cfg(feature = "mureka")]
    let mureka_client = std::env::var("MUREKA_API_KEY")
        .ok()
        .map(|key| {
            println!("[Kimi Bridge] Mureka API key found - music generation enabled");
            MurekaClient::new(key)
        });

    #[cfg(feature = "mureka")]
    if mureka_client.is_none() {
        println!("[Kimi Bridge] MUREKA_API_KEY not set - music generation disabled");
    }

    #[cfg(not(feature = "mureka"))]
    println!("[Kimi Bridge] Mureka feature not enabled - music generation disabled");

    let (tx, _rx) = broadcast::channel::<String>(16);

    let state = Arc::new(ServerState {
        api_key,
        #[cfg(feature = "mureka")]
        mureka_client,
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
    #[cfg(feature = "mureka")]
    println!("Music:    {} (set MUREKA_API_KEY to enable)",
        if state.mureka_client.is_some() { "enabled" } else { "disabled" });
    #[cfg(not(feature = "mureka"))]
    println!("Music:    disabled (compile with --features mureka)");
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
    println!("Music example:");
    println!("  curl -X POST http://127.0.0.1:8081/chat \\");
    println!("    -H 'Content-Type: application/json' \\");
    println!("    -d '{{\"message\": \"生成一首轻松的钢琴曲\"}}'");
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
