// ============================================================================
// LLM API Types
// ============================================================================

#[derive(Debug, Deserialize)]
struct LlmResponse {
    choices: Vec<LlmChoice>,
}

#[derive(Debug, Deserialize)]
struct LlmChoice {
    message: LlmMessage,
}

#[derive(Debug, Deserialize)]
struct LlmMessage {
    content: Option<String>,
    reasoning_content: Option<String>,
    tool_calls: Option<Vec<LlmToolCall>>,
}

#[derive(Debug, Deserialize)]
struct LlmToolCall {
    id: String,
    function: LlmFunctionCall,
}

#[derive(Debug, Deserialize)]
struct LlmFunctionCall {
    name: String,
    arguments: String,
}

