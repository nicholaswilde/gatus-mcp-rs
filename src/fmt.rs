use crate::client::EndpointStatus;

pub fn format_endpoint_status(endpoint: &EndpointStatus) -> String {
    let mut output = format!("### {}\n", endpoint.name);
    output.push_str(&format!("- **Group:** {}\n", endpoint.group));
    output.push_str(&format!("- **Status:** {}\n", endpoint.status));
    
    if let Some(result) = endpoint.results.first() {
        output.push_str("\n#### Latest Result\n");
        output.push_str(&format!("- **Timestamp:** {}\n", result.timestamp));
        output.push_str(&format!("- **Success:** {}\n", result.success));
        output.push_str(&format!("- **Duration:** {}ms\n", result.duration / 1_000_000));
        
        if !result.errors.is_empty() {
            output.push_str("- **Errors:**\n");
            for error in &result.errors {
                output.push_str(&format!("  - {}\n", error));
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
            Some(r) => if r.success { "✅" } else { "❌" },
            None => "❓",
        };
        output.push_str(&format!("| {} | {} | {} | {} |\n", 
            endpoint.name, 
            endpoint.group, 
            endpoint.status,
            latest
        ));
    }
    
    output
}
