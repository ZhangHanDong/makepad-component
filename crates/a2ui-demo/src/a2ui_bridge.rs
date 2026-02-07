//! A2UI Bridge Server
//!
//! Connects any OpenAI-compatible LLM with A2UI Makepad renderer via tool use.
//!
//! Environment variables:
//!   LLM_API_URL   - Chat completions endpoint (default: https://api.moonshot.ai/v1/chat/completions)
//!   LLM_MODEL     - Model name (default: kimi-k2.5)
//!   LLM_API_KEY   - API key (or MOONSHOT_API_KEY for backwards compat)
//!   LLM_PORT      - Server port (default: 8081)

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use log::{info, warn, error, debug};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::RwLock;

#[cfg(feature = "mureka")]
const MUREKA_API_URL: &str = "https://api.mureka.ai";

include!("a2ui_bridge_impl/mureka.rs");
include!("a2ui_bridge_impl/tools.rs");
include!("a2ui_bridge_impl/types.rs");
include!("a2ui_bridge_impl/builder.rs");
include!("a2ui_bridge_impl/server.rs");

// ============================================================================
// Main
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();

    // LLM configuration from environment
    let llm_api_url = std::env::var("LLM_API_URL")
        .unwrap_or_else(|_| "https://api.moonshot.ai/v1/chat/completions".to_string());

    let llm_model = std::env::var("LLM_MODEL")
        .unwrap_or_else(|_| "kimi-k2.5".to_string());

    let api_key = std::env::var("LLM_API_KEY")
        .or_else(|_| std::env::var("MOONSHOT_API_KEY"))
        .unwrap_or_else(|_| "not-needed".to_string());

    let port: u16 = std::env::var("LLM_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8081);

    // Optional: Get Mureka API key for music generation (only with mureka feature)
    #[cfg(feature = "mureka")]
    let mureka_client = std::env::var("MUREKA_API_KEY")
        .ok()
        .map(|key| {
            info!("Mureka API key found - music generation enabled");
            MurekaClient::new(key)
        });

    #[cfg(feature = "mureka")]
    if mureka_client.is_none() {
        info!("MUREKA_API_KEY not set - music generation disabled");
    }

    #[cfg(not(feature = "mureka"))]
    info!("Mureka feature not enabled - music generation disabled");

    let (tx, _rx) = broadcast::channel::<String>(16);

    let state = Arc::new(ServerState {
        api_key,
        llm_api_url,
        llm_model,
        #[cfg(feature = "mureka")]
        mureka_client,
        tx,
        conversation: RwLock::new(Vec::new()),
        latest_a2ui: RwLock::new(None),
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;

    println!("===========================================");
    println!("  A2UI Bridge Server");
    println!("===========================================");
    println!();
    println!("Server:   http://127.0.0.1:{}", port);
    println!("LLM API:  {}", state.llm_api_url);
    println!("Model:    {}", state.llm_model);
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
    println!("Environment variables:");
    println!("  LLM_API_URL  - Chat completions endpoint");
    println!("  LLM_MODEL    - Model name");
    println!("  LLM_API_KEY  - API key (or MOONSHOT_API_KEY)");
    println!("  LLM_PORT     - Server port (default: 8081)");
    println!();
    println!("Example:");
    println!("  curl -X POST http://127.0.0.1:{}/chat \\", port);
    println!("    -H 'Content-Type: application/json' \\");
    println!("    -d '{{\"message\": \"Create a login form\"}}'");
    println!();
    println!("Press Ctrl+C to stop");
    println!();

    loop {
        let (stream, remote_addr) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let state = state.clone();

        info!("Connection from {}", remote_addr);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move |req| {
                    handle_request(req, state.clone())
                }))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}
