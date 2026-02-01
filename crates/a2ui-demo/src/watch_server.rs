//! File-watching A2A Server for live A2UI development
//!
//! Watches a JSON file and streams changes to connected clients via SSE.
//!
//! Run: cargo run -p a2ui-demo --bin watch-server --features mock-server
//! Edit: ui_live.json to see changes in real-time

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::fs;

const JSON_FILE: &str = "ui_live.json";

/// Watch the JSON file for changes and broadcast updates
async fn watch_file(tx: broadcast::Sender<String>) {
    let path = Path::new(JSON_FILE);
    let mut last_content = String::new();
    let mut last_modified = std::time::SystemTime::UNIX_EPOCH;

    println!("[Watcher] Watching file: {}", JSON_FILE);

    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // Check if file exists
        if !path.exists() {
            continue;
        }

        // Check modification time
        let metadata = match fs::metadata(path).await {
            Ok(m) => m,
            Err(_) => continue,
        };

        let modified = metadata.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);

        if modified > last_modified {
            // Read file content
            let content = match fs::read_to_string(path).await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("[Watcher] Error reading file: {}", e);
                    continue;
                }
            };

            // Only broadcast if content actually changed
            if content != last_content {
                println!("[Watcher] File changed, broadcasting update...");

                // Validate JSON
                match serde_json::from_str::<serde_json::Value>(&content) {
                    Ok(_) => {
                        let _ = tx.send(content.clone());
                        last_content = content;
                        println!("[Watcher] Update broadcast complete");
                    }
                    Err(e) => {
                        eprintln!("[Watcher] Invalid JSON: {}", e);
                    }
                }
            }

            last_modified = modified;
        }
    }
}

/// Convert JSON array to SSE messages
fn json_to_sse(json_content: &str) -> String {
    let mut sse_body = String::new();

    // Parse as array of A2UI messages
    let messages: Vec<serde_json::Value> = match serde_json::from_str(json_content) {
        Ok(m) => m,
        Err(_) => {
            // Try as single message
            if let Ok(single) = serde_json::from_str::<serde_json::Value>(json_content) {
                vec![single]
            } else {
                return sse_body;
            }
        }
    };

    // Wrap each message in JSON-RPC format for A2A protocol
    for (i, msg) in messages.iter().enumerate() {
        let wrapped = serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "event",
                "taskId": "live-task",
                "data": msg
            }
        });
        let data = serde_json::to_string(&wrapped).unwrap();
        sse_body.push_str(&format!("data: {}\n\n", data));
        println!("[Server] Sending message {}", i + 1);
    }

    sse_body
}

/// Handle incoming HTTP requests
async fn handle_request(
    req: Request<Incoming>,
    tx: broadcast::Sender<String>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    match (req.method(), req.uri().path()) {
        // CORS preflight
        (&Method::OPTIONS, _) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
                .header("Access-Control-Allow-Headers", "Content-Type, Accept, Authorization")
                .body(Full::new(Bytes::new()))
                .unwrap();
            Ok(response)
        }

        // Main RPC endpoint - initial load + subscribe to changes
        (&Method::POST, "/rpc") => {
            println!("[Server] Client connected, sending current UI...");

            // Read current file content
            let content = fs::read_to_string(JSON_FILE).await.unwrap_or_else(|_| {
                // Default empty UI if file doesn't exist
                r#"[{"beginRendering": {"surfaceId": "main", "root": "root"}}]"#.to_string()
            });

            // Send task started
            let mut sse_body = String::new();
            let task_start = serde_json::json!({
                "jsonrpc": "2.0",
                "result": {
                    "kind": "task",
                    "id": "live-task",
                    "contextId": "live-ctx",
                    "status": {"state": "running"}
                }
            });
            sse_body.push_str(&format!("data: {}\n\n", serde_json::to_string(&task_start).unwrap()));

            // Send current content
            sse_body.push_str(&json_to_sse(&content));

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/event-stream")
                .header("Cache-Control", "no-cache")
                .header("Connection", "keep-alive")
                .header("Access-Control-Allow-Origin", "*")
                .body(Full::new(Bytes::from(sse_body)))
                .unwrap();

            Ok(response)
        }

        // SSE endpoint for live updates (long-polling style)
        (&Method::GET, "/live") => {
            println!("[Server] Live update client connected, waiting for changes...");

            let mut rx = tx.subscribe();

            // Wait for next update (with timeout)
            let sse_body = match tokio::time::timeout(
                tokio::time::Duration::from_secs(30),
                rx.recv()
            ).await {
                Ok(Ok(content)) => {
                    println!("[Server] Sending live update to client");
                    json_to_sse(&content)
                }
                _ => {
                    // Timeout - send keepalive
                    "data: {\"keepalive\": true}\n\n".to_string()
                }
            };

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/event-stream")
                .header("Cache-Control", "no-cache")
                .header("Access-Control-Allow-Origin", "*")
                .body(Full::new(Bytes::from(sse_body)))
                .unwrap();

            Ok(response)
        }

        // Status endpoint
        (&Method::GET, "/status") => {
            let status = serde_json::json!({
                "status": "running",
                "watching": JSON_FILE,
                "endpoints": {
                    "POST /rpc": "Initial UI load (A2A protocol)",
                    "GET /live": "Live updates (SSE)"
                }
            });

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Full::new(Bytes::from(serde_json::to_string_pretty(&status).unwrap())))
                .unwrap();
            Ok(response)
        }

        // 404
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from("Not Found")))
                .unwrap();
            Ok(response)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await?;

    // Create broadcast channel for file changes
    let (tx, _) = broadcast::channel::<String>(16);
    let tx_clone = tx.clone();

    // Start file watcher
    tokio::spawn(async move {
        watch_file(tx_clone).await;
    });

    println!("===========================================");
    println!("  A2UI Live Server - File Watcher Mode");
    println!("===========================================");
    println!();
    println!("Watching: {}", JSON_FILE);
    println!("Server:   http://{}", addr);
    println!();
    println!("Endpoints:");
    println!("  POST /rpc  - A2A protocol (initial load)");
    println!("  GET /live  - Live updates (SSE)");
    println!("  GET /status - Server status");
    println!();
    println!("Edit {} to update the UI!", JSON_FILE);
    println!("Press Ctrl+C to stop");
    println!();

    loop {
        let (stream, remote_addr) = listener.accept().await?;
        println!("[Server] Connection from {}", remote_addr);

        let io = TokioIo::new(stream);
        let tx = tx.clone();

        tokio::task::spawn(async move {
            let service = service_fn(move |req| {
                let tx = tx.clone();
                async move { handle_request(req, tx).await }
            });

            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service)
                .await
            {
                eprintln!("[Server] Connection error: {:?}", err);
            }
        });
    }
}
