# Tool Mapping for Consolidation

## New Tool: `manage_resources`
**Description**: Discover and manage Gatus resources and instance state.

| Action | Original Tool | Identifier (`id`) | Notes |
| :--- | :--- | :--- | :--- |
| `list-services` | `manage_services(action="list")` | - | Returns a compact summary of all services. |
| `list-groups` | (New) | - | Lists unique group names. |
| `list-endpoints` | (New) | `group` (optional) | Lists all endpoint keys, optionally filtered by group. |
| `get-config` | `get_config` | - | Returns the current monitoring configuration. |
| `get-health` | `get_instance_health` | - | Checks the health of the Gatus instance. |

## New Tool: `get_metrics`
**Description**: Retrieve status, metrics, and history for services and endpoints.

| Action | Original Tool | Identifier (`id`) | Other Args |
| :--- | :--- | :--- | :--- |
| `system-stats` | `get_system_stats` | - | - |
| `service-details` | `get_service_info(action="details")` | Service Name | - |
| `service-history` | `get_service_info(action="history")` | Service Name | `limit` |
| `group-summary` | `get_group_summary` | Group Name | - |
| `uptime` | `get_uptime` OR `get_endpoint_stats(type="uptime")` | Service or Key | `timeframe` |
| `response-time` | `get_endpoint_stats(type="response-time")` | Key | `timeframe` |
| `alert-history` | `get_alert_history` | - | `limit` |

## Parameterization
- `timeframe`: Enum ["1h", "24h", "7d", "30d"] (Default: "24h")
- `limit`: Integer (Default: 5 for alerts, 10 for history)
