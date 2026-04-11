use crate::gatus::GatusClient;
use serde_json::{json, Value};
use std::sync::Arc;

pub struct McpHandler {
    gatus_client: Arc<GatusClient>,
}

impl McpHandler {
    pub fn new(gatus_client: GatusClient) -> Self {
        Self {
            gatus_client: Arc::new(gatus_client),
        }
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
                "tools": [
                    {
                        "name": "list_services",
                        "description": "List all services monitored by Gatus",
                        "inputSchema": {
                            "type": "object",
                            "properties": {}
                        }
                    }
                ]
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
            "list_services" => self.handle_list_services_tool(id).await,
            _ => self.error_response(id, -32601, "Tool not found"),
        }
    }

    async fn handle_list_services_tool(&self, id: Value) -> Value {
        match self.gatus_client.list_services().await {
            Ok(services) => {
                let thinned_services: Vec<Value> = services
                    .into_iter()
                    .map(|s| {
                        json!({
                            "name": s.name,
                            "group": s.group,
                            "status": s.status
                        })
                    })
                    .collect();

                json!({
                    "jsonrpc": "2.0",
                    "result": {
                        "content": [
                            {
                                "type": "text",
                                "text": serde_json::to_string_pretty(&thinned_services).unwrap()
                            }
                        ]
                    },
                    "id": id
                })
            }
            Err(e) => self.error_response(id, -32000, &format!("Gatus API error: {}", e)),
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
