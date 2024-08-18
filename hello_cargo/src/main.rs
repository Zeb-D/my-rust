use axum::routing::get;
use socketioxide::{extract::SocketRef, SocketIo};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (layer, io) = SocketIo::builder()
        .max_payload(10_000_000) // Max HTTP payload size of 10M
        .max_buffer_size(10_000) // Max number of packets in the buffer
        .build_layer();
    
    // Register a handler for the default namespace
    io.ns("/", |s: SocketRef| {
        // For each "message" event received, send a "message-back" event with the "Hello World!" event
        s.on("message", |s: SocketRef| {
            s.emit("message-back", "Hello World!").ok();
        });
    });

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:30000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
