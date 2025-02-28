// 保障Rust并发编程的可靠性：深入内存顺序机制 https://mp.weixin.qq.com/s/6QtGhwVyGzWRj_7cwtCb1w
// 处理器硬件优化带来的指令重排序和缓存不一致性，使得多线程环境下的内存访问变得复杂。Rust的Ordering枚举通过五种不同的内存顺序模式，为开发者提供了精细的控制手段。
// 理解这些模式需要先明确两个基本概念：
// 1. 顺序一致性（Sequential Consistency）：理想的执行模型，所有线程观察到的操作顺序一致
// 2. 处理器实际行为：现代CPU采用乱序执行、缓存分层等优化技术，导致实际执行顺序与代码顺序存在差异
// Rust的原子类型（如AtomicBool、AtomicUsize）配合不同的Ordering参数，正是为了在这两个极端之间找到平衡点。
// 这种机制既保证了必要的执行效率，又提供了确定性的内存可见性保证。

#[cfg(test)]
mod tests {
    use std::thread;

    #[test]
    fn test_relaxed() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let counter = AtomicUsize::new(0);
        // Relaxed顺序提供最基本的原子性保证，不包含任何内存屏障。
        // 适用于不需要同步其他内存操作的场景，比如简单的计数器。但使用时必须确保没有数据依赖关系，否则可能产生违反直觉的结果。
        counter.fetch_add(1, Ordering::Relaxed);
        assert_eq!(counter.load(Ordering::Relaxed), 1);

        // 这对组合形成了典型的生产者-消费者模式。Acquire确保后续读操作不会被重排序到获取操作之前，
        // Release确保之前的写操作不会被重排序到释放之后。这种模式非常适合构建自旋锁等同步机制。
        use std::sync::atomic::AtomicBool;
        let lock = AtomicBool::new(false);
        // 获取锁
        while lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            println!("compare_exchange")
        }
        // 释放锁
        lock.store(false, Ordering::Release);
        assert_eq!(lock.load(Ordering::Relaxed), false);

        // SeqCst：全局一致性的代价
        let flag = AtomicBool::new(false);
        // 线程A
        flag.store(true, Ordering::SeqCst);
        println!("thread A {} finished", flag.load(Ordering::SeqCst));

        if flag.load(Ordering::SeqCst) {
            // 保证看到最新值
            println!("thread B {} finished", flag.load(Ordering::SeqCst));
        }
    }

    #[test]
    fn test_spin_locked() {
        // 自旋锁的实现示例
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        use std::thread;

        // 这个实现展示了Acquire-Release的典型应用，确保了锁获取和释放操作的内存可见性。
        struct SpinLock {
            locked: AtomicBool,
        }
        impl SpinLock {
            fn new() -> Self {
                SpinLock {
                    locked: AtomicBool::new(false),
                }
            }
            fn lock(&self) {
                while self
                    .locked
                    .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
                    .is_err()
                {}
            }
            fn unlock(&self) {
                self.locked.store(false, Ordering::Release);
            }
        }

        // 使用示例
        let lock = Arc::new(SpinLock::new());
        let lock_clone = Arc::clone(&lock);

        thread::spawn(move || {
            lock_clone.lock();
            // 临界区操作
            lock_clone.unlock();
        });
        println!()
    }

    #[test]
    fn test_relaxed_seq_cst() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct Counter {
            count: AtomicUsize,
        }

        impl Counter {
            fn increment(&self) {
                self.count.fetch_add(1, Ordering::Relaxed);
            }
            fn get(&self) -> usize {
                self.count.load(Ordering::SeqCst)
            }
        }

        let c = Counter {
            count: AtomicUsize::new(1),
        };
        c.increment();
        println!("{}", c.get())
    }
    
    // 是否需要同步？ → 否 → Relaxed
    //             ↓
    //             是 → 是否需要全局可见？ → 是 → SeqCst
    //                         ↓
    //                         否 → Acquire/Release组合
}
