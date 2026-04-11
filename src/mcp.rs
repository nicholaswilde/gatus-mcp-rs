use crate::client::GatusClient;
use crate::fmt::{format_endpoint_status, format_endpoints_summary};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

pub const PROTOCOL_VERSION: &str = "2024-11-05";

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

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
        let req: JsonRpcRequest = match serde_json::from_value(request) {
            Ok(r) => r,
            Err(_) => return self.error_response(Value::Null, -32600, "Invalid Request"),
        };

        let id = req.id.unwrap_or(Value::Null);

        match req.method.as_str() {
            "initialize" => self.handle_initialize(id).await,
            "tools/list" => self.handle_list_tools(id).await,
            "tools/call" => self.handle_call_tool(id, req.params).await,
            "notifications/initialized" => json!(null),
            _ => self.error_response(id, -32601, "Method not found"),
        }
    }

    async fn handle_initialize(&self, id: Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "result": {
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "gatus-mcp-rs",
                    "version": env!("CARGO_PKG_VERSION")
                }
            },
            "id": id
        })
    }

    async fn handle_list_tools(&self, id: Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "result": {
                "tools": self.get_tool_definitions()
            },
            "id": id
        })
    }

    fn get_tool_definitions(&self) -> Vec<Value> {
        vec![
            json!({
                "name": "list_services",
                "description": "List all services monitored by Gatus (compact summary)",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            }),
            json!({
                "name": "get_endpoint_statuses",
                "description": "Get detailed statuses for all monitored endpoints",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            }),
            json!({
                "name": "get_service_status",
                "description": "Get current status and latest results for a specific service",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "service": {
                            "type": "string",
                            "description": "Name of the service (e.g. 'Authentik')"
                        }
                    },
                    "required": ["service"]
                }
            }),
            json!({
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
            }),
        ]
    }

    async fn handle_call_tool(&self, id: Value, params: Option<Value>) -> Value {
        let params = params.unwrap_or(Value::Null);
        let name = match params.get("name").and_then(|n| n.as_str()) {
            Some(n) => n,
            None => return self.error_response(id, -32602, "Missing tool name"),
        };

        let arguments = params.get("arguments").unwrap_or(&Value::Null);

        match name {
            "list_services" => self.handle_list_services_tool(id).await,
            "get_endpoint_statuses" => self.handle_get_endpoint_statuses_tool(id).await,
            "get_service_status" => self.handle_get_service_status_tool(id, arguments).await,
            "get_service_history" => self.handle_get_service_history_tool(id, arguments).await,
            _ => self.error_response(id, -32601, "Tool not found"),
        }
    }

    async fn handle_list_services_tool(&self, id: Value) -> Value {
        match self.gatus_client.list_services().await {
            Ok(services) => self.success_response(
                id,
                json!({
                    "content": [
                        {
                            "type": "text",
                            "text": format_endpoints_summary(&services)
                        }
                    ]
                }),
            ),
            Err(e) => self.error_response(id, -32000, &format!("Gatus API error: {}", e)),
        }
    }

    async fn handle_get_endpoint_statuses_tool(&self, id: Value) -> Value {
        match self.gatus_client.list_services().await {
            Ok(services) => self.success_response(
                id,
                json!({
                    "content": [
                        {
                            "type": "text",
                            "text": serde_json::to_string_pretty(&services).unwrap()
                        }
                    ]
                }),
            ),
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
                    Some(s) => self.success_response(
                        id,
                        json!({
                            "content": [
                                {
                                    "type": "text",
                                    "text": format_endpoint_status(&s)
                                }
                            ]
                        }),
                    ),
                    None => self.error_response(
                        id,
                        -32602,
                        &format!("Service '{}' not found", service_name),
                    ),
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

        let limit = arguments
            .get("limit")
            .and_then(|l| l.as_u64())
            .unwrap_or(10) as usize;

        match self.gatus_client.list_services().await {
            Ok(services) => {
                let service = services.into_iter().find(|s| s.name == service_name);
                match service {
                    Some(s) => {
                        let history: Vec<_> = s.results.into_iter().take(limit).collect();
                        self.success_response(
                            id,
                            json!({
                                "content": [
                                    {
                                        "type": "text",
                                        "text": serde_json::to_string_pretty(&history).unwrap()
                                    }
                                ]
                            }),
                        )
                    }
                    None => self.error_response(
                        id,
                        -32602,
                        &format!("Service '{}' not found", service_name),
                    ),
                }
            }
            Err(e) => self.error_response(id, -32000, &format!("Gatus API error: {}", e)),
        }
    }

    fn success_response(&self, id: Value, result: Value) -> Value {
        json!({
            "jsonrpc": "2.0",
            "result": result,
            "id": id
        })
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
