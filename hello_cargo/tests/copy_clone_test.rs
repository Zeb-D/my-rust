// Rust中的Copy与Clone：揭秘数据复制的核心差异 https://mp.weixin.qq.com/s/G1DZVOZL1M73kfwRYWuF9g
// 在Rust语言的所有权系统中，数据复制是一个需要开发者精确控制的底层操作。Copy和Clone这两个trait常被初学者混淆，但它们在语义和实现层面存在本质区别。
#[cfg(test)]
mod tests {

    #[test]
    fn test_default() {
        // Rust通过所有权系统实现内存安全的核心目标。当变量被赋值给其他变量时，默认行为是转移所有权而非复制数据。这种设计能有效防止悬垂指针，但也带来了一个关键问题：何时需要真正的数据复制？
        let x = 5;
        let _y = x; // 整数默认复制
        println!("{}", x); // 正常执行

        let s1 = String::from("hello");
        let _s2 = s1; // 所有权转移
                      // println!("{}", s1); // 编译错误：value borrowed after move
    }

    #[test]
    fn test_copy_trait() {
        // Copy trait标记的类型启用自动按位复制语义，这种复制发生在编译阶段，完全不需要开发者显式调用方法
        // 1. 所有字段都实现Copy
        // 2. 类型不包含析构函数
        #[derive(Debug, Copy, Clone)]
        struct Point {
            _x: i32,
            _y: i32,
        }

        let p1 = Point { _x: 10, _y: 20 };
        let _p2 = p1; // 自动复制发生
        println!("p1: {:?}", p1); // 仍然有效
    }

    #[test]
    fn test_clone_trait() {
        // Clone trait要求显式调用clone()方法执行数据复制。这种复制通常是深拷贝（deep copy），适用于需要完全复制资源的场景。标准库中的String和Vec等类型都实现了Clone。
        #[derive(Clone, Debug)]
        struct Buffer {
            _data: Vec<u8>,
        }

        let buf1 = Buffer {
            _data: vec![1, 2, 3],
        };
        let _buf2 = buf1.clone(); // 显式深拷贝
        println!("buf1 {:?}", buf1)
    }

    #[test]
    fn test_imp_clone_trait() {
        use std::alloc::{alloc, dealloc, Layout};
        use std::ptr::{self, copy_nonoverlapping};

        struct CustomArray {
            ptr: *mut u8,
            len: usize,
        }

        impl CustomArray {
            /// 创建一个新的 CustomArray
            fn new(len: usize) -> Self {
                let layout = Layout::array::<u8>(len).expect("Invalid layout");
                let ptr = unsafe { alloc(layout) };

                if ptr.is_null() {
                    panic!("Memory allocation failed");
                }

                // 初始化内存（可以改为更复杂的初始化逻辑）
                unsafe { ptr::write_bytes(ptr, 0, len) };

                Self { ptr, len }
            }
        }

        impl Clone for CustomArray {
            fn clone(&self) -> Self {
                let layout = Layout::array::<u8>(self.len).expect("Invalid layout");
                let new_ptr = unsafe { alloc(layout) };

                if new_ptr.is_null() {
                    panic!("Memory allocation failed");
                }

                unsafe { copy_nonoverlapping(self.ptr, new_ptr, self.len) };

                Self {
                    ptr: new_ptr,
                    len: self.len,
                }
            }
        }

        impl Drop for CustomArray {
            fn drop(&mut self) {
                if !self.ptr.is_null() {
                    let layout = Layout::array::<u8>(self.len).expect("Invalid layout");
                    unsafe { dealloc(self.ptr, layout) };
                }
            }
        }

        let original = CustomArray::new(10);
        // 复制数据
        let cloned = original.clone();
        // 确保两个实例的指针不同
        assert_ne!(original.ptr, cloned.ptr);

        // 确保长度一致
        assert_eq!(original.len, cloned.len);

        // 检查内容是否相同
        unsafe {
            for i in 0..original.len {
                assert_eq!(*original.ptr.add(i), *cloned.ptr.add(i));
            }
        }
    }

    #[test]
    fn test_arc_large_data() {
        use std::sync::Arc;
        use std::thread;

        #[allow(dead_code)] // 允许未使用字段，避免警告
        struct LargeData([u8; 1024]);

        fn process_data(data: Arc<LargeData>) {
            let ptr = Arc::as_ptr(&data); // 获取 Arc 内部数据的指针
            println!(
                "Processing data in thread: {:?}, data ptr: {:p}",
                thread::current().id(),
                ptr
            );
        }
        let data = Arc::new(LargeData([1; 1024])); // 创建 Arc 管理的 LargeData
        println!("Main thread, data ptr: {:p}", Arc::as_ptr(&data));

        let data_clone = Arc::clone(&data); // 克隆 Arc，增加引用计数

        let handle = thread::spawn(move || {
            println!("sub thread, data clone: {:p}", Arc::as_ptr(&data_clone));
            process_data(data_clone); // 在子线程中处理数据
                                      // 这里不能访问 `data`，因为 `data_clone` 已经 move 进来了
        });

        handle.join().expect("Thread execution failed");

        // 线程结束后，原 Arc 仍然可用，引用计数应该恢复到 1
        println!("Main thread after join, data ptr: {:p}", Arc::as_ptr(&data));
        assert_eq!(Arc::strong_count(&data), 1);
    }
}
