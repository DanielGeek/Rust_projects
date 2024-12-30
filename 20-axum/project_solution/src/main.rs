use dotenvy::dotenv;
use std::env;
use project_solution::{app_state::AppState, run, utilities::token_wrapper::TokenWrapper};
use sea_orm::Database;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").unwrap();
    let jwt_secret = env::var("JWT_SECRET").unwrap();
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };

    let app_state = AppState {
        db,
        jwt_secret: TokenWrapper(jwt_secret),
    };

    run(app_state).await;
}
