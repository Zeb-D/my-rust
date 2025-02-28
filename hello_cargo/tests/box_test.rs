// Rust中的Box：打破堆栈限制 https://mp.weixin.qq.com/s/xAJ8K_4xkGJG5sTDuxpWPg
// 在Rust的内存管理体系中，Box类型犹如一把打开堆内存世界的金钥匙。这个看似简单的智能指针背后，蕴含着解决内存安全、递归类型和动态分发等复杂问题的精妙设计。
#[cfg(test)]
mod tests {
    use crate::{c_process_data, SensorData};

    #[test]
    fn test_heap_stack() {
        let stack_data = 1024; // 栈上分配的整数
        let heap_data = Box::new(2048); // 堆上分配的整数

        println!("栈数据: {}", stack_data);
        println!("堆数据: {}", heap_data);
    }

    #[test]
    fn test_dyn_box() {
        // 这个动物农场示例展示了Box<dyn Animal>如何容纳不同类型的实例。
        // 编译器在编译期无法确定具体类型大小，通过堆分配和虚表(vtable)机制，实现了运行时的动态调用。
        trait Animal {
            fn speak(&self);
        }

        struct Dog;
        impl Animal for Dog {
            fn speak(&self) {
                println!("Woof!");
            }
        }

        struct Cat;
        impl Animal for Cat {
            fn speak(&self) {
                println!("Meow!");
            }
        }

        fn animal_farm() -> Vec<Box<dyn Animal>> {
            vec![Box::new(Dog), Box::new(Cat)]
        }

        let animal_list = animal_farm();
        for animal in animal_list {
            animal.speak();
        }
    }

    #[test]
    fn test_enum_box() {
        // 这个经典的Cons列表实现中，每个节点通过Box持有下一个节点的指针。如果没有Box的堆分配，编译器将无法确定递归类型的内存大小，导致编译失败。
        #[derive(Debug)]
        enum List<T> {
            Cons(T, Box<List<T>>),
            Nil,
        }

        fn build_list() -> List<i32> {
            List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
        }

        let number_list = build_list();
        println!("{:?}", number_list);
    }

    #[test]
    fn test_drop_box() {
        // 自动内存回收，还展示了如何通过std::mem::drop提前释放资源。这种控制能力在与C语言交互或管理外部资源时尤为重要。
        struct CustomData {
            value: String,
        }

        impl Drop for CustomData {
            fn drop(&mut self) {
                println!("释放资源: {}", self.value);
            }
        }

        let data = Box::new(CustomData {
            value: String::from("重要数据"),
        });

        // 显式释放内存
        drop(data);
        println!("数据已提前释放");
    }

    #[test]
    fn test_ffi_box() {
        // 当Rust需要与C语言交互时，Box成为跨越内存模型鸿沟的桥梁。它能将Rust对象转换为原始指针，同时保持内存安全。

        // 这个FFI示例展示了如何安全地将Rust对象传递给C函数。通过Box::into_raw获得原始指针后，需要确保C端正确管理内存生命周期，或使用Box::from_raw回收内存。

        // 这个FFI示例展示了如何安全地将Rust对象传递给C函数。通过Box::into_raw获得原始指针后，需要确保C端正确管理内存生命周期，或使用Box::from_raw回收内存。
        let data = Box::new(SensorData {
            temperature: 25.6,
            pressure: 1013.25,
        });

        let ptr = Box::into_raw(data);
        unsafe {
            println!("{:?}", ptr as *mut libc::c_void);
            // C端负责释放内存
            // c_process_data(ptr as *mut libc::c_void)
        }
    }
}

extern "C" {
    fn c_process_data(data: *mut libc::c_void);
}

struct SensorData {
    temperature: f32,
    pressure: f32,
}
