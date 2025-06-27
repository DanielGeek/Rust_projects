use dotenv::dotenv;
use ollama_rs::Ollama;
use ollama_rs::coordinator::Coordinator;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::generation::tools::Tool;
use ollama_rs_macros::tool_group;
use reqwest::Client;
use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Default)]
struct StockTool {
    client: Client,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct Params {
    first_ticker: String,
    second_ticker: String,
}

#[derive(Debug, Deserialize)]
pub struct MarketData {
    pub top_gainers: Vec<Ticker>,
}

#[derive(Debug, Deserialize)]
pub struct Ticker {
    pub ticker: String,
    pub price: String,
    pub change_amount: String,
    pub change_percentage: String,
    pub volume: String,
}

impl Tool for StockTool {
    type Params = Params;

    fn name() -> &'static str {
        "Stock Investment Analysis Tool"
    }

    fn description() -> &'static str {
        "Help pick best stock to invest"
    }

    async fn call(
        &mut self,
        parameters: Self::Params,
    ) -> Result<String, Box<dyn Error + Sync + Send + 'static>> {
        println!("In Function Stock Agent Tool");

        dotenv().ok();

        let api_key = env::var("ALPHAVANTAGE_API_KEY").expect("ALPHAVANTAGE_API_KEY not set");

        let response = self
            .client
            .get(format!(
                "https://www.alphavantage.co/query?function=TOP_GAINERS_LOSERS&apikey={}",
                api_key
            ))
            .send()
            .await?;

        let result = response.json::<MarketData>().await?;

        let first_ticker = result
            .top_gainers
            .iter()
            .find(|p| p.ticker == parameters.first_ticker);

        let second_ticker = result
            .top_gainers
            .iter()
            .find(|p| p.ticker == parameters.second_ticker);

        if first_ticker.is_none() && second_ticker.is_none() {
            let first = result.top_gainers.first().unwrap();
            let response = format!(
                "Both are not in top gainers instead you can trade {} with change % {}",
                first.ticker, first.change_percentage
            );
            return Ok(response);
        } else if first_ticker.is_none() {
            let response = format!("Go for {}", second_ticker.unwrap().ticker);
            Ok(response)
        } else if second_ticker.is_none() {
            let response = format!("Go for {}", first_ticker.unwrap().ticker);
            Ok(response)
        } else {
            Ok(String::from("Both are good"))
        }
    }
}

#[tokio::main]
async fn main() {
    let stock_tool = StockTool::default();

    let tools = tool_group![stock_tool];

    let llm = Ollama::default();

    let mut coordinator =
        Coordinator::new_with_tools(llm, String::from("qwq"), vec![], tools).debug(true);

    let _resp = coordinator
        .chat(
            vec![
                ChatMessage::system(
                    String::from(
                        "You are a stock market expert. Use the stock_tool for stock_related queries including comparisions.
                        Only respond to stock market questions, reject others with a polite message."
                )
            ),
            ChatMessage::user(
                String::from(
                    "Compare AAPL and NVDA"
                )
            )
            ]
        ).await.unwrap();
}
