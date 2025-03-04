// https://mp.weixin.qq.com/s/t7UtEbAS1sgdc88tIMle9g Rust 中的数据同步 parking_lot
// 在 Rust 多线程环境中，变量必须始终在线程之间同步，以避免数据竞争。
// 在安全的 Rust 中，如果不让此变量实现Sync特性，就不可能声明一个全局变量并让其从多个线程访问。
#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;

    #[test]
    fn test_mutex_condvar() {
        #[derive(Clone)]
        struct SharedChannel<T> {
            inner: Arc<Inner<T>>,
        }

        impl<T> SharedChannel<T> {
            fn new(capacity: usize) -> Self {
                Self {
                    inner: Arc::new(Inner {
                        buffer: Mutex::new(VecDeque::with_capacity(capacity)),
                        capacity,
                        data_available: Condvar::new(),
                        space_available: Condvar::new(),
                    }),
                }
            }
            fn send(&self, value: T) {
                let mut buffer = self.inner.buffer.lock().unwrap();
                while buffer.len() == self.inner.capacity {
                    buffer = self.inner.space_available.wait(buffer).unwrap();
                }
                buffer.push_back(value);
                self.inner.data_available.notify_one();
            }
            fn receive(&self) -> T {
                let mut buffer = self.inner.buffer.lock().unwrap();
                while buffer.is_empty() {
                    buffer = self.inner.data_available.wait(buffer).unwrap();
                }
                let value = buffer.pop_front().unwrap();
                self.inner.space_available.notify_one();
                value
            }
        }

        struct Inner<T> {
            buffer: Mutex<VecDeque<T>>,
            capacity: usize,
            data_available: Condvar,
            space_available: Condvar,
        }

        let channel = SharedChannel::new(2);
        let sender = channel.clone();
        let receiver = channel.clone();

        let send_handle = std::thread::spawn(move || {
            sender.send(1);
            sender.send(2);
        });

        let receive_handle = std::thread::spawn(move || {
            let val1 = receiver.receive();
            let val2 = receiver.receive();
            println!("Received: {} and {}", val1, val2);
        });

        send_handle.join().unwrap();
        receive_handle.join().unwrap();
    }

    #[test]
    fn test_mutex_condvar_cell() {
        #[derive(Clone)]
        struct SharedChannel<T> {
            inner: Arc<Inner<T>>,
        }
        impl<T> SharedChannel<T> {
            fn new() -> Self {
                Self {
                    inner: Arc::new(Inner {
                        buffer: Mutex::new(None),
                        data_available: Condvar::new(),
                    }),
                }
            }
            fn set(&self, value: T) {
                let mut buffer = self.inner.buffer.lock().unwrap();
                *buffer = Some(value);
                self.inner.data_available.notify_one();
            }
            fn take(&self) -> T {
                let mut buffer = self.inner.buffer.lock().unwrap();
                loop {
                    if let Some(value) = buffer.take() {
                        return value;
                    }
                    buffer = self.inner.data_available.wait(buffer).unwrap();
                }
            }
        }
        struct Inner<T> {
            buffer: Mutex<Option<T>>,
            data_available: Condvar,
        }

        let channel = SharedChannel::new();
        let channel_clone = channel.clone();

        let handle = std::thread::spawn(move || {
            channel_clone.set(42);
        });

        let received = channel.take();
        assert_eq!(received, 42);

        handle.join().unwrap();
    }

    #[test]
    fn test_barriers() {
        #[derive(Clone)]
        struct Barrier {
            inner: Arc<Inner>,
        }

        impl Barrier {
            fn new(n: usize) -> Self {
                Self {
                    inner: Arc::new(Inner {
                        count: Mutex::new(0),
                        n,
                        notify: Condvar::new(),
                    }),
                }
            }

            fn wait(&self) -> Guard {
                let mut count = self.inner.count.lock().unwrap();
                *count += 1;

                if *count >= self.inner.n {
                    self.inner.notify.notify_all();
                } else {
                    while *count < self.inner.n {
                        count = self.inner.notify.wait(count).unwrap();
                    }
                }

                Guard {
                    barrier: self.clone(),
                }
            }
        }

        struct Guard {
            barrier: Barrier,
        }
        impl Drop for Guard {
            fn drop(&mut self) {
                let mut count = self.barrier.inner.count.lock().unwrap();
                *count -= 1;
            }
        }

        struct Inner {
            count: Mutex<usize>,
            n: usize,
            notify: Condvar,
        }

        let barrier = Barrier::new(3);
        let mut handles = vec![];

        for _ in 0..3 {
            let barrier_clone = barrier.clone();
            let handle = thread::spawn(move || {
                let _guard = barrier_clone.wait();
                println!("Thread passed the barrier");
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
