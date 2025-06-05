use axum::{Extension, Json, extract::State, http::StatusCode};
use sea_orm::DatabaseConnection;

use crate::{
    database::users::Model as UserModel, queries::task_queries, utilities::app_error::AppError,
};

use super::{ResponseDataTask, ResponseTask, create_task_extractor::ValidateCreateTask};

pub async fn create_task(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    task: ValidateCreateTask, // extractor validator for the request body
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    let task = task_queries::create_task(task, &user, &db).await?;
    let response = ResponseTask {
        id: task.id,
        title: task.title,
        description: task.description,
        priority: task.priority,
        completed_at: task.completed_at.map(|time| time.to_string()),
    };

    Ok((
        StatusCode::CREATED,
        Json(ResponseDataTask { data: response }),
    ))
}
