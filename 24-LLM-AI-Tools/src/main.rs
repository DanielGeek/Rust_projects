use ollama_rs::{
    Ollama,
    coordinator::Coordinator,
    generation::{chat::ChatMessage, tools::implementations::Calculator},
};

/// Get the weather for a given city.
///
/// * city - City to get the weather for.
#[ollama_rs::function]
async fn get_weather(city: String) -> Result<String, Box<dyn Error + Sync + Send>> {
    print!("Inside tool ");
    Ok(reqwest::get(format!("https://wttr.in/{city}?format=%C+%t"))
        .await?
        .text()
        .await?)
}

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();
    let history = vec![];

    let tools = ollama_rs::tool_group![get_weather];
    // let tools = ollama_rs::tool_group![Calculator {}];

    let mut cordinator =
        Coordinator::new_with_tools(ollama, String::from("llama3.2:latest"), history, tools);

    let user_message = "What's the weather in Zulia?";

    let user_chat_message = ChatMessage::user(user_message.to_owned());

    let resp = cordinator.chat(vec![user_chat_message]).await.unwrap();

    print!("Response: {}", resp.message.content);
}
