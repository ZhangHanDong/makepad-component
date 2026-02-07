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
use log::{info, error};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// Generate streaming A2UI messages for a Payment Page demo
/// Messages are split to demonstrate progressive streaming rendering
fn sample_messages() -> Vec<serde_json::Value> {
    vec![
        // 1. Task started
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "task",
                "id": "task-pay-001",
                "contextId": "ctx-pay",
                "status": {"state": "running"}
            }
        }),

        // 2. Begin rendering - initialize surface
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "event",
                "taskId": "task-pay-001",
                "data": {
                    "beginRendering": {
                        "surfaceId": "main",
                        "root": "payment-root"
                    }
                }
            }
        }),

        // 3. First wave: Root structure + Title
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "event",
                "taskId": "task-pay-001",
                "data": {
                    "surfaceUpdate": {
                        "surfaceId": "main",
                        "components": [
                            {
                                "id": "payment-root",
                                "component": {
                                    "Column": {
                                        "children": {"explicitList": [
                                            "title",
                                            "subtitle",
                                            "order-card",
                                            "payment-card",
                                            "summary-card",
                                            "action-row"
                                        ]}
                                    }
                                }
                            },
                            {
                                "id": "title",
                                "component": {
                                    "Text": {
                                        "text": {"literalString": "💳 Payment Checkout"},
                                        "usageHint": "h1"
                                    }
                                }
                            },
                            {
                                "id": "subtitle",
                                "component": {
                                    "Text": {
                                        "text": {"literalString": "Review your order and complete payment"},
                                        "usageHint": "caption"
                                    }
                                }
                            }
                        ]
                    }
                }
            }
        }),

        // 4. Second wave: Order Card with items
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "event",
                "taskId": "task-pay-001",
                "data": {
                    "surfaceUpdate": {
                        "surfaceId": "main",
                        "components": [
                            {
                                "id": "order-card",
                                "component": {
                                    "Card": { "child": "order-content" }
                                }
                            },
                            {
                                "id": "order-content",
                                "component": {
                                    "Column": {
                                        "children": {"explicitList": [
                                            "order-title",
                                            "item-1",
                                            "item-2",
                                            "item-3"
                                        ]}
                                    }
                                }
                            },
                            {
                                "id": "order-title",
                                "component": {
                                    "Text": {
                                        "text": {"literalString": "📦 Order Items"},
                                        "usageHint": "h3"
                                    }
                                }
                            },
                            // Item 1
                            {
                                "id": "item-1",
                                "component": {
                                    "Row": {
                                        "children": {"explicitList": ["item-1-name", "item-1-qty", "item-1-price"]}
                                    }
                                }
                            },
                            {
                                "id": "item-1-name",
                                "component": {
                                    "Text": { "text": {"path": "/order/items/0/name"} }
                                }
                            },
                            {
                                "id": "item-1-qty",
                                "component": {
                                    "Text": { "text": {"path": "/order/items/0/qty"}, "usageHint": "caption" }
                                }
                            },
                            {
                                "id": "item-1-price",
                                "component": {
                                    "Text": { "text": {"path": "/order/items/0/price"} }
                                }
                            },
                            // Item 2
                            {
                                "id": "item-2",
                                "component": {
                                    "Row": {
                                        "children": {"explicitList": ["item-2-name", "item-2-qty", "item-2-price"]}
                                    }
                                }
                            },
                            {
                                "id": "item-2-name",
                                "component": {
                                    "Text": { "text": {"path": "/order/items/1/name"} }
                                }
                            },
                            {
                                "id": "item-2-qty",
                                "component": {
                                    "Text": { "text": {"path": "/order/items/1/qty"}, "usageHint": "caption" }
                                }
                            },
                            {
                                "id": "item-2-price",
                                "component": {
                                    "Text": { "text": {"path": "/order/items/1/price"} }
                                }
                            },
                            // Item 3
                            {
                                "id": "item-3",
                                "component": {
                                    "Row": {
                                        "children": {"explicitList": ["item-3-name", "item-3-qty", "item-3-price"]}
                                    }
                                }
                            },
                            {
                                "id": "item-3-name",
                                "component": {
                                    "Text": { "text": {"path": "/order/items/2/name"} }
                                }
                            },
                            {
                                "id": "item-3-qty",
                                "component": {
                                    "Text": { "text": {"path": "/order/items/2/qty"}, "usageHint": "caption" }
                                }
                            },
                            {
                                "id": "item-3-price",
                                "component": {
                                    "Text": { "text": {"path": "/order/items/2/price"} }
                                }
                            }
                        ]
                    }
                }
            }
        }),

        // 5. Third wave: Payment method selection
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "event",
                "taskId": "task-pay-001",
                "data": {
                    "surfaceUpdate": {
                        "surfaceId": "main",
                        "components": [
                            {
                                "id": "payment-card",
                                "component": {
                                    "Card": { "child": "payment-content" }
                                }
                            },
                            {
                                "id": "payment-content",
                                "component": {
                                    "Column": {
                                        "children": {"explicitList": [
                                            "payment-title",
                                            "method-credit",
                                            "method-paypal",
                                            "method-alipay",
                                            "method-wechat"
                                        ]}
                                    }
                                }
                            },
                            {
                                "id": "payment-title",
                                "component": {
                                    "Text": {
                                        "text": {"literalString": "💰 Payment Method"},
                                        "usageHint": "h3"
                                    }
                                }
                            },
                            {
                                "id": "method-credit",
                                "component": {
                                    "CheckBox": {
                                        "label": {"literalString": "💳 Credit Card"},
                                        "value": {"path": "/payment/creditCard"}
                                    }
                                }
                            },
                            {
                                "id": "method-paypal",
                                "component": {
                                    "CheckBox": {
                                        "label": {"literalString": "🅿️ PayPal"},
                                        "value": {"path": "/payment/paypal"}
                                    }
                                }
                            },
                            {
                                "id": "method-alipay",
                                "component": {
                                    "Row": {
                                        "children": {"explicitList": ["alipay-icon", "alipay-checkbox"]}
                                    }
                                }
                            },
                            {
                                "id": "alipay-icon",
                                "component": {
                                    "Image": {
                                        "url": {"literalString": "alipay.png"},
                                        "usageHint": "icon"
                                    }
                                }
                            },
                            {
                                "id": "alipay-checkbox",
                                "component": {
                                    "CheckBox": {
                                        "label": {"literalString": "Alipay 支付宝"},
                                        "value": {"path": "/payment/alipay"}
                                    }
                                }
                            },
                            {
                                "id": "method-wechat",
                                "component": {
                                    "Row": {
                                        "children": {"explicitList": ["wechat-icon", "wechat-checkbox"]}
                                    }
                                }
                            },
                            {
                                "id": "wechat-icon",
                                "component": {
                                    "Image": {
                                        "url": {"literalString": "wechat.png"},
                                        "usageHint": "icon"
                                    }
                                }
                            },
                            {
                                "id": "wechat-checkbox",
                                "component": {
                                    "CheckBox": {
                                        "label": {"literalString": "WeChat Pay 微信支付"},
                                        "value": {"path": "/payment/wechat"}
                                    }
                                }
                            }
                        ]
                    }
                }
            }
        }),

        // 6. Fourth wave: Summary card with totals
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "event",
                "taskId": "task-pay-001",
                "data": {
                    "surfaceUpdate": {
                        "surfaceId": "main",
                        "components": [
                            {
                                "id": "summary-card",
                                "component": {
                                    "Card": { "child": "summary-content" }
                                }
                            },
                            {
                                "id": "summary-content",
                                "component": {
                                    "Column": {
                                        "children": {"explicitList": [
                                            "summary-title",
                                            "subtotal-row",
                                            "shipping-row",
                                            "tax-row",
                                            "total-row"
                                        ]}
                                    }
                                }
                            },
                            {
                                "id": "summary-title",
                                "component": {
                                    "Text": {
                                        "text": {"literalString": "📊 Order Summary"},
                                        "usageHint": "h3"
                                    }
                                }
                            },
                            // Subtotal
                            {
                                "id": "subtotal-row",
                                "component": {
                                    "Row": {
                                        "children": {"explicitList": ["subtotal-label", "subtotal-value"]}
                                    }
                                }
                            },
                            {
                                "id": "subtotal-label",
                                "component": {
                                    "Text": { "text": {"literalString": "Subtotal:"} }
                                }
                            },
                            {
                                "id": "subtotal-value",
                                "component": {
                                    "Text": { "text": {"path": "/summary/subtotal"} }
                                }
                            },
                            // Shipping
                            {
                                "id": "shipping-row",
                                "component": {
                                    "Row": {
                                        "children": {"explicitList": ["shipping-label", "shipping-value"]}
                                    }
                                }
                            },
                            {
                                "id": "shipping-label",
                                "component": {
                                    "Text": { "text": {"literalString": "Shipping:"} }
                                }
                            },
                            {
                                "id": "shipping-value",
                                "component": {
                                    "Text": { "text": {"path": "/summary/shipping"} }
                                }
                            },
                            // Tax
                            {
                                "id": "tax-row",
                                "component": {
                                    "Row": {
                                        "children": {"explicitList": ["tax-label", "tax-value"]}
                                    }
                                }
                            },
                            {
                                "id": "tax-label",
                                "component": {
                                    "Text": { "text": {"literalString": "Tax:"} }
                                }
                            },
                            {
                                "id": "tax-value",
                                "component": {
                                    "Text": { "text": {"path": "/summary/tax"} }
                                }
                            },
                            // Total
                            {
                                "id": "total-row",
                                "component": {
                                    "Row": {
                                        "children": {"explicitList": ["total-label", "total-value"]}
                                    }
                                }
                            },
                            {
                                "id": "total-label",
                                "component": {
                                    "Text": { "text": {"literalString": "Total:"}, "usageHint": "h2" }
                                }
                            },
                            {
                                "id": "total-value",
                                "component": {
                                    "Text": { "text": {"path": "/summary/total"}, "usageHint": "h2" }
                                }
                            }
                        ]
                    }
                }
            }
        }),

        // 7. Fifth wave: Action buttons
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "event",
                "taskId": "task-pay-001",
                "data": {
                    "surfaceUpdate": {
                        "surfaceId": "main",
                        "components": [
                            {
                                "id": "action-row",
                                "component": {
                                    "Row": {
                                        "children": {"explicitList": ["cancel-btn", "pay-btn"]}
                                    }
                                }
                            },
                            {
                                "id": "cancel-btn-text",
                                "component": {
                                    "Text": { "text": {"literalString": "❌ Cancel"} }
                                }
                            },
                            {
                                "id": "cancel-btn",
                                "component": {
                                    "Button": {
                                        "child": "cancel-btn-text",
                                        "action": { "name": "cancelPayment", "context": [] }
                                    }
                                }
                            },
                            {
                                "id": "pay-btn-text",
                                "component": {
                                    "Text": { "text": {"literalString": "✅ Confirm & Pay"} }
                                }
                            },
                            {
                                "id": "pay-btn",
                                "component": {
                                    "Button": {
                                        "child": "pay-btn-text",
                                        "primary": true,
                                        "action": {
                                            "name": "confirmPayment",
                                            "context": [
                                                {"key": "total", "value": {"path": "/summary/total"}}
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

        // 8. Data model update with all values
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "event",
                "taskId": "task-pay-001",
                "data": {
                    "dataModelUpdate": {
                        "surfaceId": "main",
                        "path": "/order",
                        "contents": [
                            {
                                "key": "items",
                                "valueArray": [
                                    {
                                        "valueMap": [
                                            {"key": "name", "valueString": "🎧 Premium Headphones"},
                                            {"key": "qty", "valueString": "x1"},
                                            {"key": "price", "valueString": "$99.99"}
                                        ]
                                    },
                                    {
                                        "valueMap": [
                                            {"key": "name", "valueString": "🖱️ Wireless Mouse"},
                                            {"key": "qty", "valueString": "x2"},
                                            {"key": "price", "valueString": "$79.98"}
                                        ]
                                    },
                                    {
                                        "valueMap": [
                                            {"key": "name", "valueString": "⌨️ Mechanical Keyboard"},
                                            {"key": "qty", "valueString": "x1"},
                                            {"key": "price", "valueString": "$129.99"}
                                        ]
                                    }
                                ]
                            }
                        ]
                    }
                }
            }
        }),

        // 9. Payment methods data
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "event",
                "taskId": "task-pay-001",
                "data": {
                    "dataModelUpdate": {
                        "surfaceId": "main",
                        "path": "/payment",
                        "contents": [
                            {"key": "creditCard", "valueBoolean": true},
                            {"key": "paypal", "valueBoolean": false},
                            {"key": "alipay", "valueBoolean": false},
                            {"key": "wechat", "valueBoolean": false}
                        ]
                    }
                }
            }
        }),

        // 10. Summary data
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "event",
                "taskId": "task-pay-001",
                "data": {
                    "dataModelUpdate": {
                        "surfaceId": "main",
                        "path": "/summary",
                        "contents": [
                            {"key": "subtotal", "valueString": "$309.96"},
                            {"key": "shipping", "valueString": "$9.99"},
                            {"key": "tax", "valueString": "$24.80"},
                            {"key": "total", "valueString": "$344.75"}
                        ]
                    }
                }
            }
        }),

        // 11. Task completed
        serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "kind": "task",
                "id": "task-pay-001",
                "contextId": "ctx-pay",
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
            info!("Received request: {}...", &body_str[..body_str.len().min(100)]);

            // Build SSE response body
            let messages = sample_messages();
            let mut sse_body = String::new();

            for (i, msg) in messages.iter().enumerate() {
                let data = serde_json::to_string(msg).unwrap();
                sse_body.push_str(&format!("data: {}\n\n", data));
                info!("Queued message {}/{}", i + 1, messages.len());
            }

            info!("Stream complete - {} messages sent", messages.len());

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
    env_logger::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await?;

    println!("===========================================");
    println!("  Mock A2A Server - Payment Page Demo");
    println!("===========================================");
    println!("Listening on http://{}/rpc", addr);
    println!("Press Ctrl+C to stop");
    println!();

    loop {
        let (stream, remote_addr) = listener.accept().await?;
        info!("Connection from {}", remote_addr);

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_request))
                .await
            {
                error!("Connection error: {:?}", err);
            }
        });
    }
}
