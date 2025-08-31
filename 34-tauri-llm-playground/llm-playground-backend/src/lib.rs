// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use futures_util::StreamExt;
use ollama_rs::{
    Ollama,
    generation::chat::{ChatMessage, request::ChatMessageRequest},
};
use serde::{Deserialize, Serialize};
use tauri::{State, ipc::Channel};
use tokio::sync::Mutex;

struct AppState {
    ollama: Mutex<Ollama>,
}

#[derive(Serialize)]
struct ChatResponse {
    message: String,
}

#[derive(Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[tauri::command]
async fn get_models(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let models = {
        let client = state.ollama.lock().await;
        client
            .list_local_models()
            .await
            .map_err(|e| format!("Failed to list models: {:?}", e))?
    };

    Ok(models.iter().map(|m| m.name.clone()).collect())
}

#[tauri::command]
async fn chat(
    request: ChatRequest,
    on_stream: Channel<ChatResponse>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let client = state.ollama.lock().await;
    let chat_request = ChatMessageRequest::new(request.model, request.messages);

    let mut stream = client
        .send_chat_messages_stream(chat_request)
        .await
        .map_err(|e| format!("Failed to start chat stream: {:?}", e))?;

    while let Some(response) = stream.next().await {
        let response = response.map_err(|e| format!("Stream error: {:?}", e))?;
        let chat_response = ChatResponse {
            message: response.message.content,
        };

        on_stream.send(chat_response).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            ollama: Mutex::new(Ollama::default()),
        })
        .invoke_handler(tauri::generate_handler![get_models, chat])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
