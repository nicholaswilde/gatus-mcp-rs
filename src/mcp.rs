use serde_json::{json, Value};

pub struct McpHandler {
    // We'll add state here as needed (e.g., Gatus client)
}

impl McpHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn handle(&self, request: Value) -> Value {
        let id = request.get("id").cloned().unwrap_or(Value::Null);
        let method = match request.get("method").and_then(|m| m.as_str()) {
            Some(m) => m,
            None => return self.error_response(id, -32600, "Invalid Request"),
        };

        match method {
            "tools/list" => self.handle_list_tools(id).await,
            "tools/call" => self.handle_call_tool(id, request.get("params")).await,
            _ => self.error_response(id, -32601, "Method not found"),
        }
    }

    async fn handle_list_tools(&self, id: Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "result": {
                "tools": []
            },
            "id": id
        })
    }

    async fn handle_call_tool(&self, id: Value, params: Option<&Value>) -> Value {
        let name = match params.and_then(|p| p.get("name")).and_then(|n| n.as_str()) {
            Some(n) => n,
            None => return self.error_response(id, -32602, "Invalid params"),
        };

        match name {
            // Tools will be implemented in Phase 3
            _ => self.error_response(id, -32601, "Tool not found"),
        }
    }

    fn error_response(&self, id: Value, code: i32, message: &str) -> Value {
        json!({
            "jsonrpc": "2.0",
            "error": {
                "code": code,
                "message": message
            },
            "id": id
        })
    }
}

impl Default for McpHandler {
    fn default() -> Self {
        Self::new()
    }
}
