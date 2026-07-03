#[cfg(test)]
mod tests {
    use std::mem;
    use std::ptr::without_provenance_mut;
    use std::sync::Arc;

    // waiting for https://google.github.io/comprehensive-rust/unsafe-deep-dive/rules-of-the-game/copying-memory/safe.html

    #[test]
    fn test_own_ascii() {
        /// Text that is guaranteed to be encoded within 7-bit ASCII.
        pub struct Ascii<'a>(&'a mut [u8]);
        impl<'a> Ascii<'a> {
            pub fn new(bytes: &'a mut [u8]) -> Option<Self> {
                bytes.iter().all(|&b| b.is_ascii()).then(|| Ascii(bytes))
            }

            /// Creates a new `Ascii` from a byte slice without checking for ASCII
            /// validity.
            ///
            /// # Safety
            ///
            /// Providing non-ASCII bytes results in undefined behavior.
            pub unsafe fn new_unchecked(bytes: &'a mut [u8]) -> Self {
                debug_assert!(bytes.iter().all(|&b| b.is_ascii()));
                Ascii(bytes)
            }
        }
    }

    #[test]
    fn test_references() {
        let mut boxed = Box::new(123);
        let a: *mut i32 = &mut *boxed as *mut i32;
        let b: *mut i32 = std::ptr::null_mut();

        unsafe {
            println!("{:?}", *a);
            println!("{:?}", b.as_mut());
        }
    }

    #[test]
    fn test_determining() {
        let b: *mut i32 = std::ptr::null_mut();
        unsafe {
            println!("{:?}", b.as_mut());
            println!("{:?}", b); // 地址0
            let c = without_provenance_mut::<()>(0);
            println!("{:?}", c); // 地址0
        }
    }

    #[test]
    fn test_no_mangle() {
        #[unsafe(no_mangle)]
        unsafe fn get(array: *const i32, offset: isize) -> i32 {
            unsafe { *array.offset(offset) }
        }

        let arr = [10, 20, 30, 40];
        let ptr = arr.as_ptr();
        unsafe {
            // 合法：获取索引 2 的元素
            let val = get(ptr, 2);
            println!("arr[2] = {}", val);

            // 危险！越界访问，未定义行为
            let _bad = get(ptr, 5);
            println!("{_bad}")
        }
    }
    #[test]
    fn test_inbounds() {
        let arr = [1, 2, 3];
        // 安全版会被 panic
        // 使用 unsafe 获取越界引用
        let ptr = unsafe { arr.as_ptr().add(8) }; // 指针越界，但尚未创建引用
        // 但是一旦创建引用：
        let r = unsafe { &*ptr }; // 创建越界引用（即使不读取）就是 UB
        // 如果后续代码依赖该引用（甚至只比较地址），优化器可能产生错误
        println!("Pointer address: {:p}", r); // 仅仅打印地址也可能触发问题
        println!("{r}") // 从add(4)之后就开始随机打印不同的值包括1
    }

    #[test]
    fn test_lifetimes() {
        let r: &i32;
        {
            let x = 42;
            r = unsafe { &*(&x as *const i32) }; // 强行延长生命周期
        } // x 被销毁
        println!("{}", r); // 悬垂引用！未定义行为

        // let r: &i32;
        // {
        //     let x = 42;
        //     r = &x; // 编译报错
        // }
    }

    #[test]
    fn test_pointer_provenance() {
        let addr = 0x1234_usize;
        let p = addr as *const u32; // 现在 Rust 会警告或报错（取决于版本）
        unsafe {
            println!("{}", *p);
        } // 几乎肯定崩溃
    }

    #[test]
    fn test_initialization() {
        let data: [u8; 4] = unsafe { mem::uninitialized() }; // 危险！未初始化
        println!("{:?}", data);
        let n = u32::from_le_bytes(data); // 读取未初始化的字节 -> 未定义行为
        println!("{}", n);
    }

    #[test]
    fn test_aliasing() {
        let mut x = 42;
        let p = &mut x as *mut i32;
        let r1 = unsafe { &mut *p };
        let r2 = unsafe { &mut *p }; // 同时存在两个可变引用！未定义行为
        *r1 += 1;
        *r2 += 1; // 编译器可能重排，结果不可预测

        println!("{x}");
    }

    #[test]
    fn test_validity() {
        let data = [3u8; 1];
        println!("{:?}", data);
        let p = data.as_ptr();
        let b: bool = unsafe { *p.cast::<bool>() }; // 未定义行为！3 不是 bool 有效值
        println!("{}", b);
    }

    #[test]
    fn test_alignment() {
        let data = [0u8; 16];
        // 取地址，强制偏移 1 字节，使其非 8 字节对齐
        let p = data.as_ptr().wrapping_add(1);
        // let p = data.as_ptr().wrapping_add(8);
        let r: &u64 = unsafe { &*(p as *const u64) }; // 未定义行为！非对齐引用
        println!("{}", r);
    }

    #[test]
    fn test_overflow() {
        /// Adds 2^31 - 1 to negative numbers.
        unsafe fn may_overflow(a: i32) -> i32 {
            a + i32::MAX
        }

        // let a: i32 = 1;
        // let b = a + i32::MAX; // 编译失败
        // print!("{b}");

        let x = unsafe { may_overflow(123) };
        println!("{x}");
    }

    #[test]
    fn test_unsafe_std() {
        let pid = unsafe { libc::getpid() };
        println!("{pid}");

        // let mut buf = [0u8; 8];
        // let ptr = buf.as_mut_ptr() as *mut libc::c_void;
        //
        // let status = unsafe { libc::getrandom(ptr, buf.len(), 0) };
        // if status > 0 {
        //     println!("{buf:?}");
        // }

        fn iter_sum(xs: &[u64]) -> u64 {
            xs.iter().sum()
        }

        fn fast_sum(xs: &[u64]) -> u64 {
            let mut acc = 0;
            let mut i = 0;
            unsafe {
                while i < xs.len() {
                    acc += *xs.get_unchecked(i);
                    i += 1;
                }
            }
            acc
        }

        let data: Vec<_> = (0..1_000_000).collect();

        let baseline = iter_sum(&data);
        let unchecked = fast_sum(&data);

        assert_eq!(baseline, unchecked);
        println!("{unchecked}")
    }

    #[test]
    fn test_unsafe_trait() {
        pub struct LogicalClock {
            inner: std::sync::Arc<std::sync::atomic::AtomicUsize>,
        }
        unsafe impl Send for LogicalClock {}
        unsafe impl Sync for LogicalClock {}

        let c = LogicalClock {
            inner: Arc::new(Default::default()),
        };
        c.inner.store(4, std::sync::atomic::Ordering::SeqCst);
        println!("{:?}", c.inner)
    }

    #[test]
    fn test_unsafe_fn() {
        /// Convert a nullable pointer to a reference.
        ///
        /// Returns `None` when `p` is null, otherwise wraps `val` in `Some`.
        fn ptr_to_ref<'a, T>(ptr: *mut T) -> Option<&'a mut T> {
            if ptr.is_null() {
                None
            } else {
                // SAFETY: `ptr` is non-null
                unsafe { Some(&mut *ptr) }
            }
        }

        let mut numbers = vec![0, 1, 2, 3, 4];
        let ptr = ptr_to_ref(numbers.as_mut_ptr());
        println!("{:?}", ptr);
    }

    #[test]
    fn test_unsafe() {
        let numbers = vec![0, 1, 2, 3, 4];
        let i = numbers.len() / 2 + 1;

        let x = *unsafe { numbers.get_unchecked(i) };
        assert_eq!(i, x);

        let x = numbers.get(10);
        println!("{:?}", x);
    }
}
