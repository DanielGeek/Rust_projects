use crate::chain::Chain;
use anyhow::Error;
use async_trait::async_trait;
use dotenv::dotenv;
use ollama_rs::Ollama;
use sqlx::Row;
use sqlx::postgres::PgPool;
use std::env;

pub struct TextToSqlChain {
    client: Ollama,
    db: PgPool,
}

#[async_trait]
impl Chain for TextToSqlChain {
    async fn initialize() -> Result<Box<dyn Chain + Send>, Error>
    where
        Self: Sized,
    {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&db_url)
            .await
            .expect("Failed to connect to database");

        Ok(Box::new(TextToSqlChain {
            client: Ollama::default(),
            db: pool,
        }))
    }

    async fn run(&self, input: String) -> Result<String, Error> {
        let result = self.get_db_info().await.unwrap();
        Ok(result)
    }
}

impl TextToSqlChain {
    async fn get_db_info(&self) -> Result<String, Error> {
        let tables_query =
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'";
        let rows = sqlx::query(tables_query).fetch_all(&self.db).await?;

        let mut tables_info = Vec::new();

        for row in rows {
            let table_name: String = row.get("table_name");

            let columns_query = format!(
                "SELECT column_name FROM information_schema.columns WHERE table_name = '{}'",
                table_name
            );

            let columns_rows = sqlx::query(&columns_query).fetch_all(&self.db).await?;

            let columns: Vec<String> = columns_rows
                .iter()
                .map(|col_row| col_row.get("column_name"))
                .collect();

            tables_info.push(format!("{}: {}", table_name, columns.join(", ")));
        }

        Ok(tables_info.join("\n"))
    }
}
