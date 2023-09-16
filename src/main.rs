use std::{collections::HashMap, sync::Arc};

use axum::{
    routing::{get, post},
    Router, response::IntoResponse, http::StatusCode, extract::{State, Path}, Json,
};
use serde::Serialize;
use uuid::Uuid;
use time::{Date, macros::date};

#[derive(Clone, Serialize)]
struct User {
    pub id: Uuid,
    pub name: String,
    pub nick: String,
    pub birth_data: Date,
    pub stack: Vec<String>,
}

type AppState = Arc<HashMap<Uuid, User>>;

#[tokio::main]
async fn main() {
    let mut users: HashMap<Uuid, User> = HashMap::new();

    let user = User {
        id: Uuid::now_v7(),
        name: "John Doe".to_string(),
        nick: "johndoe".to_string(),
        birth_data: date!(1986 - 01 - 01),
        stack: vec!["rust".to_string(), "go".to_string()],
    };

    users.insert(user.id, user);

    let app_state = Arc::new(users);

    let app = Router::new()
        .route("/user", get(search_user))
        .route("/user/:id", get(find_user))
        .route("/user", post(create_user))
        .route("/user-count", get(count_user))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn search_user(State(users): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, "Search user")
}

async fn find_user(
    State(users): State<AppState>, 
    Path(user_id): Path<Uuid>) -> impl IntoResponse 
    {

    match users.get(&user_id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err(StatusCode::NOT_FOUND)
    }
}

async fn create_user() -> impl IntoResponse {
    (StatusCode::CREATED, "Create user")
}

async fn count_user() -> impl IntoResponse {
    (StatusCode::OK, "Count user")
}