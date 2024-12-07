// https://mp.weixin.qq.com/s/UCTkLfDm6FXzZOj7ZyExWw 深入Rust的repr黑魔法：内存布局优化实战，让你的代码性能提升50%
// 通过合理使用repr，我们可以：
// 优化内存布局减少内存占用提高缓存利用率优化数据对齐
// 实现更好的互操作性C/C++接口兼容FFI交互ABI稳定性
// 性能优化SIMD支持缓存友好减少填充字节
// 使用建议：默认情况下使用Rust的自动布局需要C兼容性时使用repr(C)特殊场景（如网络协议）考虑repr(packed)单字段包装使用repr(transparent)注意packed结构体的性能影响
// 性能优化要点：合理排列字段顺序注意缓存行对齐避免过度优化benchmark验证效果
#[cfg(test)]
mod tests {
    // 默认情况下，Rust编译器可能会重排字段以优化内存
    #[derive(Debug)]
    struct DefaultStruct {
        a: u8,  // 1字节
        b: u32, // 4字节
        c: u16, // 2字节
    }
    fn show_default_layout() {
        println!(
            "Size of DefaultStruct: {}",
            std::mem::size_of::<DefaultStruct>()
        ); // 8
    }

    #[test]
    fn test_show_default_layout() {
        show_default_layout()
    }

    // C布局
    #[repr(C)]
    struct CStruct {
        a: u8,
        b: u32,
        c: u16,
    }
    // 紧凑布局
    #[repr(packed)]
    struct PackedStruct {
        a: u8,
        b: u32,
        c: u16,
    }
    // 透明布局
    #[repr(transparent)]
    struct TransparentStruct(u64);
    fn compare_layouts() {
        println!("Size of CStruct: {}", std::mem::size_of::<CStruct>()); // 12
        println!(
            "Size of PackedStruct: {}",
            std::mem::size_of::<PackedStruct>()
        ); // 7
        println!(
            "Size of TransparentStruct: {}",
            std::mem::size_of::<TransparentStruct>()
        ); // 8
    }

    #[test]
    fn test_compare_layouts() {
        compare_layouts()
    }

    // 与C语言交互
    #[repr(C)]
    struct InteropStruct {
        len: i32,
        data: *mut u8,
    }
    extern "C" {
        fn process_data(data: *mut InteropStruct);
    }
    impl InteropStruct {
        fn new(len: i32) -> Self {
            InteropStruct {
                len,
                data: std::ptr::null_mut(),
            }
        }
    }

    // 对齐规则
    #[repr(C, align(8))]
    struct AlignedStruct {
        a: u8,
        b: u16,
        c: u32,
    }
    fn alignment_demo() {
        println!(
            "Alignment of AlignedStruct: {}",
            std::mem::align_of::<AlignedStruct>()
        ); // 8
        println!(
            "Size of AlignedStruct: {}",
            std::mem::size_of::<AlignedStruct>()
        ); //8
    }

    #[test]
    fn test_alignment_demo() {
        alignment_demo()
    }

    // 内存节省技巧
    // 常规结构体
    #[repr(C)]
    struct NormalStruct {
        a: u8,
        b: u16,
        c: u32,
    }
    // 紧凑结构体
    #[repr(packed)]
    struct CompactStruct {
        a: u8,
        b: u16,
        c: u32,
    }
    fn compare_sizes() {
        println!("Normal size: {}", std::mem::size_of::<NormalStruct>()); //8
        println!("Packed size: {}", std::mem::size_of::<CompactStruct>()); //7
    }

    #[test]
    fn test_compare_sizes(){
        compare_sizes()
    }

    // 零成本封装
    #[repr(transparent)]
    struct NewType(String);
    impl NewType {
        fn new(s: String) -> Self {
            NewType(s)
        }
        fn into_inner(self) -> String {
            self.0
        }
    }
    // ABI兼容性
    #[repr(transparent)]
    struct FFIString(*mut std::os::raw::c_char);

    // 类型安全包装
    #[repr(transparent)]
    struct UserId(u64);
    #[repr(transparent)]
    struct OrderId(u64);
    fn process_user(user_id: UserId) {
        // 类型安全：不会意外传入OrderId
        println!("Processing user: {}", user_id.0);
    }

    // 位域优化
    #[repr(C)]
    struct BitFlags {
        flags: u32,
    }
    impl BitFlags {
        const FLAG_A: u32 = 1 << 0;
        const FLAG_B: u32 = 1 << 1;
        const FLAG_C: u32 = 1 << 2;
        fn new() -> Self {
            BitFlags { flags: 0 }
        }
        fn set_flag_a(&mut self, value: bool) {
            if value {
                self.flags |= Self::FLAG_A;
            } else {
                self.flags &= !Self::FLAG_A;
            }
        }
    }

    // 缓存对齐优化
    #[repr(align(64))]
    struct CacheLine {
        data: [u8; 64],
        flag: bool,
    }
    impl CacheLine {
        fn new() -> Self {
            CacheLine {
                data: [0; 64],
                flag: false,
            }
        }
    }

    // 数据结构优化
    // 优化前
    struct UnoptimizedStruct {
        small1: u8,//1
        big1: u64,//8
        small2: u8,//1
        big2: u64,//8
    }
    // 优化后
    #[repr(C)]
    struct OptimizedStruct {
        big1: u64,
        big2: u64,
        small1: u8,
        small2: u8,
    }
    fn compare_performance() {
        println!(
            "Unoptimized size: {}",
            std::mem::size_of::<UnoptimizedStruct>()
        );//24
        println!("Optimized size: {}", std::mem::size_of::<OptimizedStruct>());//24
        println!(
            "CacheLine size: {}",
            std::mem::size_of::<CacheLine>()
        );//128
    }
    #[test]
    fn test_compare_performance(){
        compare_performance()
    }

    // 选择正确的repr
    // 1. 与C交互使用repr(C)
    #[repr(C)]
    struct FFIStruct {
        length: i32,
        data: *mut u8,
    }
    // 2. 需要最小内存使用repr(packed)
    #[repr(packed)]
    struct NetworkPacket {
        version: u8,
        flags: u8,
        length: u16,
        payload: [u8; 32],
    }
    // 3. 单字段包装使用repr(transparent)
    #[repr(transparent)]
    struct Wrapper<T>(T);
}
