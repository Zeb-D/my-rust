use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tokio::time::{sleep, Duration};

struct CancellableFuture<F> {
    inner: F,
    cancelled: Arc<Mutex<bool>>, // 使用 Arc<Mutex> 实现线程安全的取消标志
}

impl<F: Future> CancellableFuture<F> {
    fn new(future: F) -> Self {
        CancellableFuture {
            inner: future,
            cancelled: Arc::new(Mutex::new(false)),
        }
    }
}

impl<F: Future> Future for CancellableFuture<F> {
    type Output = Option<F::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 检查是否已取消
        if *self.cancelled.lock().unwrap() {
            return Poll::Ready(None);
        }

        // 安全地轮询内部 Future
        match unsafe { self.as_mut().map_unchecked_mut(|this| &mut this.inner) }.poll(cx) {
            Poll::Ready(output) => Poll::Ready(Some(output)),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[tokio::main]
async fn main() {
    let future = CancellableFuture::new(async {
        sleep(Duration::from_secs(5)).await;
        println!("Task completed!");
        42
    });

    let cancelled = Arc::clone(&future.cancelled); // 克隆 Arc 用于取消操作

    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        let mut cancelled = cancelled.lock().unwrap();
        *cancelled = true;
        println!("Task cancelled!");
    });

    if let Some(result) = future.await {
        println!("Got result: {}", result);
    } else {
        println!("Task was cancelled");
    }
}
