use reqwest::Client;
use serde::Deserialize;

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
struct PalettePayload {
    colors: Vec<String>,
}

#[tauri::command]
async fn fetch_palette_from_llm(prompt: String) -> Result<Vec<String>, String> {
    let client = Client::new();

    let model = std::env::var("LMSTUDIO_MODEL")
        .unwrap_or_else(|_| "qwen/qwen3.6-27b".to_string());

    let payload = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "system",
                "content": "You are a color palette assistant. Return a cohesive palette of 5 hex color codes matching the description. The colors should work well together (e.g., primary, secondary, accent, light, dark variants)."
            },
            {
                "role": "user",
                "content": format!("Generate a color palette for: {}", prompt)
            }
        ],
        "temperature": 0.7,
        "response_format": {
            "type": "json_schema",
            "json_schema": {
                "name": "palette_payload",
                "strict": true,
                "schema": {
                    "type": "object",
                    "properties": {
                        "colors": {
                            "type": "array",
                            "items": { "type": "string" }
                        }
                    },
                    "required": ["colors"],
                    "additionalProperties": false
                }
            }
        }
    });

    let api_url = std::env::var("LMSTUDIO_API_URL")
        .unwrap_or_else(|_| "http://localhost:1234/v1/chat/completions".to_string());

    let raw_response = client.post(&api_url)
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

    let mut target_json = message.content.trim();
    if target_json.is_empty() {
        if let Some(ref reasoning) = message.reasoning_content {
            target_json = reasoning.trim();
        }
    }

    if target_json.is_empty() {
        return Err("Model returned an entirely empty response object.".to_string());
    }

    let palette_data: PalettePayload = serde_json::from_str(target_json)
        .map_err(|_| format!("Failed to extract schema from text block: '{}'", target_json))?;

    Ok(palette_data.colors)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch_palette_from_llm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
