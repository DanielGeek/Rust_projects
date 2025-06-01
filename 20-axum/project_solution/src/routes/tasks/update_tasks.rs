use crate::{
    database::{tasks::{Entity as Tasks, self}, users::Model},
    utilities::app_error::AppError,
};
use axum::{
    extract::{Path, State}, http::StatusCode, Extension
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};

pub async fn mark_completed(
    Path(task_id): Path<i32>,
    Extension(user): Extension<Model>,
    State(db): State<DatabaseConnection>,
) -> Result<(), AppError> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user.id)))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting task to update it: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "An error happened")
        })?;

    let mut task = if let Some(task) = task {
        task.into_active_model()
    } else {
        return Err(AppError::new(
            StatusCode::NOT_FOUND,
            "not found",
        ));
    };

    let now: chrono::DateTime<Utc> = Utc::now();
    task.completed_at = Set(Some(now.into()));
    task.save(&db).await.map_err(|error| {
            eprintln!("Error marking task as completed: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error while updating completed at",)
    })?;

    Ok(())
} 
