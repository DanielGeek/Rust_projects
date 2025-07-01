use dotenv::dotenv;
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use rig::{completion::Prompt, providers::huggingface::ClientBuilder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("HUGGING_FACE_API_KEY").expect("HUGGING_FACE_API_KEY not found");
    let client = ClientBuilder::new(&api_key).build();

    let agent = client
        .agent("deepseek-ai/DeepSeek-R1-Distill-Qwen-32B")
        .preamble("You are an expert in mathematics")
        .tool(Calculator)
        .build();

    let response = agent.prompt("Calculate 2 + 10").await.unwrap();

    println!("{}", response);
}

#[derive(Deserialize)]
struct OperationArgs {
    x: u32,
    y: u32,
}

#[derive(Debug, thiserror::Error)]
#[error("Math Error")]
struct Error;

#[derive(Deserialize, Serialize)]
struct Calculator;

impl Tool for Calculator {
    const NAME: &'static str = "add";
    type Error = Error;
    type Args = OperationArgs;
    type Output = u32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: String::from("add"),
            description: String::from("add x and y together"),
            parameters: json!({
                "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "The first number to add"
                    },
                    "y": {
                        "type": "number",
                        "description": "The second number to add"
                    }
                },
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("Inside the tool");
        let result = args.x + args.y;
        Ok(result)
    }
}
