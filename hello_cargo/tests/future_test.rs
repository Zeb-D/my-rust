// 对 Rust 中异步取消问题的一些见解 https://mp.weixin.qq.com/s/fmanNA76Ng2gj5nY1Fy_2g
// Rust 凭借其独特的所有权系统和零成本抽象，为异步编程提供了强大而安全的支持。
// 然而，在这看似平静的表面之下，隐藏着一些复杂的问题，其中异步任务的取消（Cancellation）就是一个经常被忽视却至关重要的话题。
// 异步取消是指在异步任务执行过程中，因为某些原因（如超时、用户中断、资源约束等）需要提前终止任务的机制。在 Rust 中，这个看似简单的操作实际上涉及到了复杂的控制流转换和资源管理问题。

#[cfg(test)]
mod tests {
    // 在 Rust 的异步世界中，取消操作并不是简单地"终止"一个任务，而是通过一系列精心设计的机制来确保安全性和可预测性。
    // 当一个异步任务被取消时，会发生以下过程：
    // Future 的 poll 方法接收到取消信号
    // 执行资源清理和状态重置
    // 向上层调用者传播取消状态
    // 确保所有相关资源被正确释放
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

        fn cancel(&self) {
            let mut cancelled = self.cancelled.lock().unwrap();
            *cancelled = true;
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

    #[tokio::test]
    async fn test_future() {
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

    #[tokio::test]
    async fn test_raii_future() {
        // Rust 的 RAII（资源获取即初始化）模式在异步取消场景下可能会遇到挑战。
        // 定义一个资源守卫
        #[warn(dead_code)]
        struct ResourceGuard {
            resource: Arc<Mutex<Vec<String>>>,
        }

        impl ResourceGuard {
            async fn new(resource: Arc<Mutex<Vec<String>>>) -> Self {
                // 模拟资源获取
                resource.lock().unwrap().push("Acquired".to_string());
                ResourceGuard { resource }
            }
        }

        impl Drop for ResourceGuard {
            fn drop(&mut self) {
                // 在析构时释放资源
                println!("Resource released");
                // 由于 Drop 是同步的，不能直接使用异步操作
                // 如果需要异步释放资源，可以使用其他机制（如 spawn 一个任务）
            }
        }

        // 模拟一个可能被取消的长时间操作
        async fn process_with_resource(resource: Arc<Mutex<Vec<String>>>) {
            let guard = ResourceGuard::new(resource.clone()).await;
            // 模拟长时间操作
            sleep(Duration::from_secs(2)).await;
            // guard 在这里被自动释放
        }

        let resource = Arc::new(Mutex::new(Vec::new()));

        // 第一次调用
        process_with_resource(resource.clone()).await;
        let resource_state = resource.lock().unwrap().clone();
        assert_eq!(*resource_state, vec!["Acquired".to_string()]);
        drop(resource_state); // 显式释放锁

        // 第二次调用
        process_with_resource(resource.clone()).await;
        let resource_state = resource.lock().unwrap().clone();
        assert_eq!(
            *resource_state,
            vec!["Acquired".to_string(), "Acquired".to_string()]
        );
        drop(resource_state); // 显式释放锁
    }

    #[tokio::test]
    async fn test_complex_task_future() {
        // 取消操作往往会产生级联效应，影响整个异步任务链。
        use std::time::Duration;
        use tokio::time::sleep;

        struct SubTask {
            name: String,
            duration: Duration,
        }
        impl SubTask {
            async fn execute(&self) -> Result<String, &'static str> {
                sleep(self.duration).await;
                Ok(format!("{} completed", self.name))
            }
        }

        struct ComplexTask {
            subtasks: Vec<SubTask>,
        }
        impl ComplexTask {
            async fn execute(self) -> Result<Vec<String>, &'static str> {
                // 并行执行所有子任务
                let results =
                    futures::future::join_all(self.subtasks.iter().map(|task| task.execute()))
                        .await;

                // 收集所有成功的结果
                let successful_results: Vec<String> =
                    results.into_iter().filter_map(|r| r.ok()).collect();

                if successful_results.is_empty() {
                    Err("All subtasks failed")
                } else {
                    Ok(successful_results)
                }
            }
        }

        let mut complex_task = ComplexTask {
            subtasks: Vec::new(),
        };
        complex_task.subtasks.push(SubTask {
            name: String::from("Yd"),
            duration: Duration::from_secs(11),
        });
        println!("{:?}", complex_task.execute().await.unwrap())
    }
}
