#[cfg(test)]
mod tests {
    // https://google.github.io/comprehensive-rust/concurrency/async-exercises/solutions.html

    #[test]
    fn test_blocking_executor() {
        use futures::future::join_all;
        use std::time::Instant;

        async fn sleep_ms(start: &Instant, id: u64, duration_ms: u64) {
            std::thread::sleep(std::time::Duration::from_millis(duration_ms));
            println!(
                "future {id} slept for {duration_ms}ms, finished after {}ms",
                start.elapsed().as_millis()
            );
        }

        #[tokio::main(flavor = "current_thread")]
        async fn main() {
            let start = Instant::now();
            let sleep_futures = (1..=10).map(|t| sleep_ms(&start, t, t * 10));
            join_all(sleep_futures).await;
        }

        main()
    }

    #[tokio::test]
    async fn test_tokio_select() {
        use tokio::sync::mpsc;
        use tokio::time::{Duration, sleep};

        let (tx, mut rx) = mpsc::channel(32);
        let listener = tokio::spawn(async move {
            tokio::select! {
                Some(msg) = rx.recv() => println!("got: {msg}"),
                _ = sleep(Duration::from_millis(50)) => println!("timeout"),
            };
        });
        sleep(Duration::from_millis(10)).await;
        tx.send(String::from("Hello!"))
            .await
            .expect("Failed to send greeting");

        listener.await.expect("Listener failed");
    }

    #[tokio::test]
    async fn test_tokio_join() {
        use anyhow::Result;
        use futures::future;
        use reqwest;
        use std::collections::HashMap;

        async fn size_of_page(url: &str) -> Result<usize> {
            let resp = reqwest::get(url).await?;
            Ok(resp.text().await?.len())
        }

        let urls: [&str; 4] = [
            "https://google.com",
            "https://httpbin.org/ip",
            "https://play.rust-lang.org/",
            "BAD_URL",
        ];
        let futures_iter = urls.into_iter().map(size_of_page);
        let results = future::join_all(futures_iter).await;
        let page_sizes_dict: HashMap<&str, Result<usize>> =
            urls.into_iter().zip(results.into_iter()).collect();
        println!("{page_sizes_dict:?}");
    }

    #[test]
    fn test_async_await() {
        use futures::executor::block_on;

        async fn count_to(count: i32) {
            for i in 0..count {
                println!("Count is: {i}!");
            }
        }

        async fn async_main(count: i32) {
            count_to(count).await;
        }

        block_on(async_main(10));
    }
}
