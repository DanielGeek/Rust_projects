use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};

use crate::{
    database::{
        tasks::{self, Model as TaskModel},
        users::Model as UserModel,
    },
    routes::tasks::create_task_extractor::ValidateCreateTask,
    utilities::app_error::AppError,
};

pub async fn create_task(
    task: ValidateCreateTask,
    user: &UserModel,
    db: &DatabaseConnection,
) -> Result<TaskModel, AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title.unwrap()),
        description: Set(task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    let task = new_task
        .save(db)
        .await
        .map_err(|error| {
            eprintln!("Error creatiing new task: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating task")
        })?;

        convert_active_to_model(task)
}

fn convert_active_to_model(active_task: tasks::ActiveModel) -> Result<TaskModel, AppError> {
    active_task.try_into_model().map_err(|error| {
        eprintln!("Error converting task active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
