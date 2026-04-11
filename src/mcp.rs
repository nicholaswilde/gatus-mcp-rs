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
                    },
                    {
                        "name": "get_service_status",
                        "description": "Get current status and latest results for a specific service",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "service": {
                                    "type": "string",
                                    "description": "Name of the service (e.g. 'core_mcp')"
                                }
                            },
                            "required": ["service"]
                        }
                    },
                    {
                        "name": "get_service_history",
                        "description": "Get a list of recent health check results for a specific service",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "service": {
                                    "type": "string",
                                    "description": "Name of the service"
                                },
                                "limit": {
                                    "type": "integer",
                                    "description": "Maximum number of results to return",
                                    "default": 10
                                }
                            },
                            "required": ["service"]
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

        let arguments = params.and_then(|p| p.get("arguments")).unwrap_or(&Value::Null);

        match name {
            "list_services" => self.handle_list_services_tool(id).await,
            "get_service_status" => self.handle_get_service_status_tool(id, arguments).await,
            "get_service_history" => self.handle_get_service_history_tool(id, arguments).await,
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

    async fn handle_get_service_status_tool(&self, id: Value, arguments: &Value) -> Value {
        let service_name = match arguments.get("service").and_then(|s| s.as_str()) {
            Some(s) => s,
            None => return self.error_response(id, -32602, "Missing 'service' argument"),
        };

        match self.gatus_client.list_services().await {
            Ok(services) => {
                let service = services.into_iter().find(|s| s.name == service_name);
                match service {
                    Some(s) => {
                        json!({
                            "jsonrpc": "2.0",
                            "result": {
                                "content": [
                                    {
                                        "type": "text",
                                        "text": serde_json::to_string_pretty(&s).unwrap()
                                    }
                                ]
                            },
                            "id": id
                        })
                    }
                    None => self.error_response(id, -32602, &format!("Service '{}' not found", service_name)),
                }
            }
            Err(e) => self.error_response(id, -32000, &format!("Gatus API error: {}", e)),
        }
    }

    async fn handle_get_service_history_tool(&self, id: Value, arguments: &Value) -> Value {
        let service_name = match arguments.get("service").and_then(|s| s.as_str()) {
            Some(s) => s,
            None => return self.error_response(id, -32602, "Missing 'service' argument"),
        };

        let limit = arguments.get("limit").and_then(|l| l.as_u64()).unwrap_or(10) as usize;

        match self.gatus_client.list_services().await {
            Ok(services) => {
                let service = services.into_iter().find(|s| s.name == service_name);
                match service {
                    Some(s) => {
                        let history: Vec<_> = s.results.into_iter().take(limit).collect();
                        json!({
                            "jsonrpc": "2.0",
                            "result": {
                                "content": [
                                    {
                                        "type": "text",
                                        "text": serde_json::to_string_pretty(&history).unwrap()
                                    }
                                ]
                            },
                            "id": id
                        })
                    }
                    None => self.error_response(id, -32602, &format!("Service '{}' not found", service_name)),
                }
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
