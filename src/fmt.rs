use crate::client::{EndpointStatus, SystemStats};

pub fn format_endpoint_status(endpoint: &EndpointStatus) -> String {
    let mut output = format!("### {}\n", endpoint.name);
    output.push_str(&format!("- **Group:** {}\n", endpoint.group));
    output.push_str(&format!("- **Status:** {}\n", endpoint.display_status()));

    if let Some(result) = endpoint.results.first() {
        output.push_str("\n#### Latest Result\n");
        output.push_str(&format!("- **Timestamp:** {}\n", result.timestamp));
        output.push_str(&format!("- **Success:** {}\n", result.success));
        output.push_str(&format!(
            "- **Duration:** {}ms\n",
            result.duration / 1_000_000
        ));

        if let Some(exp) = result.certificate_expiration {
            let days = exp / (24 * 60 * 60 * 1_000_000_000);
            output.push_str(&format!("- **SSL Expiration:** {} days remaining\n", days));
        }

        if !result.errors.is_empty() {
            output.push_str("- **Errors:**\n");
            for error in &result.errors {
                output.push_str(&format!("  - {}\n", error));
            }
        }

        if !result.success {
            if result.headers.is_some() {
                output.push_str("- **Headers:** (present)\n");
            }
            if let Some(ref body) = result.body {
                let snippet = if body.len() > 100 {
                    format!("{}...", &body[..100])
                } else {
                    body.clone()
                };
                output.push_str(&format!("- **Body Snippet:** {}\n", snippet));
            }
        }
    }

    output
}

pub fn format_endpoints_summary(endpoints: &[EndpointStatus]) -> String {
    let mut output = String::from("| Service | Group | Status | Latest Result |\n");
    output.push_str("| :--- | :--- | :--- | :--- |\n");

    for endpoint in endpoints {
        let latest = match endpoint.results.first() {
            Some(r) => {
                if r.success {
                    "✅"
                } else {
                    "❌"
                }
            }
            None => "❓",
        };
        output.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            endpoint.name,
            endpoint.group,
            endpoint.display_status(),
            latest
        ));
    }

    output
}

pub fn format_system_stats(stats: &SystemStats) -> String {
    let mut output = String::from("### Gatus System Health Summary\n\n");
    output.push_str(&format!("- **Total Endpoints:** {}\n", stats.total));
    output.push_str(&format!("- **UP:** {}\n", stats.up));
    output.push_str(&format!("- **DOWN:** {}\n", stats.down));
    output.push_str(&format!("- **DEGRADED:** {}\n", stats.degraded));
    output.push_str(&format!(
        "- **Certificates Expiring Soon:** {}\n",
        stats.certificates_expiring_soon
    ));
    output
}

pub fn format_config_summary(endpoints: &[EndpointStatus]) -> String {
    let mut output = String::from("### Gatus Endpoint Configurations\n\n");
    for endpoint in endpoints {
        output.push_str(&format!(
            "#### {} (Group: {})\n",
            endpoint.name, endpoint.group
        ));
        if let Some(result) = endpoint.results.first() {
            if !result.condition_results.is_empty() {
                output.push_str("- **Conditions:**\n");
                for condition in &result.condition_results {
                    let status = if condition.success { "✅" } else { "❌" };
                    output.push_str(&format!("  - {} {}\n", condition.condition, status));
                }
            }
        }
        output.push('\n');
    }
    output
}
