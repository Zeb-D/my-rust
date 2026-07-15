#[cfg(test)]
mod tests {
    use std::mem::{MaybeUninit, transmute};
    use std::ptr::without_provenance_mut;
    use std::sync::Arc;
    use std::{mem, ptr};

    #[test]
    fn test_self_ref_drop_unpinned() {
        use std::cell::RefCell;
        use std::marker::PhantomPinned;
        use std::pin::Pin;

        thread_local! {
        static BATCH_FOR_PROCESSING: RefCell<Vec<String>> = RefCell::new(Vec::new());
    }

        #[derive(Debug)]
        struct CustomString(String);

        #[derive(Debug)]
        struct SelfRef {
            data: CustomString,
            ptr: *const CustomString,
            _pin: PhantomPinned,
        }

        impl SelfRef {
            fn new(data: &str) -> Pin<Box<SelfRef>> {
                let mut boxed = Box::pin(SelfRef {
                    data: CustomString(data.to_owned()),
                    ptr: std::ptr::null(),
                    _pin: PhantomPinned,
                });

                let ptr: *const CustomString = &boxed.data;
                unsafe {
                    Pin::get_unchecked_mut(Pin::as_mut(&mut boxed)).ptr = ptr;
                }
                boxed
            }
        }

        impl Drop for SelfRef {
            fn drop(&mut self) {
                // 用空字符串替换 data，取出原始数据，防止双重释放
                let original = std::mem::replace(&mut self.data, CustomString(String::new()));
                BATCH_FOR_PROCESSING.with(|log| log.borrow_mut().push(original.0));
            }
        }

        let pinned = SelfRef::new("Rust 🦀");
        drop(pinned);

        BATCH_FOR_PROCESSING.with(|batch| {
            println!("Batch: {:?}", batch.borrow());
        });
    }

    #[test]
    fn test_self_ref_drop_pinned() {
        use std::marker::PhantomPinned;
        use std::pin::Pin;

        struct SelfRef {
            data: String,
            ptr: *const String,
            _pin: PhantomPinned,
        }

        impl SelfRef {
            fn new(data: impl Into<String>) -> Pin<Box<SelfRef>> {
                let mut this = Box::pin(SelfRef {
                    data: data.into(),
                    ptr: std::ptr::null(),
                    _pin: PhantomPinned,
                });
                let ptr: *const String = &this.data;
                // SAFETY: `this` is pinned before we create the self-reference.
                unsafe {
                    Pin::as_mut(&mut this).get_unchecked_mut().ptr = ptr;
                }
                this
            }

            // This function can only be called on a pinned `SelfRef`.
            unsafe fn drop_pinned(self: Pin<&mut SelfRef>) {
                // `self` is pinned, so we must not move out of it.
                println!("dropping {}", self.data);
            }
        }

        impl Drop for SelfRef {
            fn drop(&mut self) {
                // We can safely call `drop_pinned` because `drop` is the last time
                // the value is used. We use `new_unchecked` because we know `self`
                // will not be moved again.
                unsafe {
                    SelfRef::drop_pinned(Pin::new_unchecked(self));
                }
            }
        }

        let _pinned = SelfRef::new("Hello, ");
        // `Drop` runs without moving the pinned value
    }

    #[test]
    fn test_self_ref_pin() {
        use std::marker::PhantomPinned;
        use std::pin::Pin;

        #[derive(Debug)]
        pub struct SelfReferentialBuffer {
            data: [u8; 1024],
            cursor: *mut u8,
            _pin: PhantomPinned,
        }

        impl SelfReferentialBuffer {
            pub fn new() -> Pin<Box<Self>> {
                let buffer = SelfReferentialBuffer {
                    data: [0; 1024],
                    cursor: std::ptr::null_mut(),
                    _pin: PhantomPinned,
                };
                let mut pinned = Box::pin(buffer);

                unsafe {
                    let mut_ref = Pin::get_unchecked_mut(pinned.as_mut());
                    mut_ref.cursor = mut_ref.data.as_mut_ptr();
                }

                pinned
            }

            // 修改为接受 Pin<&mut Self>，内部用 unsafe 获取可变引用
            pub fn reset_cursor(self: Pin<&mut Self>) {
                let this = unsafe { self.get_unchecked_mut() };
                this.cursor = this.data.as_mut_ptr();
            }

            pub fn read(&self, n_bytes: usize) -> &[u8] {
                unsafe {
                    let start = self.data.as_ptr();
                    let end = start.add(self.data.len());
                    let cursor = self.cursor as *const u8;

                    assert!((start..=end).contains(&cursor), "cursor is out of bounds");

                    let offset = cursor.offset_from(start) as usize;
                    let available = self.data.len().saturating_sub(offset);
                    let len = n_bytes.min(available);

                    &self.data[offset..offset + len]
                }
            }

            pub fn write(mut self: Pin<&mut Self>, bytes: &[u8]) {
                let this = unsafe { self.as_mut().get_unchecked_mut() };
                unsafe {
                    let start = this.data.as_mut_ptr();
                    let end = start.add(1024);

                    assert!(
                        (start..=end).contains(&this.cursor),
                        "cursor is out of bounds"
                    );
                    let available = end.offset_from(this.cursor) as usize;
                    let len = bytes.len().min(available);

                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), this.cursor, len);
                    this.cursor = this.cursor.add(len);
                }
            }
        }

        let mut rf = SelfReferentialBuffer::new();

        // 通过 as_mut() 获取 Pin<&mut Self> 来调用 write
        rf.as_mut().write(&[82, 12, 83, 85, 33]);

        // 同样，reset_cursor 需要 &mut self，也通过 as_mut()
        rf.as_mut().reset_cursor();

        // read 只需要 &self，可以直接用
        let a = rf.read(4);
        println!("{:?}", a); // 输出 [82, 12, 83, 85]
    }

    #[test]
    fn test_self_ref_offset() {
        #[derive(Debug)]
        pub struct SelfReferentialBuffer {
            data: [u8; 1024],
            position: usize,
        }

        impl SelfReferentialBuffer {
            pub fn new() -> Self {
                SelfReferentialBuffer {
                    data: [0; 1024],
                    position: 0,
                }
            }

            pub fn read(&self, n_bytes: usize) -> &[u8] {
                let available = self.data.len().saturating_sub(self.position);
                let len = n_bytes.min(available);
                &self.data[self.position..self.position + len]
            }

            pub fn write(&mut self, bytes: &[u8]) {
                let available = self.data.len().saturating_sub(self.position);
                let len = bytes.len().min(available);
                self.data[self.position..self.position + len].copy_from_slice(&bytes[..len]);
                self.position += len;
            }
        }

        let mut sr = SelfReferentialBuffer::new();
        println!("1 {:?}", sr.data.as_mut_ptr());

        sr.write(&[82, 12, 3, 12]);
        println!("2 {sr:?}");

        let a = sr.read(4);
        println!("{a:?}");

        sr.position = 0;
        let a = sr.read(4);
        println!("{a:?}");
    }

    #[test]
    fn test_self_ref_raw_pointer() {
        #[derive(Debug)]
        pub struct SelfReferentialBuffer {
            data: [u8; 1024],
            cursor: *mut u8,
        }

        impl SelfReferentialBuffer {
            pub fn new() -> Self {
                let mut buffer = SelfReferentialBuffer {
                    data: [0; 1024],
                    cursor: std::ptr::null_mut(),
                };

                buffer.update_cursor();
                buffer
            }

            // Danger: must be called after every move
            pub fn update_cursor(&mut self) {
                self.cursor = self.data.as_mut_ptr();
            }

            pub fn read(&self, n_bytes: usize) -> &[u8] {
                unsafe {
                    let start = self.data.as_ptr();
                    let end = start.add(1024);
                    let cursor = self.cursor as *const u8;

                    assert!((start..=end).contains(&cursor), "cursor is out of bounds");

                    let available = end.offset_from(cursor) as usize;
                    let len = n_bytes.min(available);
                    std::slice::from_raw_parts(cursor, len)
                }
            }

            pub fn write(&mut self, bytes: &[u8]) {
                unsafe {
                    let start = self.data.as_mut_ptr();
                    let end = start.add(1024);

                    assert!(
                        (start..=end).contains(&self.cursor),
                        "cursor is out of bounds"
                    );
                    let available = end.offset_from(self.cursor) as usize;
                    let len = bytes.len().min(available);

                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), self.cursor, len);
                    self.cursor = self.cursor.add(len);
                }
            }
        }

        let mut sr = SelfReferentialBuffer::new();
        sr.update_cursor();
        println!("1 {:?}", sr.data.as_mut_ptr());

        sr.write(&[82, 12, 3, 12]);
        sr.update_cursor(); // 重置游标到开头
        println!("2 {:?}", sr.data.as_mut_ptr());

        println!("3 {sr:?}");
        let a = sr.read(4);
        println!("{a:?}");
        let b = sr.read(4);
        println!("{b:?}");
    }

    #[test]
    fn test_unpin() {
        use std::marker::PhantomPinned;
        use std::pin::Pin;

        #[derive(Debug)]
        struct MyStruct {
            data: String,
            ptr: *const String,
            // 加入 PhantomPinned，使该类型成为 !Unpin
            _pin: PhantomPinned,
        }

        impl MyStruct {
            fn new(data: String) -> Self {
                MyStruct {
                    data,
                    ptr: std::ptr::null(),
                    _pin: PhantomPinned,
                }
            }

            // 初始化 ptr，使其指向自己的 data 字段
            pub fn init(self: Pin<&mut Self>) {
                let this = unsafe { self.get_unchecked_mut() };
                this.ptr = &this.data as *const String;
            }
        }

        let s = String::from("Hello, world!");
        let ms = MyStruct::new(s);
        println!("{ms:?}");
        let mut pinned = std::pin::pin!(ms);

        pinned.as_mut().init();
        println!("{pinned:?}");
    }

    #[test]
    fn test_move_and_inspect() {
        #[derive(Debug, Default)]
        pub struct DynamicBuffer {
            data: Vec<u8>,
            position: usize,
        }

        #[unsafe(no_mangle)]
        pub fn move_and_inspect(x: DynamicBuffer) {
            println!("{x:?}");
        }

        let a = DynamicBuffer::default();
        let mut b = a;
        b.data.push(b'R');
        b.data.push(b'U');
        b.data.push(b'S');
        b.data.push(b'T');
        move_and_inspect(b);
    }

    #[test]
    fn test_partial_init() {
        // let mut buf = [0u8; 2048];
        let mut buf = [const { MaybeUninit::<u8>::uninit() }; 2048];

        let external_data = b"Hello, Rust!";
        let len = external_data.len();

        for (dest, src) in buf.iter_mut().zip(external_data) {
            dest.write(*src);
        }

        // SAFETY: We initialized exactly 'len' bytes of `buf` with UTF-8 text
        let text: &str = unsafe {
            let ptr: *const u8 = buf.as_ptr().cast::<u8>();
            let init: &[u8] = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(init)
        };

        println!("{text}");

        let len = len + 16;
        let text: &str = unsafe {
            let ptr: *const u8 = buf.as_ptr().cast::<u8>();
            let init: &[u8] = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(init)
        };

        println!("{text}");
    }

    #[test]
    fn test_how_uninit() {
        // Step 1: Create MaybeUninit
        let mut uninit = MaybeUninit::uninit();

        // Step 2: Write a valid value to the memory
        let i = uninit.write(1);
        println!("{}", &i);

        // Step 3: Inform the type system that the memory location is valid
        let init = unsafe { uninit.assume_init() };

        println!("{init}");
    }

    #[test]
    fn test_uninit_write() {
        let mut buf = MaybeUninit::<String>::uninit();

        // Initialize
        let data_ref = buf.write(String::from("Hello, Rust!"));
        println!("data1: {}", data_ref);

        // Overwrite
        let data_ref = buf.write(String::from("Hi again"));
        println!("data2: {}", data_ref);

        // Assignment replaces the whole MaybeUninit value.
        buf = MaybeUninit::new(String::from("Goodbye"));
        let data = unsafe { buf.assume_init() };
        println!("data3: {}", data);
    }
    #[test]
    fn test_zeroed() {
        // 真实的 0
        let mut x = [const { MaybeUninit::<f32>::zeroed() }; 10];
        x[6].write(7.0f32);
        // SAFETY: All values of `x` have been written to
        let x: [u32; 10] = unsafe { transmute(x) };
        println!("{x:?}")
    }

    #[test]
    fn test_uninit_ffi_c() {
        #[repr(C)]
        struct ComplexStruct {
            x: i32,
            y: f64,
        }

        // 如果报错，则需要对cargo build 下，把 c 代码进行软链接下
        unsafe extern "C" {
            fn c_init_struct(ptr: *mut ComplexStruct);
        }

        // 分配未初始化的结构体内存
        let mut uninit = MaybeUninit::<ComplexStruct>::uninit();
        unsafe {
            c_init_struct(uninit.as_mut_ptr()); // C 函数负责填充
            let data = uninit.assume_init(); // 安全地取回（前提是 C 确实填充了）
            println!("x: {}, y: {}", data.x, data.y);
        }
    }
    #[test]
    fn test_uninit_scene1() {
        // 在栈上分配 1KB 空间，但**不初始化**
        let mut buf: [MaybeUninit<u8>; 1024] = [MaybeUninit::uninit(); 1024];

        // 获取裸指针，传给操作系统读函数（模拟）
        let ptr = buf.as_mut_ptr() as *mut u8;
        // 假设操作系统实际读取了 512 字节（模拟写入）
        unsafe {
            for i in 0..512 {
                ptr.add(i).write(i as u8); // 模拟内核写入数据
            }
        }

        // 【关键】我们只将前 512 个字节视为已初始化的数据
        let initialized_slice =
            unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, 512) };

        println!("读取到了 {} 字节", initialized_slice.len());
        println!("{initialized_slice:?}")
    }

    #[test]
    fn test_arrays_uninit() {
        let input = b"RUST";

        let mut buf = [const { MaybeUninit::<u8>::uninit() }; 2048];

        // Initialize elements by writing values to the memory
        for (i, input_byte) in input.iter().enumerate() {
            unsafe {
                let dst = buf.as_mut_ptr().add(i);
                ptr::write((*dst).as_mut_ptr(), *input_byte);
            }
        }

        // When a portion of an array is initialized, one can
        // use unsafe to isolate it
        let ptr_to_init_subslice = buf.as_ptr() as *const u8;
        let init = unsafe { std::slice::from_raw_parts(ptr_to_init_subslice, input.len()) };
        let text = std::str::from_utf8(init).unwrap();
        println!("{text}");

        // We must manually drop the initialized elements
        for element in &mut buf[0..input.len()] {
            unsafe {
                element.assume_init_drop();
            }
        }
    }

    #[test]
    fn test_exposed() {
        pub fn copy(dest: &mut [u8], source: *const u8) {
            let source = {
                let mut len = 0;

                let mut end = source;
                while unsafe { *end != 0 } {
                    len += 1;
                    end = unsafe { end.add(1) };
                }
                println!("{:?}", len);
                unsafe { std::slice::from_raw_parts(source, len + 1) }
            };

            for (dest, src) in dest.iter_mut().zip(source) {
                *dest = *src;
            }
        }

        let a = [114, 117, 115, 116].as_ptr();
        let _a = [114, 117, 115, 116].as_ptr();
        let b = &mut [82, 85, 83, 84, 0];

        println!("{}", String::from_utf8_lossy(b));
        copy(b, a);
        println!("{}", String::from_utf8_lossy(b));
    }

    #[test]
    fn test_encapsulated_copy() {
        pub fn copy(dest: &mut [u8], source: &[u8]) {
            let len = dest.len().min(source.len());
            let mut i = 0;
            while i < len {
                // SAFETY: `i` must be in-bounds as it was produced by source.len()
                let new = unsafe { source.get_unchecked(i) };

                // SAFETY: `i` must be in-bounds as it was produced by dest.len()
                let old = unsafe { dest.get_unchecked_mut(i) };

                *old = *new;
                i += 1;
            }
        }

        let a = &[114, 117, 115, 116];
        let b = &mut [82, 85, 83, 84];

        println!("{}", String::from_utf8_lossy(b));
        copy(b, a);
        println!("{}", String::from_utf8_lossy(b));
    }

    #[test]
    fn test_copy_vec() {
        pub fn copy(dest: &mut [u8], source: &[u8]) {
            for (dest, src) in dest.iter_mut().zip(source) {
                *dest = *src;
            }
        }

        let a = &[114, 117, 115, 116];
        let b = &mut [82, 85, 83, 84];

        println!("{}", String::from_utf8_lossy(b));
        copy(b, a);
        println!("{}", String::from_utf8_lossy(b));
    }

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
