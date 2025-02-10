use axum::{
    routing::{post},
    extract::{Json, Multipart},
    response::IntoResponse,
    Router,
};
use iroh::{
    node::Node,
    p2p::{Config as P2PConfig, Ticket},
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize Iroh node
    let config = P2PConfig::default();
    let node = Node::new(config).await?;
    let node_arc = Arc::new(Mutex::new(node));

    // Setup Axum routes
    let app = Router::new()
        .route("/api/send", post(upload_file))
        .route("/api/receive", post(download_file))
        .layer(axum::AddExtensionLayer::new(node_arc));

    println!("Server running at http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Deserialize)]
struct ReceiveRequest {
    ticket: String,
}

// Upload a file and generate a ticket
async fn upload_file(
    mut multipart: Multipart,
    state: axum::extract::Extension<Arc<Mutex<Node>>>,
) -> impl IntoResponse {
    let node = state.lock().await;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        let blob_id = node.blobs().put(data).await.unwrap();
        let ticket = node.p2p().make_ticket(blob_id).unwrap();
        return Json(serde_json::json!({ "ticket": ticket.to_string() }));
    }

    Json(serde_json::json!({ "error": "No file uploaded" }))
}

// Download a file using a ticket
async fn download_file(
    Json(payload): Json<ReceiveRequest>,
    state: axum::extract::Extension<Arc<Mutex<Node>>>,
) -> impl IntoResponse {
    let node = state.lock().await;
    let ticket: Ticket = payload.ticket.parse().unwrap();

    match node.p2p().get(ticket).await {
        Ok(data) => data,
        Err(_) => Json(serde_json::json!({ "error": "Failed to retrieve file" })),
    }
}
