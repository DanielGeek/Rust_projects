use dotenvy::dotenv;
use std::env;
use project_solution::run;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").unwrap();
    run(database_url).await;
}
