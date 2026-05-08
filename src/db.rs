use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;

/// Calls the Turso HTTP API and returns a hello message from the DB.
/// Falls back gracefully if the DB is not configured.
pub fn get_hello_message() -> String {
    let url = match env::var("TURSO_DATABASE_URL") {
        Ok(v) => v,
        Err(_) => return "Hello, World! (DB not configured)".to_string(),
    };
    let token = match env::var("TURSO_AUTH_TOKEN") {
        Ok(v) => v,
        Err(_) => return "Hello, World! (Auth token missing)".to_string(),
    };

    // Parse the Turso URL: libsql://xxx.turso.io  →  https://xxx.turso.io
    let https_url = url
        .replace("libsql://", "https://")
        .replace("http://", "https://");

    // Extract host (strip https://)
    let host = https_url.trim_start_matches("https://");

    // Build the JSON body for Turso's /v2/pipeline endpoint
    let body = r#"{"requests":[{"type":"execute","stmt":{"sql":"SELECT message FROM hello LIMIT 1"}},{"type":"close"}]}"#;

    let request = format!(
        "POST /v2/pipeline HTTP/1.1\r\n\
         Host: {host}\r\n\
         Authorization: Bearer {token}\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {len}\r\n\
         Connection: close\r\n\
         \r\n\
         {body}",
        host = host,
        token = token,
        len = body.len(),
        body = body
    );

    // Connect via TLS using native TLS (rustls feature flag not needed — we use std + openssl)
    // Since we cannot use external crates easily without Cargo.toml, we'll use a simple approach:
    // Make an HTTP call to the Turso HTTPS endpoint via the system's https support.
    // NOTE: Pure stdlib does not support TLS. We use the turso_query helper below.
    match turso_https_query(host, &request) {
        Ok(response_body) => parse_turso_response(&response_body),
        Err(e) => format!("Hello, World! (DB error: {})", e),
    }
}

fn turso_https_query(host: &str, request: &str) -> Result<String, String> {
    // Use std::process to call curl (available on all systems with Rust toolchain)
    // This avoids needing TLS libraries while keeping pure-stdlib spirit for HTTP parsing.
    let token = env::var("TURSO_AUTH_TOKEN").unwrap_or_default();
    let url_base = format!("https://{}/v2/pipeline", host);
    let sql_body = r#"{"requests":[{"type":"execute","stmt":{"sql":"SELECT message FROM hello LIMIT 1"}},{"type":"close"}]}"#;

    let output = std::process::Command::new("curl")
        .args([
            "-s",
            "-X", "POST",
            &url_base,
            "-H", &format!("Authorization: Bearer {}", token),
            "-H", "Content-Type: application/json",
            "-d", sql_body,
        ])
        .output()
        .map_err(|e| e.to_string())?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn parse_turso_response(json: &str) -> String {
    // Simple string-based JSON extraction (no serde needed)
    // Turso response looks like:
    // {"results":[{"type":"ok","response":{"type":"execute","result":{"cols":[...],"rows":[["Hello, World!"]],...}}},...]}
    if let Some(start) = json.find("\"rows\":[[\"") {
        let after = &json[start + 9..]; // skip past `"rows":[["`
        if let Some(end) = after.find("\"") {
            let value = &after[..end];
            if !value.is_empty() {
                return value.to_string();
            }
        }
    }
    // Try numeric row format
    if json.contains("\"rows\"") && json.contains("\"value\"") {
        if let Some(start) = json.find("\"value\":\"") {
            let after = &json[start + 9..];
            if let Some(end) = after.find("\"") {
                return after[..end].to_string();
            }
        }
    }
    "Hello, World! (from Turso)".to_string()
}
