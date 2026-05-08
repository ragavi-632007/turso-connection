use std::net::TcpListener;
use std::io::{Read, Write};
use std::env;

mod db;

fn main() {
    // Load .env manually
    load_env();

    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).expect("Failed to bind to address");
    println!("🚀 Server running at http://{}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0u8; 4096];
                let n = stream.read(&mut buffer).unwrap_or(0);
                let request = String::from_utf8_lossy(&buffer[..n]);

                // Simple routing
                let path = get_path(&request);

                let (status, content_type, body) = match path.as_str() {
                    "/" => {
                        let html = include_str!("../frontend/index.html");
                        ("200 OK", "text/html; charset=utf-8", html.to_string())
                    }
                    "/api/hello" => {
                        // Fetch message from Turso DB
                        let message = db::get_hello_message();
                        let json = format!(r#"{{"message": "{}"}}"#, message);
                        ("200 OK", "application/json", json)
                    }
                    _ => {
                        ("404 Not Found", "text/plain", "Not Found".to_string())
                    }
                };

                let response = format!(
                    "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
                    status,
                    content_type,
                    body.len(),
                    body
                );

                stream.write_all(response.as_bytes()).ok();
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
}

fn get_path(request: &str) -> String {
    let first_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() >= 2 {
        parts[1].to_string()
    } else {
        "/".to_string()
    }
}

fn load_env() {
    if let Ok(content) = std::fs::read_to_string(".env") {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, val)) = line.split_once('=') {
                let val = val.trim_matches('"').trim_matches('\'');
                env::set_var(key.trim(), val.trim());
            }
        }
    }
}
