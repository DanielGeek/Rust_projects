use crate::schema::users;
use actix_web::{get, post, web, web::Path, HttpResponse};
use diesel::Insertable;
use log::error;
use serde::Deserialize;

use crate::application::use_cases::get_user::GetUserUseCase;
use crate::application::use_cases::register_user::RegisterUserUseCase;
use crate::infrastructure::repositories::postgres_user_repository::PostgresUserRepository;

#[derive(Debug, Clone, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[post("/")]
pub async fn register_user_handler(
    repo: web::Data<PostgresUserRepository>,
    input: web::Json<NewUser>,
) -> HttpResponse {
    match RegisterUserUseCase::new(repo.into_inner())
        .execute(input.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(ex) => {
            error!("Error registering user! {:?}", ex);
            HttpResponse::InternalServerError().body("Please try again...!")
        }
    }
}

#[get("/{email}")]
pub async fn get_by_email(
    repo: web::Data<PostgresUserRepository>,
    path: Path<(String,)>,
) -> HttpResponse {
    match GetUserUseCase::new(repo.into_inner())
        .get(path.into_inner().0)
        .await
    {
        Some(customer) => HttpResponse::Ok().json(customer),
        None => {
            error!("Error registering user!");
            HttpResponse::InternalServerError().body("Please try again...!")
        }
    }
}