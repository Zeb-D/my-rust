// 一个 Socket.IO 服务器用Rust实现 https://mp.weixin.qq.com/s/yt1clZwd1uWWM5CCuW7P7g
// Socketioxide是一个socket.io服务器实现，可作为tower的层或者服务工作。它与tower/tokio/hyper生态系统可以很好的融合在一起。
#[cfg(test)]
mod tests {
    use axum::routing::get;
    use socketioxide::{extract::SocketRef, SocketIo};

    #[tokio::main]
    async fn socketioxide_server() -> Result<(), Box<dyn std::error::Error>> {
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
    fn test_socketioxide_server() {
        println!("{:?}", socketioxide_server().ok());
    }

    use reqwest;
    #[tokio::main]
    async fn socketioxide_client() -> Result<String, Box<dyn std::error::Error>> {
        let resp = reqwest::get("http://0.0.0.0:3000/").await?.text().await?;
        println!("{}", resp);
        Ok(resp)
    }
    #[test]
    fn test_socketioxide_client() {
        println!("{:?}", socketioxide_client().ok())
    }
}
