//! A2UI Streaming Demo
//!
//! Run the mock server first:
//!   python3 debug/mock_a2a_server.py
//!
//! Then run this demo:
//!   cargo run --bin a2ui-streaming

fn main() {
    a2ui_demo::streaming_demo::streaming_app_main()
}
