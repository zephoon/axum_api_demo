use crate::state::AppState;

mod handlers;
mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app = routes::app(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
