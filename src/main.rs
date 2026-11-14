use axum::{
    extract::{Path, Query},
    routing::get, Router};


use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
struct Task{
    id: u32,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
}


fn seed_tasks() -> Vec<Task> {
    vec![
        Task { id: 1, title: "Lean Axum".to_string(), done: false}
    ]
}

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/{id}", get(get_task));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    println!("Server Running on http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str{
    "Server Up"
}

async fn health() -> &'static str{
    "Ok"
}

async fn list_tasks(Query(params): Query<HashMap<String, String>>) -> String{
    let filter = params.get("done").map_or("everything".to_string(), |v| format!("done={}", v));
    format!("Listing tasks, filter: {}", filter)
}

async fn create_task() -> &'static str{
    "Task created"
}

async fn get_task(Path(id): Path<u32>) -> String{
    format!("Task Request - Task ID: {}", id)
}
