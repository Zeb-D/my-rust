// https://mp.weixin.qq.com/s/G8dLbfo2oXScswNvjoBMHQ Rust网络请求神器：reqwest 让你的 HTTP 调用如丝般顺滑！
// reqwest 是一个高级 HTTP 客户端库，专为 Rust 语言设计。它提供了一套简洁易用的 API，使得发送各种 HTTP 请求变得异常简单。
#[cfg(test)]
mod tests {
    use reqwest;
    #[tokio::main]
    async fn main() -> Result<String, Box<dyn std::error::Error>> {
        let resp = reqwest::get("https://www.rust-lang.org")
            .await?
            .text()
            .await?;
        // println!("{}", resp);
        Ok(resp)
    }
    #[test]
    fn test_main() {
        println!("{:?}", main().ok())
    }

    use reqwest::Client;
    use serde_json::json;
    #[tokio::main]
    async fn main1() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        // 2. 发送 POST 请求
        let res = client
            .post("http://httpbin.org/post")
            // .json(&json!({            "key": "value"        }))
            .send()
            .await?;
        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());
        let body = res.text().await?;
        println!("Body:\n{}", body);
        Ok(())
    }
    #[test]
    fn test_main1() {
        main1();
    }

    use futures::future::join_all;
    #[tokio::main]
    async fn main_async() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let urls = vec![
            "https://www.rust-lang.org",
            "https://github.com",
            "https://www.wikipedia.org",
        ];
        // 3. 异步并发请求
        let requests = urls.into_iter().map(|url| client.get(url).send());
        let responses = join_all(requests).await;
        for response in responses {
            if let Ok(resp) = response {
                println!("{}: Status {}", resp.url(), resp.status());
            }
        }
        Ok(())
    }
    #[test]
    fn test_main_async() {
        main_async();
    }

    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;
    #[tokio::main]
    async fn main_download() -> Result<(), Box<dyn std::error::Error>> {
        let client: Client = Client::new();
        // 4. 文件下载
        let res = client
            .get("https://www.rust-lang.org/logos/rust-logo-512x512.png")
            .send()
            .await?;
        let mut file = File::create("rust-logo.png").await?;
        let content = res.bytes().await?;
        file.write_all(&content).await?;
        println!("File downloaded successfully!");
        Ok(())
    }
    #[test]
    fn test_main_download() {
        main_download();
    }

    // 5. 自定义请求头
    use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
    #[tokio::main]
    async fn main_custom_header() -> Result<(), Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("My Rust App"));
        let client = Client::builder().default_headers(headers).build()?;
        let res = client
            .get("https://api.github.com/repos/rust-lang/rust")
            .send()
            .await?;
        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());
        Ok(())
    }
    #[test]
    fn test_main_custom_header() {
        main_custom_header();
    }

    // 6. 处理认证
    #[tokio::main]
    async fn main_auth() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let res = client
            .get("https://api.github.com/user")
            .basic_auth("username", Some("password"))
            .send()
            .await?;
        if res.status().is_success() {
            let body = res.text().await?;
            println!("Response: {}", body);
        } else {
            println!("Request failed with status: {}", res.status());
        }
        Ok(())
    }
    #[test]
    fn test_main_auth(){
        main_auth();
    }
}
