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
    use std::ops::Sub;
    use std::pin::Pin;
    use std::sync::atomic::Ordering;
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

    #[tokio::test]
    async fn test_graceful_shutdown_cancel_future() {
        use std::sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        };
        use tokio::{
            sync::{mpsc, Mutex},
            time::{sleep, Duration},
        };

        #[derive(Debug)]
        enum TaskStatus {
            Running,
            Completed(String),
            Cancelled,
            Failed(String),
        }

        #[derive(Clone)]
        struct CancellableTask {
            cancel_flag: Arc<AtomicBool>,
            status_sender: Arc<mpsc::Sender<TaskStatus>>,
        }

        impl CancellableTask {
            fn new(status_sender: mpsc::Sender<TaskStatus>) -> Self {
                CancellableTask {
                    cancel_flag: Arc::new(AtomicBool::new(false)),
                    status_sender: Arc::new(status_sender),
                }
            }

            async fn run(&self) -> Result<String, &'static str> {
                self.status_sender.send(TaskStatus::Running).await.unwrap();

                for i in 1..=5 {
                    // 每步检查取消标志
                    if self.cancel_flag.load(Ordering::SeqCst) {
                        self.status_sender
                            .send(TaskStatus::Cancelled)
                            .await
                            .unwrap();
                        return Err("Task cancelled");
                    }

                    sleep(Duration::from_secs(1)).await;

                    // 强制插入一个 yield，防止长时间操作导致无法及时响应取消
                    tokio::task::yield_now().await;

                    let progress = format!("Task is running {}...", i);
                    self.status_sender.send(TaskStatus::Running).await.unwrap();

                    // 每步检查取消标志
                    if self.cancel_flag.load(Ordering::SeqCst) {
                        self.status_sender
                            .send(TaskStatus::Cancelled)
                            .await
                            .unwrap();
                        return Err("Task cancelled");
                    }
                }

                let result = "Task completed successfully".to_string();
                self.status_sender
                    .send(TaskStatus::Completed(result.clone()))
                    .await
                    .unwrap();
                Ok(result)
            }

            fn cancel(&self) {
                self.cancel_flag.store(true, Ordering::SeqCst);
            }

            async fn reset(&self) {
                self.cancel_flag.store(false, Ordering::SeqCst);
                sleep(Duration::from_millis(10)).await; // 确保任务有时间结束
            }
        }

        struct TaskMonitor {
            task: CancellableTask,
            status_receiver: mpsc::Receiver<TaskStatus>,
        }

        impl TaskMonitor {
            fn new() -> Arc<Mutex<Self>> {
                let (status_sender, status_receiver) = mpsc::channel(100);
                Arc::new(Mutex::new(TaskMonitor {
                    task: CancellableTask::new(status_sender),
                    status_receiver,
                }))
            }

            async fn monitor(&mut self) {
                let task = self.task.clone();
                let task_handle = tokio::spawn(async move { task.run().await });

                while let Some(status) = self.status_receiver.recv().await {
                    match status {
                        TaskStatus::Running => println!("Task is running..."),
                        TaskStatus::Completed(result) => {
                            println!("Task completed with result: {}", result);
                            break;
                        }
                        TaskStatus::Cancelled => {
                            println!("Task was cancelled");
                            break;
                        }
                        TaskStatus::Failed(error) => {
                            println!("Task failed: {}", error);
                            break;
                        }
                    }
                }

                if let Err(err) = task_handle.await {
                    println!("Task panicked: {:?}", err);
                }
            }
        }

        let monitor = TaskMonitor::new();

        // 第一次任务，1 秒后取消任务
        let monitor_clone = Arc::clone(&monitor);
        let monitor_handle = tokio::spawn(async move {
            monitor_clone.lock().await.monitor().await;
        });

        sleep(Duration::from_secs(1)).await; // 等待任务开始运行

        // 取消任务
        monitor.lock().await.task.cancel();
        println!("✅ 任务已取消");

        sleep(Duration::from_secs(1)).await; // 给任务一点时间响应取消
        monitor_handle.await.unwrap();

        println!("✅ 任务已取消，开始新的任务...");

        // 重新启动任务
        let monitor_clone = Arc::clone(&monitor);
        monitor.lock().await.task.reset().await; // 重新设置任务
        let monitor_handle = tokio::spawn(async move {
            monitor_clone.lock().await.monitor().await;
        });

        sleep(Duration::from_secs(6)).await; // 等待新任务完成
        monitor_handle.await.unwrap();

        println!("✅ 任务完成！");
    }

    #[tokio::test]
    async fn test_optimized_cancel_future() {
        // 虽然需要及时响应取消请求，但过于频繁的取消检查会影响性能。
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        use tokio::time::{Duration, Instant};

        #[derive(Clone)]
        struct OptimizedTask {
            cancel_flag: Arc<AtomicBool>,
            check_interval: Duration,
            last_check: Instant,
        }

        impl OptimizedTask {
            async fn run(&mut self) -> Result<(), &'static str> {
                let now = Instant::now();
                let max_duration = Duration::from_secs(2); // 设置超时

                while !self.should_check_cancellation() {
                    // 每次迭代都检查取消标志
                    if self.cancel_flag.load(Ordering::Relaxed) {
                        return Err("Task cancelled");
                    }

                    // 更新 `self.last_check` 来检查是否已经达到 `check_interval`
                    self.last_check = Instant::now();

                    // 检查是否超时
                    if now.elapsed() > max_duration {
                        println!("Task time out");
                        return Err("Task time out");
                    }

                    // 执行任务的工作...
                    println!("task is running... time going {:?}", now.elapsed());

                    // 模拟任务的工作
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }

                println!("time to end");
                Ok(())
            }

            fn should_check_cancellation(&self) -> bool {
                self.last_check.elapsed() >= self.check_interval
            }
        }

        // 创建 cancel_flag
        let cancel_flag = Arc::new(AtomicBool::new(false));

        // 第一次任务：验证超时
        let mut task = OptimizedTask {
            cancel_flag: cancel_flag.clone(),
            check_interval: Duration::from_millis(500),
            last_check: Instant::now(),
        };

        let result = tokio::spawn(async move { task.run().await }).await.unwrap();
        // 验证任务超时
        println!("{:?}", result);
        assert!(
            result.is_err(),
            "Task should be err (timeout), but it was: {:?}",
            result
        );

        // 第二次任务：验证取消
        let mut task = OptimizedTask {
            cancel_flag: cancel_flag.clone(),
            check_interval: Duration::from_millis(500),
            last_check: Instant::now(),
        };

        let task_handle = tokio::spawn(async move { task.run().await });

        // 等待一些时间让任务运行
        tokio::time::sleep(Duration::from_millis(300)).await;

        // 设置取消标志
        cancel_flag.store(true, Ordering::SeqCst);

        // 等待任务完成
        let result = task_handle.await.unwrap();
        // 验证任务被取消
        println!("{:?}", result);
        assert!(
            result.is_err(),
            "Task should be err (cancelled), but it was: {:?}",
            result
        );
    }

    #[tokio::test]
    async fn test_managed_resource_cleanup_future() {
        use std::collections::HashMap;
        use std::sync::Arc;
        use tokio::sync::{oneshot, Mutex};
        use tokio::time::{sleep, Duration};

        struct ManagedResource {
            data: Arc<Mutex<HashMap<String, Vec<u8>>>>,
            cleanup_tx: Option<oneshot::Sender<()>>,
            cleaned: Arc<Mutex<bool>>, // 额外标志位，确保清理后可验证
        }

        impl ManagedResource {
            async fn new() -> Self {
                let (cleanup_tx, cleanup_rx) = oneshot::channel();
                let data = Arc::new(Mutex::new(HashMap::new()));
                let cleaned = Arc::new(Mutex::new(false));

                let cleanup_data = Arc::clone(&data);
                let cleanup_flag = Arc::clone(&cleaned);

                tokio::spawn(async move {
                    tokio::select! {
                        _ = cleanup_rx => {
                            // 执行清理操作
                            cleanup_data.lock().await.clear();
                            *cleanup_flag.lock().await = true;
                            println!("Resource cleaned up");
                        }
                    }
                });

                ManagedResource {
                    data,
                    cleanup_tx: Some(cleanup_tx),
                    cleaned,
                }
            }

            /// ✅ **改为 `&mut self`，避免移动 `self`**
            async fn cleanup(&mut self) {
                if let Some(tx) = self.cleanup_tx.take() {
                    let _ = tx.send(());
                    println!("cleanup done");
                }
            }
            /// ✅ **新增：检查资源是否被清理**
            async fn is_cleaned_up(&self) -> bool {
                *self.cleaned.lock().await
            }
        }
        impl Drop for ManagedResource {
            fn drop(&mut self) {
                if let Some(tx) = self.cleanup_tx.take() {
                    let _ = tx.send(());
                    println!("cleanup Drop");
                }
            }
        }

        // **🔹 CASE 1: 手动清理数据**
        async fn test_cleanup_manual() {
            let mut resource = ManagedResource::new().await;

            // ✅ **添加数据**
            {
                let mut data = resource.data.lock().await;
                data.insert("key".to_string(), vec![1, 2, 3]);
            }

            // ✅ **手动触发清理**
            resource.cleanup().await;
            sleep(Duration::from_millis(50)).await; // 等待清理完成

            // ✅ **检查数据是否被清空**
            let data = resource.data.lock().await;
            assert!(data.is_empty(), "手动 cleanup() 之后，数据应该被清空！");
            assert!(resource.is_cleaned_up().await, "标志位应为 true！");
            println!("✅ test_cleanup_manual 通过！");
        }

        // **🔹 CASE 2: 依赖 Drop 自动清理**
        async fn test_cleanup_on_drop() {
            let resource = ManagedResource::new().await;

            // ✅ **添加数据**
            {
                let mut data = resource.data.lock().await;
                data.insert("key".to_string(), vec![4, 5, 6]);
            }

            // ✅ **不手动调用 cleanup()，直接 drop**
            drop(resource);
            sleep(Duration::from_millis(50)).await; // 等待 Drop 触发清理

            println!("✅ test_cleanup_on_drop 通过！");
        }

        // **运行测试**
        test_cleanup_manual().await;
        test_cleanup_on_drop().await;
    }

    #[tokio::test]
    async fn test_traced_future() {
        // 使用结构化日志来追踪异步任务的生命周期
        use std::future::Future;
        use std::pin::Pin;
        use std::task::{Context, Poll};
        use tokio::time::{sleep, Duration};
        use tracing::{info, instrument, Level};
        use tracing_subscriber;
        use uuid::Uuid;

        #[derive(Debug, Clone)]
        struct TaskId(Uuid);

        struct TracedFuture<F> {
            inner: Pin<Box<F>>, // 修复：Future 需要放入 `Pin<Box<F>>`
            task_id: TaskId,
        }

        impl<F: Future> TracedFuture<F> {
            fn new(future: F) -> Self {
                TracedFuture {
                    inner: Box::pin(future), // 修复：用 `Box::pin` 确保 `inner` 被固定
                    task_id: TaskId(Uuid::new_v4()),
                }
            }
        }

        impl<F: Future> Future for TracedFuture<F> {
            type Output = F::Output;
            #[instrument(skip(self, cx), fields(task_id = ?self.task_id.0))]
            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                info!("Polling task");
                let inner = self.get_mut(); // `get_mut()` 获取 `&mut Self`
                let future = inner.inner.as_mut(); // `as_mut()` 获取 `Pin<&mut F>`

                match future.poll(cx) {
                    Poll::Ready(output) => {
                        info!("Task completed");
                        Poll::Ready(output)
                    }
                    Poll::Pending => {
                        info!("Task pending");
                        Poll::Pending
                    }
                }
            }
        }

        // 初始化 tracing 日志（仅初始化一次）
        tracing_subscriber::fmt::init(); // 初始化日志

        async fn sample_task() -> &'static str {
            println!("Sample task");
            sleep(Duration::from_millis(100)).await;
            "Task Done"
        }

        // **🔹 测试 1：任务是否能完成**
        let traced = TracedFuture::new(sample_task());
        let result = traced.await;
        assert_eq!(result, "Task Done");
        println!("✅ 测试 1 通过：任务成功完成");

        // **🔹 测试 2：任务挂起后是否能恢复执行**
        async fn pending_task() -> &'static str {
            sleep(Duration::from_millis(50)).await;
            sleep(Duration::from_millis(50)).await;
            "Resumed Task"
        }

        let traced = TracedFuture::new(pending_task());
        let result = traced.await;
        assert_eq!(result, "Resumed Task");
        println!("✅ 测试 2 通过：任务挂起后恢复执行");
    }
}

// 通过本文的深入探讨，我们可以总结出以下关键最佳实践：
//
// 始终为异步任务设计取消机制
// 使用原子操作来管理取消标志
// 实现优雅的资源清理
// 提供清晰的状态反馈
// 合理处理取消的传播
// 优化检查点以平衡响应性和性能
// 使用结构化日志进行调试
// 考虑级联效应
// 正确处理 RAII 资源
// 实现可测试的取消行为
