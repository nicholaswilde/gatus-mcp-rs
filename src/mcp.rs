use crate::client::GatusClient;
use crate::fmt::{
    format_config_summary, format_endpoint_status, format_endpoints_summary, format_system_stats,
};
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

    #[tracing::instrument(skip(self, request))]
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
                "name": "manage_services",
                "description": "Manage and list Gatus monitored services",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["list", "status"],
                            "description": "Action to perform: 'list' (compact summary) or 'status' (detailed endpoint statuses)"
                        }
                    },
                    "required": ["action"]
                }
            }),
            json!({
                "name": "get_service_info",
                "description": "Retrieve detailed information or history for a specific service",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "service": {
                            "type": "string",
                            "description": "Name of the service (e.g. 'Authentik')"
                        },
                        "action": {
                            "type": "string",
                            "enum": ["details", "history"],
                            "description": "Action to perform: 'details' (current status/latest result) or 'history' (recent health check results)"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Maximum number of results for the 'history' action",
                            "default": 10
                        }
                    },
                    "required": ["service", "action"]
                }
            }),
            json!({
                "name": "get_system_stats",
                "description": "Get a high-level summary of all monitored services (total, up, down, degraded)",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            }),
            json!({
                "name": "get_config",
                "description": "Retrieve the current Gatus monitoring configuration summary.",
                "inputSchema": {
                    "type": "object",
                    "properties": {}
                }
            }),
        ]
    }

    #[tracing::instrument(skip(self, id, params))]
    async fn handle_call_tool(&self, id: Value, params: Option<Value>) -> Value {
        let params = params.unwrap_or(Value::Null);
        let name = match params.get("name").and_then(|n| n.as_str()) {
            Some(n) => n,
            None => return self.error_response(id, -32602, "Missing tool name"),
        };

        let arguments = params.get("arguments").unwrap_or(&Value::Null);

        match name {
            "manage_services" => self.handle_manage_services_tool(id, arguments).await,
            "get_service_info" => self.handle_get_service_info_tool(id, arguments).await,
            "get_system_stats" => self.handle_get_system_stats_tool(id, arguments).await,
            "get_config" => self.handle_get_config_tool(id, arguments).await,
            _ => self.error_response(id, -32601, "Tool not found"),
        }
    }

    async fn handle_get_config_tool(&self, id: Value, _arguments: &Value) -> Value {
        match self.gatus_client.list_services().await {
            Ok(services) => self.success_response(
                id,
                json!({
                    "content": [
                        {
                            "type": "text",
                            "text": format_config_summary(&services)
                        }
                    ]
                }),
            ),
            Err(e) => self.error_response(id, -32000, &format!("Gatus API error: {}", e)),
        }
    }

    async fn handle_get_system_stats_tool(&self, id: Value, _arguments: &Value) -> Value {
        match self.gatus_client.get_system_stats().await {
            Ok(stats) => self.success_response(
                id,
                json!({
                    "content": [
                        {
                            "type": "text",
                            "text": format_system_stats(&stats)
                        }
                    ]
                }),
            ),
            Err(e) => self.error_response(id, -32000, &format!("Gatus API error: {}", e)),
        }
    }

    async fn handle_manage_services_tool(&self, id: Value, arguments: &Value) -> Value {
        let action = match arguments.get("action").and_then(|a| a.as_str()) {
            Some(a) => a,
            None => return self.error_response(id, -32602, "Missing 'action' argument"),
        };

        match self.gatus_client.list_services().await {
            Ok(services) => {
                let text = match action {
                    "list" => format_endpoints_summary(&services),
                    "status" => serde_json::to_string_pretty(&services).unwrap(),
                    _ => {
                        return self.error_response(
                            id,
                            -32602,
                            &format!("Unknown action '{}' for manage_services", action),
                        )
                    }
                };

                self.success_response(
                    id,
                    json!({
                        "content": [
                            {
                                "type": "text",
                                "text": text
                            }
                        ]
                    }),
                )
            }
            Err(e) => self.error_response(id, -32000, &format!("Gatus API error: {}", e)),
        }
    }

    async fn handle_get_service_info_tool(&self, id: Value, arguments: &Value) -> Value {
        let service_name = match arguments.get("service").and_then(|s| s.as_str()) {
            Some(s) => s,
            None => return self.error_response(id, -32602, "Missing 'service' argument"),
        };

        let action = match arguments.get("action").and_then(|a| a.as_str()) {
            Some(a) => a,
            None => return self.error_response(id, -32602, "Missing 'action' argument"),
        };

        match self.gatus_client.list_services().await {
            Ok(services) => {
                let service = services.into_iter().find(|s| s.name == service_name);
                match service {
                    Some(s) => match action {
                        "details" => self.success_response(
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
                        "history" => {
                            let limit = arguments
                                .get("limit")
                                .and_then(|l| l.as_u64())
                                .unwrap_or(10) as usize;
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
                        _ => self.error_response(
                            id,
                            -32602,
                            &format!("Unknown action '{}' for get_service_info", action),
                        ),
                    },
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
