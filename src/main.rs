use axum::{
    routing::{get, post},
    Router, Json, extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};


#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u64,
    task: String,
    completed: bool,
}


type SharedState = Arc<Mutex<Vec<Todo>>>;


#[tokio::main]
async fn main() {

    let state: SharedState = Arc::new(Mutex::new(Vec::new()));

   
    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .with_state(state);

    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    
    
    axum::serve(listener, app).await.unwrap();
}


async fn list_todos(State(state): State<SharedState>) -> Json<Vec<Todo>> {
    let todos = state.lock().unwrap().clone();
    Json(todos)
}


async fn create_todo(
    State(state): State<SharedState>, 
    Json(payload): Json<Todo>
) -> Json<Todo> {
    let mut todos = state.lock().unwrap();
    todos.push(payload.clone());
    Json(payload)
}