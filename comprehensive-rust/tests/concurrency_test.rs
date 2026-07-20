#[cfg(test)]
mod tests {
    use crossbeam_channel::{Receiver, Sender, unbounded};
    use std::sync::{Arc, Mutex, mpsc};
    use std::thread;
    use std::time::Duration;

    // https://google.github.io/comprehensive-rust/concurrency/sync-exercises/solutions.html
    #[test]
    fn test_link_checker() {
        // 并行网页爬虫，用于检查指定域名下的所有页面中的链接是否有效

        use std::sync::{Arc, Mutex, mpsc};
        use std::thread;

        use reqwest::Url;
        use reqwest::blocking::Client;
        use scraper::{Html, Selector};
        use thiserror::Error;

        #[derive(Error, Debug)]
        enum Error {
            #[error("request error: {0}")]
            ReqwestError(#[from] reqwest::Error),
            #[error("bad http response: {0}")]
            BadResponse(String),
        }

        #[derive(Debug)]
        struct CrawlCommand {
            url: Url,
            extract_links: bool,
        }

        fn visit_page(client: &Client, command: &CrawlCommand) -> Result<Vec<Url>, Error> {
            println!("Checking {:#}", command.url);
            let response = client.get(command.url.clone()).send()?;
            if !response.status().is_success() {
                return Err(Error::BadResponse(response.status().to_string()));
            }

            let mut link_urls = Vec::new();
            if !command.extract_links {
                return Ok(link_urls);
            }

            let base_url = response.url().clone();
            let body_text = response.text()?;
            let document = Html::parse_document(&body_text);

            let selector = Selector::parse("a").unwrap();
            let href_values = document
                .select(&selector)
                .filter_map(|element| element.value().attr("href"));
            for href in href_values {
                match base_url.join(href) {
                    Ok(link_url) => {
                        link_urls.push(link_url);
                    }
                    Err(err) => {
                        println!("On {base_url:#}: ignored unparsable {href:?}: {err}");
                    }
                }
            }
            Ok(link_urls)
        }

        struct CrawlState {
            domain: String,
            visited_pages: std::collections::HashSet<String>,
        }

        impl CrawlState {
            fn new(start_url: &Url) -> CrawlState {
                let mut visited_pages = std::collections::HashSet::new();
                visited_pages.insert(start_url.as_str().to_string());
                CrawlState {
                    domain: start_url.domain().unwrap().to_string(),
                    visited_pages,
                }
            }

            /// Determine whether links within the given page should be extracted.
            fn should_extract_links(&self, url: &Url) -> bool {
                url.domain().is_some_and(|d| d == self.domain)
            }

            /// Mark the given page as visited, returning false if it had already
            /// been visited.
            fn mark_visited(&mut self, url: &Url) -> bool {
                self.visited_pages.insert(url.as_str().to_string())
            }
        }

        type CrawlResult = Result<Vec<Url>, (Url, Error)>;

        fn spawn_crawler_threads(
            command_receiver: crossbeam_channel::Receiver<CrawlCommand>,
            result_sender: crossbeam_channel::Sender<CrawlResult>,
            thread_count: u32,
        ) {
            for _ in 0..thread_count {
                let result_sender = result_sender.clone();
                let command_receiver = command_receiver.clone();
                thread::spawn(move || {
                    let client = Client::builder()
                        .timeout(Duration::from_secs(10))
                        .build()
                        .unwrap();
                    while let Ok(crawl_command) = command_receiver.recv() {
                        let crawl_result = match visit_page(&client, &crawl_command) {
                            Ok(link_urls) => Ok(link_urls),
                            Err(error) => Err((crawl_command.url, error)),
                        };
                        result_sender.send(crawl_result).unwrap();
                    }
                });
            }
        }

        fn control_crawl(
            start_url: Url,
            command_sender: Sender<CrawlCommand>,
            result_receiver: Receiver<CrawlResult>,
        ) -> Vec<Url> {
            let mut crawl_state = CrawlState::new(&start_url);
            let start_command = CrawlCommand {
                url: start_url,
                extract_links: true,
            };
            command_sender.send(start_command).unwrap();
            let mut pending_urls = 1;

            let mut bad_urls = Vec::new();
            while pending_urls > 0 {
                let crawl_result = result_receiver.recv().unwrap();
                pending_urls -= 1;

                match crawl_result {
                    Ok(link_urls) => {
                        for url in link_urls {
                            if crawl_state.mark_visited(&url) {
                                let extract_links = crawl_state.should_extract_links(&url);
                                let crawl_command = CrawlCommand { url, extract_links };
                                command_sender.send(crawl_command).unwrap();
                                pending_urls += 1;
                            }
                        }
                    }
                    Err((url, error)) => {
                        bad_urls.push(url);
                        println!("Got crawling error: {:#}", error);
                    }
                }
            }
            bad_urls
        }

        fn check_links(start_url: Url) -> Vec<Url> {
            let (result_sender, result_receiver) = unbounded();
            let (command_sender, command_receiver) = unbounded();
            spawn_crawler_threads(command_receiver, result_sender, 64);
            control_crawl(start_url, command_sender, result_receiver)
        }

        let start_url = reqwest::Url::parse(
            "https://google.github.io/comprehensive-rust/concurrency/sync-exercises/solutions.html",
        )
        .unwrap();
        let bad_urls = check_links(start_url);
        println!("Bad URLs: {:#?}", bad_urls);
    }

    #[test]
    fn test_five_dining_philosophers() {
        use std::sync::{Arc, Mutex, mpsc};
        use std::thread;
        use std::time::Duration;

        struct Chopstick;

        struct Philosopher {
            name: String,
            left_chopstick: Arc<Mutex<Chopstick>>,
            right_chopstick: Arc<Mutex<Chopstick>>,
            thoughts: mpsc::SyncSender<String>,
        }

        impl Philosopher {
            fn think(&self) {
                self.thoughts
                    .send(format!("Eureka! {} has a new idea!", &self.name))
                    .unwrap();
            }

            fn eat(&self) {
                println!("{} is trying to eat", &self.name);
                let _left = self.left_chopstick.lock().unwrap();
                let _right = self.right_chopstick.lock().unwrap();

                println!("{} is eating...", &self.name);
                thread::sleep(Duration::from_millis(10));
            }
        }

        static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

        let (tx, rx) = mpsc::sync_channel(10);

        let chopsticks = PHILOSOPHERS
            .iter()
            .map(|_| Arc::new(Mutex::new(Chopstick)))
            .collect::<Vec<_>>();

        for i in 0..chopsticks.len() {
            let tx = tx.clone();
            let mut left_chopstick = Arc::clone(&chopsticks[i]);
            let mut right_chopstick = Arc::clone(&chopsticks[(i + 1) % chopsticks.len()]);

            // To avoid a deadlock, we have to break the symmetry
            // somewhere. This will swap the chopsticks without deinitializing
            // either of them.
            if i == chopsticks.len() - 1 {
                std::mem::swap(&mut left_chopstick, &mut right_chopstick);
            }

            let philosopher = Philosopher {
                name: PHILOSOPHERS[i].to_string(),
                thoughts: tx,
                left_chopstick,
                right_chopstick,
            };

            thread::spawn(move || {
                for _ in 0..100 {
                    philosopher.eat();
                    philosopher.think();
                }
            });
        }

        drop(tx);
        for thought in rx {
            println!("{thought}");
        }
    }

    #[test]
    fn test_shared_state() {
        let v = Arc::new(Mutex::new(vec![10, 20, 30]));
        let mut handles = Vec::new();
        for i in 0..5 {
            let v = Arc::clone(&v);
            handles.push(thread::spawn(move || {
                let mut v = v.lock().unwrap();
                v.push(10 * i);
                println!("v: {v:?}");
            }));
        }

        handles.into_iter().for_each(|h| h.join().unwrap());
    }

    #[test]
    fn test_bounded_channel() {
        let (tx, rx) = mpsc::sync_channel(3);

        thread::spawn(move || {
            let thread_id = thread::current().id();
            for i in 0..10 {
                tx.send(format!("{thread_id:?}: sent Message {i}")).unwrap();
            }
            println!("{thread_id:?}: done");
        });
        thread::sleep(Duration::from_millis(100));

        for msg in rx {
            println!("Main: got {msg}");
        }
    }

    #[test]
    fn test_scoped_thread() {
        fn foo() {
            let s = String::from("Hello");
            let a = thread::scope(|scope| {
                scope.spawn(|| {
                    dbg!(s.len());
                });
            });
        }

        thread::spawn(|| {
            for i in 0..10 {
                println!("Count in thread: {i}!");
                thread::sleep(Duration::from_millis(1));
            }
        });
    }
}
