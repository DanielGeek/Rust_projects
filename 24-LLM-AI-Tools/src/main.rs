use ollama_rs::{coordinator::Coordinator, generation::{chat::ChatMessage, tools::implementations::Calculator}, Ollama};

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();
    let history = vec![];

    let tools = ollama_rs::tool_group![Calculator {}];

    let mut cordinator = Coordinator::new_with_tools(ollama,
        String::from("llama3.2:1b"),
        history,
        tools);

    let user_message = "2+2*2-4";

    let user_chat_message = ChatMessage::user(user_message.to_owned());

    let resp = cordinator.chat(vec![user_chat_message]).await.unwrap();

    print!("Response: {}", resp.message.content);

    
}
