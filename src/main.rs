use std::{collections::HashMap, sync::{Arc, RwLock}, time::Duration};
use axum::{error_handling::HandleErrorLayer, extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use tower::{BoxError, ServiceBuilder};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let db = Db::default();

    let app = Router::new()
        .route("/todos", get(todos_index))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .into_inner(),
        )
        .with_state(db);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.3002")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn todos_index(
    State(db): State<Db>,
) -> impl IntoResponse {
    let todos = db.read().unwrap();

    let todos = todos
        .values()
        .cloned()
        .collect::<Vec<_>>();

    Json(todos)
}

type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;

#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: Uuid,
    text: String, 
    complete: bool,
}