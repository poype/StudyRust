use axum::{
    extract::{Path, Query},
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/user/:id", get(get_user))
        .route("/items", get(get_items));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to Rust HTTP Server!"
}

async fn hello() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize)]
struct UserParams {
    id: String,
}

async fn get_user(Path(params): Path<UserParams>) -> String {
    format!("User ID: {}", params.id)
}

#[derive(Deserialize)]
struct ItemQuery {
    page: Option<u32>,
    limit: Option<u32>,
}

#[derive(Serialize)]
struct Item {
    id: u32,
    name: String,
}

async fn get_items(Query(query): Query<ItemQuery>) -> Json<Vec<Item>> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    
    let items: Vec<Item> = (1..=limit)
        .map(|i| Item {
            id: (page - 1) * limit + i,
            name: format!("Item {}", (page - 1) * limit + i),
        })
        .collect();
    
    Json(items)
}
