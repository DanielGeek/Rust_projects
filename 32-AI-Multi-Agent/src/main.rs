use ai_multi_agent::run;
use bb_ollama::models::message::Message;

fn main() {
    match run() {
        Ok(Message { content, .. }) => println!("{content}"),
        Err(error) => eprintln!("There was an error using AI Todo :( {error}"),
    }
}
