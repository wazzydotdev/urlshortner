use axum::{
    Json, Router,
    http::StatusCode,
    response::Html,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
mod db;
use db::*;
#[tokio::main]

async fn main() {
    // build the router
    let app = Router::new()
        .route("/newurl/{url}", get(new_url))
        .route("/url/{id}", get(fetch_url))
        .route("/r/{id}", get(redirect_page));
    // the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    // start the server
    let bind = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(bind, app).await.unwrap();
}

// saves the url and gives back the short link
async fn new_url(axum::extract::Path(url): axum::extract::Path<String>) -> String {
    let result = db::handler("", &url, "addurl").await;

    // if let checks for Some and unwraps the id in one go
    if let Some(id) = result {
        format!("Saved {} at http://127.0.0.1:3000/url/{}", url, id)
    } else {
        "Failed to save url".to_string()
    }
}
// looks up the id and gives back the saved url or a 404 if its not there
async fn fetch_url(axum::extract::Path(id): axum::extract::Path<String>) -> (StatusCode, String) {
    let result = db::handler(&id, "", "pullinfo").await;

    if let Some(url) = result {
        (StatusCode::OK, url)
    } else {
        (StatusCode::NOT_FOUND, format!("No url found for id {}", id))
    }
}

// serves the frontend page which gets baked into the binary at compile time
async fn redirect_page() -> Html<&'static str> {
    Html(include_str!("../../frontend/index.html"))
}
// says hello with the name from the path
async fn hello(axum::extract::Path(name): axum::extract::Path<String>) -> String {
    format!("Hello, {}!", name)
}
// takes json in and sends json back
#[derive(Deserialize)]
struct InputData {
    name: String,
    age: u8,
}
#[derive(Serialize)]
struct ResponseData {
    message: String,
}
