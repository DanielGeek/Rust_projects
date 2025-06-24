use std::io::{self, Write};

use chain::Chain;
use text_to_sql_chain::TextToSqlChain;

mod chain;
mod text_to_sql_chain;

#[tokio::main]
async fn main() {
    let processor = TextToSqlChain::initialize().await.unwrap();
    let mut input = String::new();
    print!("How Can I help you?: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let output = processor.run(input).await.unwrap();

    println!("Response: {:?}", output);
}
