//! Mock A2A Server for testing A2UI streaming
//!
//! Run: cargo run -p a2ui-demo --bin mock-a2a-server --features mock-server
//! Then connect from Makepad app to http://localhost:8080/rpc

use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Sample A2UI messages to stream
fn sample_messages() -> Vec<serde_json::Value> {
    vec![
        // Task running status
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "task",
                "id": "task-123",
                "contextId": "ctx-456",
                "status": {"state": "running"}
            }
        }),
        // BeginRendering
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "event",
                "taskId": "task-123",
                "data": {
                    "beginRendering": {
                        "surfaceId": "main",
                        "root": "root-column"
                    }
                }
            }
        }),
        // SurfaceUpdate with components
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "event",
                "taskId": "task-123",
                "data": {
                    "surfaceUpdate": {
                        "surfaceId": "main",
                        "components": [
                            {
                                "id": "root-column",
                                "component": {
                                    "Column": {
                                        "children": {"explicitList": ["greeting", "input-row"]}
                                    }
                                }
                            },
                            {
                                "id": "greeting",
                                "component": {
                                    "Text": {
                                        "text": {"path": "/message"},
                                        "usageHint": "h1"
                                    }
                                }
                            },
                            {
                                "id": "input-row",
                                "component": {
                                    "Row": {
                                        "children": {"explicitList": ["name-input", "greet-btn"]}
                                    }
                                }
                            },
                            {
                                "id": "name-input",
                                "component": {
                                    "TextField": {
                                        "text": {"path": "/name"},
                                        "placeholder": {"literalString": "Enter your name..."}
                                    }
                                }
                            },
                            {
                                "id": "greet-btn-text",
                                "component": {
                                    "Text": {"text": {"literalString": "Say Hello"}}
                                }
                            },
                            {
                                "id": "greet-btn",
                                "component": {
                                    "Button": {
                                        "child": "greet-btn-text",
                                        "action": {
                                            "name": "greet",
                                            "context": [
                                                {"key": "name", "value": {"path": "/name"}}
                                            ]
                                        }
                                    }
                                }
                            }
                        ]
                    }
                }
            }
        }),
        // DataModelUpdate
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "event",
                "taskId": "task-123",
                "data": {
                    "dataModelUpdate": {
                        "surfaceId": "main",
                        "path": "/",
                        "contents": [
                            {"key": "message", "valueString": "Hello from Rust A2A Server!"},
                            {"key": "name", "valueString": ""}
                        ]
                    }
                }
            }
        }),
        // Task completed
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "task",
                "id": "task-123",
                "contextId": "ctx-456",
                "status": {"state": "completed"}
            }
        }),
    ]
}

/// Handle incoming HTTP requests
async fn handle_request(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    match (req.method(), req.uri().path()) {
        // CORS preflight
        (&Method::OPTIONS, _) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "POST, OPTIONS")
                .header(
                    "Access-Control-Allow-Headers",
                    "Content-Type, Accept, Authorization, X-A2A-Extensions",
                )
                .body(Full::new(Bytes::new()))
                .unwrap();
            Ok(response)
        }

        // Main RPC endpoint
        (&Method::POST, "/rpc") => {
            // Read request body
            let body_bytes = req.collect().await.unwrap().to_bytes();
            let body_str = String::from_utf8_lossy(&body_bytes);
            println!("[Mock Server] Received request: {}...", &body_str[..body_str.len().min(200)]);

            // Build SSE response body
            let messages = sample_messages();
            let mut sse_body = String::new();

            for (i, msg) in messages.iter().enumerate() {
                let data = serde_json::to_string(msg).unwrap();
                sse_body.push_str(&format!("data: {}\n\n", data));
                println!("[Mock Server] Queued message {}/{}", i + 1, messages.len());
            }

            println!("[Mock Server] Stream complete");

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

        // Streaming endpoint with delays
        (&Method::POST, "/rpc-stream") => {
            let body_bytes = req.collect().await.unwrap().to_bytes();
            let body_str = String::from_utf8_lossy(&body_bytes);
            println!("[Mock Server] Received streaming request: {}...", &body_str[..body_str.len().min(200)]);

            // For true streaming with delays, we need chunked transfer
            // This simplified version sends all at once
            let messages = sample_messages();
            let mut sse_body = String::new();

            for (i, msg) in messages.iter().enumerate() {
                let data = serde_json::to_string(msg).unwrap();
                sse_body.push_str(&format!("data: {}\n\n", data));
                println!("[Mock Server] Prepared message {}/{}", i + 1, messages.len());
            }

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/event-stream")
                .header("Cache-Control", "no-cache")
                .header("Access-Control-Allow-Origin", "*")
                .body(Full::new(Bytes::from(sse_body)))
                .unwrap();

            Ok(response)
        }

        // 404 for everything else
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

    println!("===========================================");
    println!("  Mock A2A Server (Rust/Tokio)");
    println!("===========================================");
    println!("Listening on http://{}/rpc", addr);
    println!("Press Ctrl+C to stop");
    println!();

    loop {
        let (stream, remote_addr) = listener.accept().await?;
        println!("[Mock Server] Connection from {}", remote_addr);

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_request))
                .await
            {
                eprintln!("[Mock Server] Connection error: {:?}", err);
            }
        });
    }
}
