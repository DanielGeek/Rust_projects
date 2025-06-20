use crate::database::tasks;
use crate::database::tasks::Entity as Tasks;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    pub id: Option<i32>,
    #[serde(
        default,                                    // Deserialize as None if the value is not present
        skip_serializing_if = "Option::is_none",    // Do not serialize if the value is None
        with = "::serde_with::rust::double_option" // Deserialize as None if the value is null
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,                                    // Deserialize as None if the value is not present
        skip_serializing_if = "Option::is_none",    // Do not serialize if the value is None
        with = "::serde_with::rust::double_option" // Deserialize as None if the value is null
    )]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,                                    // Deserialize as None if the value is not present
        skip_serializing_if = "Option::is_none",    // Do not serialize if the value is None
        with = "::serde_with::rust::double_option" // Deserialize as None if the value is null
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,                                    // Deserialize as None if the value is not present
        skip_serializing_if = "Option::is_none",    // Do not serialize if the value is None
        with = "::serde_with::rust::double_option" // Deserialize as None if the value is null
    )]
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
}

pub async fn partial_update(
    Path(task_id): Path<i32>,
    State(database): State<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let mut db_task = if let Some(task) = Tasks::find_by_id(task_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        task.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    if let Some(priority) = request_task.priority {
        db_task.priority = Set(priority);
    }

    if let Some(description) = request_task.description {
        db_task.description = Set(description);
    }

    if let Some(title) = request_task.title {
        db_task.title = Set(title);
    }

    if let Some(completed_at) = request_task.completed_at {
        db_task.completed_at = Set(completed_at);
    }

    if let Some(deleted_at) = request_task.deleted_at {
        db_task.deleted_at = Set(deleted_at);
    }

    Tasks::update(db_task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
