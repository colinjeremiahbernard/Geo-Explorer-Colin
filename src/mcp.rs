use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};
use crate::models::Database;

#[derive(Deserialize)]
#[allow(dead_code)] // Prevents Rust from complaining about fields only used for JSON validation
struct McpRequest {
    jsonrpc: String,
    id: Option<serde_json::Value>,
    method: String,
    params: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct McpResponse {
    jsonrpc: String,
    id: Option<serde_json::Value>,
    result: Option<serde_json::Value>,
}

pub async fn run_mcp_server() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        if let Ok(req) = serde_json::from_str::<McpRequest>(&line) {
            let response = match req.method.as_str() {
                "list_trails" => {
                    let db = Database::load_from_file("data/trails.json")?;
                    let techs: Vec<String> = db.techs.keys().cloned().collect();
                    McpResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id,
                        result: Some(serde_json::json!({ "trails": techs })),
                    }
                }
                _ => McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: Some(serde_json::json!({ "error": "Method not found" })),
                },
            };

            println!("{}", serde_json::to_string(&response)?);
        }
    }

    Ok(())
}