// 突破自引用困境：用Rust的Pin构建可靠任务队列 https://mp.weixin.qq.com/s/xx7NIN5npl1fjT3Dv9Kc4w
// 在系统编程领域，自引用数据结构一直是个充满挑战的议题。当我们尝试构建任务队列这样的基础组件时，传统编程语言的指针操作往往会带来难以察觉的内存安全问题。
// Rust语言通过独特的所有权系统和生命周期机制，将这些潜在风险暴露在编译阶段，而当遇到自引用结构时，Pin这个特殊类型就成为了破局的关键。
// 自引用结构的核心矛盾在于：
//      当包含自指指针的数据在内存中移动时，原有的指针将指向失效的地址。
//      想象一个链表节点在内存中被整体搬迁，其内部的next指针仍指向旧地址，这种悬垂指针就像定时炸弹般危险。Rust编译器无法通过常规的借用检查来防止这种危险，这正是Pin诞生的意义所在。
#[cfg(test)]
mod tests {
    use std::marker::PhantomPinned;
    use std::pin::Pin;
    use std::ptr::NonNull;
    use std::sync::{Arc, Mutex};

    struct TaskNode<T> {
        task: T,
        next: Option<NonNull<Self>>,
        _pin: PhantomPinned,
    }

    impl<T> TaskNode<T> {
        pub fn new(task: T) -> Pin<Arc<Self>> {
            Arc::pin(Self {
                task,
                next: None,
                _pin: PhantomPinned,
            })
        }
    }

    struct TaskQueue<T> {
        head: Mutex<Option<NonNull<TaskNode<T>>>>,
        tail: Mutex<Option<NonNull<TaskNode<T>>>>,
    }
    impl<T> TaskQueue<T> {
        pub fn new() -> Self {
            Self {
                head: Mutex::new(None),
                tail: Mutex::new(None),
            }
        }
        pub fn push(&self, task: Pin<Arc<TaskNode<T>>>) {
            let ptr = NonNull::from(Arc::as_ref(&task));
            let mut tail = self.tail.lock().unwrap();

            if let Some(mut tail_ptr) = *tail {
                unsafe {
                    tail_ptr.as_mut().next = Some(ptr);
                }
            } else {
                *self.head.lock().unwrap() = Some(ptr);
            }

            *tail = Some(ptr);
        }
        pub fn pop(&self) -> Option<Pin<Arc<TaskNode<T>>>> {
            // 从头节点取出
            let mut head = self.head.lock().unwrap();
            head.take().map(|ptr| {
                let task = unsafe { Arc::from_raw(ptr.as_ptr()) };
                *head = unsafe { ptr.as_ref().next };
                if head.is_none() {
                    *self.tail.lock().unwrap() = None;
                }
                Pin::new(task)
            })
        }
    }

    // Pin<P>本质上是一个智能指针的包装器，它通过类型系统保证被包装对象不会被意外移动。这种"固定"机制不是运行时成本，而是编译期的静态保障。理解Pin需要把握三个关键特性：
    //
    // 移动语义限制：对Pin<Box<T>>这样的类型，通过私有化DerefMut实现，阻止获取可变引用
    // 分层安全：区分Unpin和!Unpin类型，允许安全类型绕过固定限制
    // 生命周期绑定：通过PhantomPinned标记类型，阻止包含自引用字段的结构体实现Unpin
    #[test]
    fn pin_basic_test() {
        let queue = TaskQueue::new();
        let task = TaskNode::new(42);
        queue.push(task);
        assert_eq!(queue.pop().unwrap().task, 42);
    }
}
