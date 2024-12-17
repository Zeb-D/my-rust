// Hyper：Rust网络开发的速度与激情 - 构建高性能Web服务的终极利器
// Hyper的主要特性异步I/O：基于Tokio运行时，支持高效的异步I/O操作。HTTP/1和HTTP/2支持：全面支持最新的HTTP协议版本。客户端和服务器：同时提供HTTP客户端和服务器功能。可扩展性：易于与其他Rust库集成，如TLS支持。零分配解析：高效的HTTP解析器，最小化内存分配。
// https://github.com/hyperium/hyper/blob/master/examples/echo.rs
// https://hyper.rs/guides/1/server/echo/
#[cfg(test)]
mod tests {
    // 基本的HTTP服务器
    #[warn(unused_imports)]
    use std::convert::Infallible;
    use std::net::SocketAddr;

    use futures::TryFutureExt;
    use http_body_util::Empty;
    use http_body_util::Full;
    use http_body_util::{combinators::BoxBody, BodyExt};
    use hyper::body::Bytes;
    use hyper::body::Frame;
    use hyper::server::conn::http1;
    use hyper::service::service_fn;
    use hyper::{Method, StatusCode};
    use hyper::{Request, Response};
    use hyper_util::rt::TokioIo;
    use rayon::str;
    use tokio::net::TcpListener;

    async fn echo(
        req: Request<hyper::body::Incoming>,
    ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/") => Ok(Response::new(full("Try POSTing data to /"))),
            (&Method::POST, "/echo") => Ok(Response::new(full("Try POSTing data to /echo"))),
        
            // Inside the match from before
            // Yet another route inside our match block...
            (&Method::POST, "/echo/uppercase") => {
                // Map this body's frame to a different type
                let frame_stream = req.into_body().map_frame(|frame| {
                    let frame = if let Ok(data) = frame.into_data() {
                        // Convert every byte in every Data frame to uppercase
                        data.iter()
                            .map(|byte| byte.to_ascii_uppercase())
                            .collect::<Bytes>()
                    } else {
                        Bytes::new()
                    };

                    Frame::data(frame)
                });

                Ok(Response::new(frame_stream.boxed()))
            } // Return 404 Not Found for other routes.
            _ => {
                let mut not_found = Response::new(empty());
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        }
    }

    // We create some utility functions to make Empty and Full bodies
    // fit our broadened Response body type.
    fn empty() -> BoxBody<Bytes, hyper::Error> {
        Empty::<Bytes>::new()
            .map_err(|never| match never {})
            .boxed()
    }
    fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
        Full::new(chunk.into())
            .map_err(|never| match never {})
            .boxed()
    }

    #[tokio::test]
    async fn hyper_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

        let listener = TcpListener::bind(addr).await?;
        println!("Listening on http://{}", addr);
        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(echo))
                    .await
                {
                    println!("Error serving connection: {:?}", err);
                }
            });
        }
    }

    use hyper::http::HeaderValue;
    use hyper::Uri;
    use tokio::io::{self, AsyncWriteExt as _};
    use tokio::net::TcpStream;
    use tokio::runtime::Runtime;

    #[tokio::main]
    async fn fetch_url(url: hyper::Uri) -> Result<(), Box<dyn std::error::Error>> {
        let host = url.host().expect("uri has no host");
        let port = url.port_u16().unwrap_or(80);
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(addr).await?;
        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        let authority = url.authority().unwrap().clone();

        let path = url.path();
        let req = Request::builder()
            .uri(path)
            .header(hyper::header::HOST, authority.as_str())
            .body(Empty::<Bytes>::new())?;

        let mut res = sender.send_request(req).await?;

        println!("--Response: {}", res.status());
        println!("Headers: {:#?}\n", res.headers());

        // Stream the body, writing each chunk to stdout as we get it
        // (instead of buffering and printing at the end).
        while let Some(next) = res.frame().await {
            let frame = next?;
            if let Some(chunk) = frame.data_ref() {
                io::stdout().write_all(chunk).await?;
            }
        }

        println!("\n\nDone!");

        Ok(())
    }
    #[test]
    fn test_fetch_url() {
        let s = "http://httpbin.org/post";
        let uri: Uri = s.parse().unwrap();
        fetch_url(uri);

        println!("-------");
        let s = "http://127.0.0.1:8000/112/idx";
        let uri: Uri = s.parse().unwrap();
        fetch_url(uri);
    }
}
