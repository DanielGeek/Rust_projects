use std::sync::Arc;
use tokio::sync::Mutex;

use rmcp::{
    Error, ServerHandler, ServiceExt,
    model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    tool,
    transport::stdio,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = HelloWorld::new().serve(stdio()).await.inspect_err(|e| {
        eprintln!("{e}");
    })?;

    service.waiting().await;

    Ok(())
}

#[derive(Clone)]
pub struct HelloWorld {
    counter: Arc<Mutex<i32>>,
}

#[tool(tool_box)]
impl HelloWorld {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    #[tool(description = "Increment the counter by 1")]
    async fn increment(&self) -> Result<CallToolResult, Error> {
        let mut counter = self.counter.lock().await;
        *counter += 1;
        Ok(CallToolResult::success(vec![Content::text(
            counter.to_string(),
        )]))
    }

    #[tool(description = "Decrement the counter by 1")]
    async fn decrement(&self) -> Result<CallToolResult, Error> {
        let mut counter = self.counter.lock().await;
        *counter -= 1;
        Ok(CallToolResult::success(vec![Content::text(
            counter.to_string(),
        )]))
    }

    #[tool(description = "Get the current value of the counter")]
    async fn get_value(&self) -> Result<CallToolResult, Error> {
        let counter = self.counter.lock().await;
        Ok(CallToolResult::success(vec![Content::text(
            counter.to_string(),
        )]))
    }

    #[tool(description = "Say something")]
    fn echo(&self) -> Result<CallToolResult, Error> {
        Ok(CallToolResult::success(vec![Content::text(String::from(
            "Hello from your frist MCP server!",
        ))]))
    }
}

#[tool(tool_box)]
impl ServerHandler for HelloWorld {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(String::from(
                "This server provides a counter tool that can increment, decrement and provide values. Also has echo",
            )),
        }
    }
}
