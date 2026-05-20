use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ChatMessage {
    content: String,
    // Using Option<String> because older models or non-reasoning models won't output this
    reasoning_content: Option<String>, 
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct LMStudioResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ColorPayload {
    hex: String,
}

#[tauri::command]
async fn fetch_color_from_llm(prompt: String) -> Result<String, String> {
    let client = Client::new();
    
    let payload = serde_json::json!({
        "model": "qwen/qwen3.6-27b", 
        "messages": [
            {
                "role": "system",
                "content": "You are a color assistant. Return a valid Hex color code matching the schema."
            },
            {
                "role": "user",
                "content": format!("Hex code for: {}", prompt)
            }
        ],
        "temperature": 0.2,
        "response_format": {
            "type": "json_schema",
            "json_schema": {
                "name": "color_payload",
                "strict": true,
                "schema": {
                    "type": "object",
                    "properties": {
                        "hex": { "type": "string" }
                    },
                    "required": ["hex"],
                    "additionalProperties": false
                }
            }
        }
    });

    let raw_response = client.post("http://localhost:1234/v1/chat/completions")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?
        .text() 
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    let parsed_res: LMStudioResponse = serde_json::from_str(&raw_response)
        .map_err(|_| format!("Format mismatch. Raw response was: {}", raw_response))?;

    let message = &parsed_res.choices.first()
        .ok_or("No choices returned from model")?.message;

    // 2. CLEVER FALLBACK: If 'content' is empty, look inside 'reasoning_content'
    let mut target_json = message.content.trim();
    if target_json.is_empty() {
        if let Some(ref reasoning) = message.reasoning_content {
            target_json = reasoning.trim();
        }
    }

    if target_json.is_empty() {
        return Err("Model returned an entirely empty response object.".to_string());
    }

    // 3. Final verification parse
    let color_data: ColorPayload = serde_json::from_str(target_json)
        .map_err(|_| format!("Failed to extract schema from text block: '{}'", target_json))?;

    Ok(color_data.hex)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch_color_from_llm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
