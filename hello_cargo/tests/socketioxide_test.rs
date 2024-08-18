#[cfg(test)]
mod tests {
    // 一个 Socket.IO 服务器用Rust实现 https://mp.weixin.qq.com/s/yt1clZwd1uWWM5CCuW7P7g
    use axum::routing::get;
    use socketioxide::{extract::SocketRef, SocketIo};
    use tokio;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let (layer, io) = SocketIo::new_layer();

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

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();

        Ok(())
    }

    #[test]
    fn test_socketioxide() {
        let (layer, io) = SocketIo::new_layer();

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

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app).await.unwrap();

        Ok(())
    }
}
