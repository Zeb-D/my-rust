// https://mp.weixin.qq.com/s/6JI3oO4BPWJE4Megh2nVJg Rust Sized trait背后的内存优化黑科技，性能提升竟如此巨大！
// 最佳实践
//      优先使用Sized类型以获得更好的性能
//      合理使用?Sized处理动态大小类型
//      注意内存布局优化
//      利用编译时检查提前发现问题
//      合理使用泛型约束优化性能
// 性能优化要点
//      固定大小类型放在结构体前面
//      使用静态分发代替动态分发
//      利用内联优化关键路径
//      合理使用零拷贝技术
//      编译期间进行类型检查

#[cfg(test)]
mod tests {
    use std::fmt::{Display, Pointer};

    // 1. Sized的本质
    #[test]
    fn test_understand_sized() {
        // 编译时已知大小的类型
        let x: i32 = 42; // Sized
        let arr: [u8; 4] = [1, 2, 3, 4]; // Sized

        // 编译时未知大小的类型
        let str_slice: &str = "hello"; // !Sized
        let dyn_trait: &dyn Display = &42; // !Sized

        generic_sized(str_slice);

        // 编译器自动添加Sized约束
        fn generic_sized<T>(value: T) {
            // 等价于 fn generic_sized<T: Sized>(value: T)
            println!("Size: {}", std::mem::size_of::<T>());
        }
    }

    #[derive(Debug)]
    struct CustomBox<T: ?Sized>(Box<T>);
    impl<T: ?Sized> CustomBox<T> {
        fn new<U>(value: U) -> CustomBox<U>
        where
            U: Sized,
        {
            CustomBox(Box::new(value))
        }
    }
    // 特化实现
    impl CustomBox<str> {
        fn from_str(s: &str) -> CustomBox<str> {
            CustomBox(s.into())
        }
    }
    // 2. 智能指针与Sized
    #[test]
    fn test_custom_box() {
        let c = CustomBox::from_str("hello");
        println!("Size: {}", std::mem::size_of::<CustomBox<&str>>());
    }

    trait VirtualMethod {
        fn process(&self);
    }
    // 优化前：需要两次解引用
    struct Unoptimized<T: ?Sized> {
        data: Box<T>,
    }
    // 优化后：胖指针直接存储
    struct Optimized<T: ?Sized> {
        data: T,
    }
    // 1. 动态分发优化
    #[test]
    fn test_optimize_dispatch() {
        // 实现VirtualMethod的具体类型
        impl VirtualMethod for String {
            fn process(&self) {
                println!("Processing string: {}", self);
            }
        }

        let opt: Optimized<Box<dyn VirtualMethod>> = Optimized {
            data: Box::new(String::from("test")),
        };
        opt.data.process();

        // 直接使用 dyn VirtualMethod 类型，示例中使用的是具体类型
        let m: Box<dyn VirtualMethod> = Box::new(String::from("test11"));
        m.process(); // 调用 Trait 方法

        // 使用 Unoptimized 结构体来存储 trait 对象
        let opt_unoptimized: Unoptimized<Box<dyn VirtualMethod>> =
            Unoptimized { data: Box::new(m) };
        opt_unoptimized.data.process(); // 调用未优化的结构体
    }

    // 2. 零成本抽象
    #[derive(Debug)]
    #[repr(C)]
    struct ZeroCost<T: ?Sized> {
        len: usize,
        data: T,
    }
    impl<T: ?Sized> ZeroCost<T> {
        // 只有Sized类型可以创建
        fn new<U: Sized>(data: U) -> ZeroCost<U> {
            ZeroCost {
                len: std::mem::size_of::<U>(),
                data,
            }
        }
        fn from_slice(slice: &[u8]) -> Box<ZeroCost<[u8]>> {
            // 实现从切片创建
            let len = slice.len();
            // ... 实现细节
            unimplemented!()
        }
    }
    #[test]
    fn test_sized() {
        let s = ZeroCost::<&str>::new("aaaa");
        println!("{:#?}", s);

        let c = CustomBox::from_str("hello");
        let c = ZeroCost::<CustomBox<str>>::new(c);
        println!("{:#?}", c);

        let arr: [u8; 4] = [1, 2, 3, 4]; // Sized
        let a = ZeroCost::<[u8]>::new(arr);
        println!("{:#?}", a);
    }
}
