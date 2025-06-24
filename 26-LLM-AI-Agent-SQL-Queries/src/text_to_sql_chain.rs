use crate::chain::Chain;
use anyhow::Error;
use async_trait::async_trait;
use dotenv::dotenv;
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use sqlx::{Column, Row, postgres::PgPool};
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
        let prompt = self.construct_prompt(input).await?;
        let request = GenerationRequest::new(String::from("llama3.2:latest"), prompt);

        let sql = self.client.generate(request).await.unwrap().response;

        println!("SQL Generated: {:?}", sql);

        let data = self.query(&sql).await.unwrap();

        Ok(data)
    }
}

impl TextToSqlChain {
    async fn construct_prompt(&self, input: String) -> Result<String, Error> {
        let db_info = self.get_db_info().await.expect("Failed to get DB info");

        Ok(format!(
            "Provided this schema: {}\nGenerate executable SQL query that answers the question: {}\nOnly return SQL QUERY",
            db_info, input
        ))
    }

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

            tables_info.push(format!("{}({})", table_name, columns.join(", ")));
        }

        Ok(tables_info.join(", "))
    }

    async fn query(&self, generate_query: &str) -> Result<String, Error> {
        let rows = sqlx::query(generate_query).fetch_all(&self.db).await?;

        let mut result_string = String::new();

        for row in rows {
            let mut row_string = String::new();
            for (index, column) in row.columns().iter().enumerate() {
                let column_name = column.name();
                let value: Option<String> = row.try_get(index).unwrap_or(None);
                row_string.push_str(&format!("{}: {:?}\n", column_name, value));
            }
            if row_string.ends_with(",") {
                row_string.truncate(row_string.len() - 2);
            }

            result_string.push_str(&format!("{{ {} }}", row_string));
        }

        Ok(result_string)
    }
}
