use dotenvy::dotenv;
use std::env;
use data::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_uri = env::var("DATABASE_URL").unwrap();
    run(&database_uri).await;
}
