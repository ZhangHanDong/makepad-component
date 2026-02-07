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
            _ => warn!("Unknown tool: {}", name),
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

            warn!("Root '{}' not found, auto-creating Column with {} top-level children", root, top_level.len());

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

