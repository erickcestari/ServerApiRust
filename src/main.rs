use std::{collections::HashMap, sync::Arc};

use axum::{
    routing::{get, post},
    Router, response::IntoResponse, http::StatusCode, extract::{State, Path}, Json,
};
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use uuid::Uuid;
use time::Date;

time::serde::format_description!(
    date_format, 
    Date,
    "[year]-[month]-[day]"
);

#[derive(Clone, Serialize)]
struct User {
    pub id: Uuid,
    pub name: String,
    pub nick: String,
    #[serde(with = "date_format")]
    pub birth_data: Date,
    pub stack: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
struct NewUser {
    pub name: String,
    pub nick: String,
    #[serde(with = "date_format")]
    pub birth_data: Date,
    pub stack: Option<Vec<String>>,
}

type AppState = Arc<RwLock<HashMap<Uuid, User>>>;

#[tokio::main]
async fn main() {
    let users: HashMap<Uuid, User> = HashMap::new();
    let app_state = Arc::new(RwLock::new(users));

    let app = Router::new()
        .route("/user", get(search_user))
        .route("/user/:id", get(find_user))
        .route("/user", post(create_user))
        .route("/count-user", get(count_user))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn search_user(
    State(users): State<AppState>, 
) -> impl IntoResponse {
    (StatusCode::OK, Json(users.read().await.values().cloned().collect::<Vec<User>>()))
}

async fn find_user(
    State(users): State<AppState>, 
    Path(user_id): Path<Uuid>) -> impl IntoResponse 
    {
    match users.read().await.get(&user_id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err(StatusCode::NOT_FOUND)
    }
}

async fn create_user(
    State(users): State<AppState>, 
    Json(new_person): Json<NewUser>) -> impl IntoResponse 
    {
    let user = User {
        id: Uuid::now_v7(),
        name: new_person.name,
        nick: new_person.nick,
        birth_data: new_person.birth_data,
        stack: new_person.stack,
    };

    users.write().await.insert(user.id, user.clone());
    (StatusCode::CREATED, Json(user))
}

async fn count_user(
    State(users): State<AppState>
) -> impl IntoResponse {
    let count = users.read().await.len();
    (StatusCode::OK, Json(count))
}