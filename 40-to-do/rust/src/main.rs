use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoItem {
    id: Uuid,
    title: String,
    completed: bool,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateTodoItem {
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
pub struct UpdateTodoItem {
    title: Option<String>,
    completed: Option<bool>,
}

struct AppState {
    todo_list: Mutex<Vec<TodoItem>>,
}

async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todo_list.lock().unwrap();
    HttpResponse::Ok().json(&*todos)
}

async fn add_todo(
    item: web::Json<CreateTodoItem>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = data.todo_list.lock().unwrap();
    let new_id = Uuid::new_v4();
    let new_todo = TodoItem {
        id: new_id,
        title: item.title.clone(),
        completed: item.completed,
        created_at: Utc::now(),
    };
    todos.push(new_todo);
    HttpResponse::Ok().json(&*todos)
}

fn main() -> std::io::Result<()> {
    let todo_list = vec![];
    let app_state = web::Data::new(AppState {
        todo_list: Mutex::new(todo_list),
    });

    Ok(())
}
