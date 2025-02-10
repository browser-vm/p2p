use std::sync::Arc;
use axum::{
    extract::{Json, Multipart, Extension},
    routing::post,
    Router,
};
use iroh::{
    Iroh,
    ticket::Ticket,
};
use tokio::sync::Mutex;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let iroh = Iroh::new().await.unwrap();
    let iroh_arc = Arc::new(Mutex::new(iroh));

    let app = Router::new()
        .route("/api/send", post(upload_file))
        .route("/api/receive", post(download_file))
        .layer(ServiceBuilder::new().layer(Extension(iroh_arc)));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn upload_file(
    multipart: Multipart,
    Extension(iroh): Extension<Arc<Mutex<Iroh>>>,
) -> impl axum::response::IntoResponse {
    let iroh = iroh.lock().await;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        let blob_id = iroh.put(data).await.unwrap();
        let ticket = iroh.make_ticket(blob_id).unwrap();
        return Json(serde_json::json!({ "ticket": ticket.to_string() }));
    }

    Json(serde_json::json!({ "error": "No file uploaded" }))
}

async fn download_file(
    Json(request): Json<ReceiveRequest>,
    Extension(iroh): Extension<Arc<Mutex<Iroh>>>,
) -> impl axum::response::IntoResponse {
    let iroh = iroh.lock().await;
    let ticket: Ticket = request.ticket.parse().unwrap();

    match iroh.get(ticket).await {
        Ok(data) => data,
        Err(_) => Json(serde_json::json!({ "error": "Failed to retrieve file" })),
    }
}

struct ReceiveRequest {
    ticket: String,
}
