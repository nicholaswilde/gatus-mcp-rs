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
                "name": "manage_resources",
                "description": "Discover and manage Gatus resources and instance state.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["list-services", "list-groups", "list-endpoints", "get-config", "get-health"],
                            "description": "Action to perform."
                        },
                        "id": {
                            "type": "string",
                            "description": "Optional identifier (e.g., group name for list-endpoints)."
                        }
                    },
                    "required": ["action"]
                }
            }),
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
            json!({
                "name": "get_group_summary",
                "description": "Get the health status of all endpoints within a specific group.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "group": {
                            "type": "string",
                            "description": "The name of the group to summarize (e.g. 'Media')"
                        }
                    },
                    "required": ["group"]
                }
            }),
            json!({
                "name": "get_uptime",
                "description": "Get the uptime percentage for a specific service over a given timeframe.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "service": {
                            "type": "string",
                            "description": "Name of the service."
                        },
                        "timeframe": {
                            "type": "string",
                            "enum": ["24h", "7d", "30d"],
                            "description": "Timeframe for uptime calculation (default: 24h)",
                            "default": "24h"
                        }
                    },
                    "required": ["service"]
                }
            }),
            json!({
                "name": "get_alert_history",
                "description": "Retrieve recent alert events and state transitions.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "limit": {
                            "type": "integer",
                            "description": "Maximum number of alerts to return (default: 5)",
                            "default": 5
                        }
                    }
                }
            }),
            json!({
                "name": "get_endpoint_stats",
                "description": "Retrieve detailed uptime and response time statistics for a specific endpoint key.",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "key": {
                            "type": "string",
                            "description": "The endpoint key (e.g. 'Core_Frontend'). Use 'manage_services' with 'list' to find names/groups if needed."
                        },
                        "type": {
                            "type": "string",
                            "enum": ["uptime", "response-time"],
                            "description": "The type of statistics to retrieve."
                        },
                        "duration": {
                            "type": "string",
                            "enum": ["1h", "24h", "7d", "30d"],
                            "description": "The duration for the statistics (default: 24h)",
                            "default": "24h"
                        }
                    },
                    "required": ["key", "type"]
                }
            }),
            json!({
                "name": "get_instance_health",
                "description": "Check the health of the Gatus instance itself.",
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
            "manage_resources" => self.handle_manage_resources_tool(id, arguments).await,
            "manage_services" => self.handle_manage_services_tool(id, arguments).await,
            "get_service_info" => self.handle_get_service_info_tool(id, arguments).await,
            "get_system_stats" => self.handle_get_system_stats_tool(id, arguments).await,
            "get_config" => self.handle_get_config_tool(id, arguments).await,
            "get_group_summary" => self.handle_get_group_summary_tool(id, arguments).await,
            "get_uptime" => self.handle_get_uptime_tool(id, arguments).await,
            "get_alert_history" => self.handle_get_alert_history_tool(id, arguments).await,
            "get_endpoint_stats" => self.handle_get_endpoint_stats_tool(id, arguments).await,
            "get_instance_health" => self.handle_get_instance_health_tool(id, arguments).await,
            _ => self.error_response(id, -32601, "Tool not found"),
        }
    }

    async fn handle_manage_resources_tool(&self, id: Value, arguments: &Value) -> Value {
        let action = match arguments.get("action").and_then(|a| a.as_str()) {
            Some(a) => a,
            None => return self.error_response(id, -32602, "Missing 'action' argument"),
        };

        match action {
            "list-services" => {
                match self.gatus_client.list_services().await {
                    Ok(services) => {
                        let text = format_endpoints_summary(&services);
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
            "list-groups" => {
                match self.gatus_client.list_services().await {
                    Ok(services) => {
                        let mut groups: Vec<_> = services.into_iter().map(|s| s.group).collect();
                        groups.sort();
                        groups.dedup();
                        let text = format!("Available groups:\n- {}", groups.join("\n- "));

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
            "list-endpoints" => {
                let group_filter = arguments.get("id").and_then(|g| g.as_str());
                match self.gatus_client.list_services().await {
                    Ok(services) => {
                        let endpoints: Vec<String> = services
                            .into_iter()
                            .filter(|s| {
                                group_filter.map_or(true, |g| s.group.eq_ignore_ascii_case(g))
                            })
                            .map(|s| s.name)
                            .collect();

                        let text = format!("Available endpoints:\n- {}", endpoints.join("\n- "));
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
            "get-config" => self.handle_get_config_tool(id, arguments).await,
            "get-health" => self.handle_get_instance_health_tool(id, arguments).await,
            _ => self.error_response(id, -32602, &format!("Unknown action '{}' for manage_resources", action)),
        }
    }

    async fn handle_get_alert_history_tool(&self, id: Value, arguments: &Value) -> Value {
        let limit = arguments.get("limit").and_then(|l| l.as_u64()).unwrap_or(5) as usize;

        match self.gatus_client.get_alert_history(limit).await {
            Ok(events) => {
                let mut text = String::from(
                    "| Service | Group | Event | Timestamp |\n| :--- | :--- | :--- | :--- |\n",
                );
                for event in events {
                    text.push_str(&format!(
                        "| {} | {} | {} | {} |\n",
                        event.service, event.group, event.event_type, event.timestamp
                    ));
                }

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
            Err(e) => {
                self.error_response(id, -32000, &format!("Error getting alert history: {}", e))
            }
        }
    }

    async fn handle_get_endpoint_stats_tool(&self, id: Value, arguments: &Value) -> Value {
        let key = match arguments.get("key").and_then(|k| k.as_str()) {
            Some(k) => k,
            None => return self.error_response(id, -32602, "Missing 'key' argument"),
        };

        let stat_type = match arguments.get("type").and_then(|t| t.as_str()) {
            Some(t) => t,
            None => return self.error_response(id, -32602, "Missing 'type' argument"),
        };

        let duration = arguments
            .get("duration")
            .and_then(|d| d.as_str())
            .unwrap_or("24h");

        match stat_type {
            "uptime" => match self.gatus_client.get_endpoint_uptimes(key, duration).await {
                Ok(uptimes) => {
                    let mut text = format!("Uptime statistics for {} over {}:\n", key, duration);
                    let mut sorted_keys: Vec<_> = uptimes.keys().collect();
                    sorted_keys.sort();
                    for k in sorted_keys {
                        text.push_str(&format!("- {}: {:.2}%\n", k, uptimes[k] * 100.0));
                    }
                    self.success_response(
                        id,
                        json!({ "content": [{ "type": "text", "text": text }] }),
                    )
                }
                Err(e) => {
                    self.error_response(id, -32000, &format!("Error getting uptime stats: {}", e))
                }
            },
            "response-time" => match self
                .gatus_client
                .get_endpoint_response_times(key, duration)
                .await
            {
                Ok(times) => {
                    if times.is_empty() {
                        return self.success_response(id, json!({ "content": [{ "type": "text", "text": "No response time data found." }] }));
                    }
                    let avg: f64 =
                        times.iter().map(|p| p.value as f64).sum::<f64>() / times.len() as f64;
                    let max = times.iter().map(|p| p.value).max().unwrap_or(0);
                    let min = times.iter().map(|p| p.value).min().unwrap_or(0);
                    let text = format!(
                        "Response time statistics for {} over {}:\n- Average: {:.2}ms\n- Min: {}ms\n- Max: {}ms\n- Data points: {}",
                        key, duration, avg, min, max, times.len()
                    );
                    self.success_response(
                        id,
                        json!({ "content": [{ "type": "text", "text": text }] }),
                    )
                }
                Err(e) => self.error_response(
                    id,
                    -32000,
                    &format!("Error getting response time stats: {}", e),
                ),
            },
            _ => self.error_response(id, -32602, &format!("Unknown stat type '{}'", stat_type)),
        }
    }

    async fn handle_get_instance_health_tool(&self, id: Value, _arguments: &Value) -> Value {
        match self.gatus_client.get_instance_health().await {
            Ok(health) => self.success_response(
                id,
                json!({
                    "content": [
                        {
                            "type": "text",
                            "text": format!("Gatus instance health: {}", health)
                        }
                    ]
                }),
            ),
            Err(e) => {
                self.error_response(id, -32000, &format!("Error getting instance health: {}", e))
            }
        }
    }

    async fn handle_get_uptime_tool(&self, id: Value, arguments: &Value) -> Value {
        let service_name = match arguments.get("service").and_then(|s| s.as_str()) {
            Some(s) => s,
            None => return self.error_response(id, -32602, "Missing 'service' argument"),
        };

        let timeframe = arguments
            .get("timeframe")
            .and_then(|t| t.as_str())
            .unwrap_or("24h");

        match self.gatus_client.get_uptime(service_name, timeframe).await {
            Ok(uptime) => self.success_response(
                id,
                json!({
                    "content": [
                        {
                            "type": "text",
                            "text": format!("Uptime for {} ({}): {:.2}%", service_name, timeframe, uptime)
                        }
                    ]
                }),
            ),
            Err(e) => self.error_response(id, -32000, &format!("Error getting uptime: {}", e)),
        }
    }

    async fn handle_get_group_summary_tool(&self, id: Value, arguments: &Value) -> Value {
        let group_name = match arguments.get("group").and_then(|g| g.as_str()) {
            Some(g) => g,
            None => return self.error_response(id, -32602, "Missing 'group' argument"),
        };

        match self.gatus_client.list_services().await {
            Ok(services) => {
                let filtered: Vec<_> = services
                    .into_iter()
                    .filter(|s| s.group.to_lowercase() == group_name.to_lowercase())
                    .collect();

                if filtered.is_empty() {
                    return self.error_response(
                        id,
                        -32602,
                        &format!("Group '{}' not found or has no services", group_name),
                    );
                }

                self.success_response(
                    id,
                    json!({
                        "content": [
                            {
                                "type": "text",
                                "text": format_endpoints_summary(&filtered)
                            }
                        ]
                    }),
                )
            }
            Err(e) => self.error_response(id, -32000, &format!("Gatus API error: {}", e)),
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
