use crate::chain::Chain;
use async_trait::async_trait;
use ollama_rs::Ollama;
use sqlx::postgres::PgPool;

pub struct TextToSqlChain {
    client: Ollama,
    db: PgPool
}

#[async_trait]
impl Chain for TextToSqlChain {
    async fn initialize() -> Result<Box<dyn Chain + Send>, Error>
    where
        Self: Sized
        {
            todo!()
        }

    async fn run(&self, input: String) -> Result<String, Error> {
        todo!()
    }
}
