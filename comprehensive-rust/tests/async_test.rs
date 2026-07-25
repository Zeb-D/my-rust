#[cfg(test)]
mod tests {
    // https://google.github.io/comprehensive-rust/concurrency/async-exercises/solutions.html
    #[tokio::test]
    async fn test_dining_philosophers_async() {
        use std::sync::Arc;
        use tokio::sync::{Mutex, mpsc};
        use tokio::time;

        struct Chopstick;

        struct Philosopher {
            name: String,
            left_chopstick: Arc<Mutex<Chopstick>>,
            right_chopstick: Arc<Mutex<Chopstick>>,
            thoughts: mpsc::Sender<String>,
        }

        impl Philosopher {
            async fn think(&self, idx: i32) {
                self.thoughts
                    .send(format!("Eureka! {} has a new idea! {idx}", &self.name))
                    .await
                    .unwrap();
            }

            async fn eat(&self, idx: i32) {
                // Keep trying until we have both chopsticks
                // Pick up chopsticks...
                let _left_chopstick = self.left_chopstick.lock().await;
                let _right_chopstick = self.right_chopstick.lock().await;

                println!("{} is eating... {idx}", &self.name);
                time::sleep(time::Duration::from_millis(5)).await;

                // The locks are dropped here
            }
        }

        // tokio scheduler doesn't deadlock with 5 philosophers, so have 3.
        static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Lucas"];

        // Create chopsticks
        let mut chopsticks = vec![];
        PHILOSOPHERS
            .iter()
            .for_each(|_| chopsticks.push(Arc::new(Mutex::new(Chopstick))));

        // Create philosophers
        let (philosophers, mut rx) = {
            let mut philosophers = vec![];
            let (tx, rx) = mpsc::channel(10);
            for (i, name) in PHILOSOPHERS.iter().enumerate() {
                let mut left_chopstick = Arc::clone(&chopsticks[i]);
                let mut right_chopstick = Arc::clone(&chopsticks[(i + 1) % PHILOSOPHERS.len()]);
                if i == PHILOSOPHERS.len() - 1 {
                    std::mem::swap(&mut left_chopstick, &mut right_chopstick);
                }
                philosophers.push(Philosopher {
                    name: name.to_string(),
                    left_chopstick,
                    right_chopstick,
                    thoughts: tx.clone(),
                });
            }
            (philosophers, rx)
            // tx is dropped here, so we don't need to explicitly drop it later
        };

        // Make them think and eat
        for phil in philosophers {
            tokio::spawn(async move {
                for i in 0..100 {
                    phil.think(i).await;
                    phil.eat(i).await;
                }
            });
        }

        // Output their thoughts
        while let Some(thought) = rx.recv().await {
            println!("Here is a thought: {thought}");
        }
    }

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
