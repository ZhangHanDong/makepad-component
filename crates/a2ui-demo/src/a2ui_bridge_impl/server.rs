// ============================================================================
// Server State
// ============================================================================

struct ServerState {
    api_key: String,
    llm_api_url: String,
    llm_model: String,
    #[cfg(feature = "mureka")]
    mureka_client: Option<MurekaClient>,
    tx: broadcast::Sender<String>,
    conversation: RwLock<Vec<Value>>,
    latest_a2ui: RwLock<Option<Value>>,
}

// ============================================================================
// LLM API Client
// ============================================================================

async fn call_llm(api_url: &str, model: &str, api_key: &str, messages: Vec<Value>) -> Result<LlmResponse, String> {
    let client = reqwest::Client::new();

    let request_body = json!({
        "model": model,
        "messages": messages,
        "tools": get_a2ui_tools(),
        "temperature": 1,
        "max_tokens": 8192,
        "stream": false
    });

    let response = client
        .post(api_url)
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

/// Streaming version of call_llm - broadcasts components as they arrive
async fn call_llm_stream(
    api_url: &str,
    model: &str,
    api_key: &str,
    messages: Vec<Value>,
    tx: &broadcast::Sender<String>,
) -> Result<LlmResponse, String> {
    use futures_util::StreamExt;

    let client = reqwest::Client::new();

    let request_body = json!({
        "model": model,
        "messages": messages,
        "tools": get_a2ui_tools(),
        "temperature": 1,
        "max_tokens": 8192,
        "stream": true
    });

    let response = client
        .post(api_url)
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
                                                    info!("Sent beginRendering");
                                                }

                                                // Build component JSON directly based on tool name
                                                let component = build_component_json(&entry.1, &args);
                                                if let Some(comp) = component {
                                                    let update_msg = json!([
                                                        {"surfaceUpdate": {"surfaceId": "main", "components": [comp.clone()]}}
                                                    ]);
                                                    let _ = tx.send(update_msg.to_string());
                                                    info!("Sent component: {}", entry.1);

                                                    // Accumulate and write to ui_live.json for /rpc polling
                                                    accumulated_components.push(comp);
                                                    let a2ui = json!([
                                                        {"beginRendering": {"surfaceId": "main", "root": "streaming-root"}},
                                                        {"surfaceUpdate": {"surfaceId": "main", "components": accumulated_components}},
                                                        {"dataModelUpdate": {"surfaceId": "main", "path": "/", "contents": []}}
                                                    ]);
                                                    let _ = std::fs::write("ui_live.json", serde_json::to_string(&a2ui).unwrap_or_default());
                                                    debug!("Updated ui_live.json ({} components)", accumulated_components.len());
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
    let tool_call_list: Vec<LlmToolCall> = tool_calls.into_iter()
        .filter(|(_, (_, name, _))| !name.is_empty())
        .map(|(_, (id, name, args))| LlmToolCall {
            id,
            function: LlmFunctionCall { name, arguments: args },
        })
        .collect();

    Ok(LlmResponse {
        choices: vec![LlmChoice {
            message: LlmMessage {
                content: None,
                reasoning_content: None,
                tool_calls: if tool_call_list.is_empty() { None } else { Some(tool_call_list) },
            },
        }],
    })
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

    info!("{} {}", method, path);

    match (method, path.as_str()) {
        // Chat endpoint - send message to LLM
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

            info!("User message: {}", chat_req.message);

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

            // Call LLM API with streaming (broadcasts components as they arrive)
            match call_llm_stream(&state.llm_api_url, &state.llm_model, &state.api_key, messages.clone(), &state.tx).await {
                Ok(response) => {
                    if let Some(choice) = response.choices.first() {
                        // Log reasoning if present
                        if let Some(reasoning) = &choice.message.reasoning_content {
                            debug!("Reasoning: {}", reasoning);
                        }

                        // Process tool calls
                        if let Some(tool_calls) = &choice.message.tool_calls {
                            info!("Received {} tool calls", tool_calls.len());

                            let mut builder = A2uiBuilder::new();

                            for tc in tool_calls {
                                let args: Value = serde_json::from_str(&tc.function.arguments)
                                    .unwrap_or(json!({}));
                                debug!("Tool: {}({})", tc.function.name, tc.function.arguments);
                                builder.process_tool_call(&tc.function.name, &args);
                            }

                            // Handle pending music generation (only with mureka feature)
                            #[cfg(feature = "mureka")]
                            if builder.has_pending_music() {
                                if let Some(ref mureka) = state.mureka_client {
                                    info!("Processing music generation requests...");

                                    for (prompt, instrumental) in builder.get_pending_music() {
                                        info!("Generating music: '{}' (instrumental: {})", prompt, instrumental);

                                        match mureka.generate_music(&prompt, instrumental).await {
                                            Ok(job_id) => {
                                                info!("Mureka job started: {}", job_id);
                                                info!("Waiting for music generation (this may take ~45 seconds)...");

                                                // Poll for completion (max 20 attempts = ~60 seconds)
                                                match mureka.wait_for_completion(&job_id, 20).await {
                                                    Ok(songs) => {
                                                        info!("Music generated! {} songs available", songs.len());
                                                        builder.set_generated_audio(songs.clone());

                                                        // Update audio player components with real URLs
                                                        if let Some(song) = songs.first() {
                                                            if let Some(url) = &song.audio_url {
                                                                info!("Audio URL: {}", url);
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
                                                        error!("Music generation failed: {}", e);
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                error!("Failed to start music generation: {}", e);
                                            }
                                        }
                                    }
                                } else {
                                    warn!("Music generation requested but MUREKA_API_KEY not set");
                                }
                            }

                            let a2ui_json = builder.build_a2ui_json();
                            let a2ui_str = serde_json::to_string_pretty(&a2ui_json).unwrap();

                            info!("Generated A2UI JSON, broadcasting...");

                            // Write to ui_live.json for watch-server
                            if let Err(e) = std::fs::write("ui_live.json", &a2ui_str) {
                                error!("Failed to write ui_live.json: {}", e);
                            } else {
                                info!("Written to ui_live.json");
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
                        .body(Full::new(Bytes::from(json!({"error": "Empty response from LLM"}).to_string())))
                        .unwrap())
                }
                Err(e) => {
                    error!("{}", e);
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
                            {"id": "title", "component": {"Text": {"text": {"literalString": "A2UI Bridge"}, "usageHint": "h1"}}},
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
                    "id": "a2ui-task",
                    "contextId": "a2ui-ctx",
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
                        "taskId": "a2ui-task",
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
                        info!("Sent streaming update via /live");
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
                "llm_api_url": state.llm_api_url,
                "model": state.llm_model,
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
