#[cfg(test)]
mod tests {

    #[test]
    fn test_raii_mem_drop() {
        use std::mem::{self, ManuallyDrop};

        {
            let s = String::from("hello");
            println!("before drop");
            mem::drop(s); // 立即释放堆内存
            // println!("{}", s); // ❌ 编译错误：s 已被移动
            println!("after drop");
        }

        {
            let s = String::from("leak me");
            let ptr = s.as_ptr();
            mem::forget(s); // 不调用 drop，堆内存泄漏
            println!("Pointer: {:?}", ptr); // 指针有效，但内存已丢失管理者
        }

        struct MyBuffer {
            data: Vec<u8>,
        }

        impl Drop for MyBuffer {
            fn drop(&mut self) {
                println!("Dropping buffer of size: {}", self.data.len());
            }
        }

        // 1. ManuallyDrop：防止自动析构
        fn test_manually_drop() {
            let mut md = ManuallyDrop::new(MyBuffer {
                data: vec![1, 2, 3],
            });
            // 不会自动打印 "Dropping buffer..."
            // 如需手动销毁：
            unsafe { ManuallyDrop::drop(&mut md) }; // 此时打印一次
        }

        // 2. Box::leak：转为永久静态引用
        fn test_box_leak() -> &'static str {
            let b = Box::new("static string".to_string());
            Box::leak(b) // 将堆内存泄漏给全局生命周期
        }

        // 3. take 和 replace：取出并替换
        fn test_take_replace() {
            let mut s = String::from("Rust");
            let old = mem::take(&mut s); // s 变为空字符串 ""
            assert_eq!(old, "Rust");
            assert_eq!(s, "");

            let mut x = 10;
            let old_x = mem::replace(&mut x, 100);
            assert_eq!(old_x, 10);
            assert_eq!(x, 100);
        }

        // 4. drop_in_place（仅限 unsafe 场景）
        unsafe fn test_drop_in_place(ptr: *mut MyBuffer) {
            std::ptr::drop_in_place(ptr); // 手动调用析构函数（但不释放内存本身）
        }

        test_manually_drop();
        test_box_leak();
        test_take_replace();
    }

    #[test]
    fn test_raii_drop_forget() {
        use std::io::{self, Write};

        struct Transaction;

        impl Transaction {
            fn start() -> Self {
                Transaction
            }

            fn commit(self) -> io::Result<()> {
                writeln!(io::stdout(), "COMMIT")?;

                // Defuse the drop bomb by preventing Drop from ever running.
                std::mem::forget(self);

                Ok(())
            }
        }

        impl Drop for Transaction {
            fn drop(&mut self) {
                // This is the "drop bomb"
                panic!("Transaction dropped without commit!");
            }
        }

        {
            let tx = Transaction::start();
            // Use `tx` to build the transaction, then commit it.
            // Comment out the call to `commit` to see the panic.
            tx.commit();
        }

        print!("end");
    }

    #[test]
    fn test_raii_before_drop() {
        use std::io::{self, Write};

        struct Transaction {
            active: bool,
        }

        impl Transaction {
            fn start() -> Self {
                Self { active: true }
            }

            fn commit(mut self) -> io::Result<()> {
                writeln!(io::stdout(), "COMMIT")?;
                self.active = false;
                Ok(())
            }
        }

        impl Drop for Transaction {
            fn drop(&mut self) {
                if self.active {
                    panic!("Transaction dropped without commit!");
                }
            }
        }

        {
            let tx = Transaction::start();
            // Use `tx` to build the transaction, then commit it.
            // Comment out the call to `commit` to see the panic.
            tx.commit();

            // let tx = Transaction::start();
            // panic
        }

        println!("end")
    }

    #[test]
    fn test_raii_mutex() {
        use std::sync::Mutex;

        let m = Mutex::new(vec![1, 2, 3]);

        let mut guard = m.lock().unwrap();
        guard.push(4);
        guard.push(5);
        println!("{guard:?}");
        std::mem::drop(guard);
        // guard.push(12);
    }

    #[test]
    fn test_raii_drop_skipped() {
        #[derive(Debug)]
        struct OwnedFd(i32);

        impl Drop for OwnedFd {
            fn drop(&mut self) {
                println!("OwnedFd::drop() called with raw fd: {:?}", self.0);
            }
        }

        impl Drop for TmpFile {
            fn drop(&mut self) {
                println!("TmpFile::drop() called with owned fd: {:?}", self.0);
                // libc::unlink("/tmp/file")
                // panic!("TmpFile::drop() panics");
            }
        }

        #[derive(Debug)]
        struct TmpFile(OwnedFd);

        impl TmpFile {
            fn open() -> Self {
                Self(OwnedFd(2))
            }

            fn close(&self) {
                panic!("TmpFile::close(): not implemented yet");
            }
        }

        let owned_fd = OwnedFd(1);

        let file = TmpFile::open();

        std::process::exit(0);

        // std::mem::forget(file);

        file.close();

        let _ = owned_fd;
    }

    #[test]
    fn test_enforce_invariants() {
        pub struct Username(String);

        impl Username {
            pub fn new(username: String) -> Result<Self, InvalidUsername> {
                if username.is_empty() {
                    return Err(InvalidUsername::CannotBeEmpty);
                }
                if username.len() > 32 {
                    return Err(InvalidUsername::TooLong {
                        len: username.len(),
                    });
                }
                Ok(Self(username))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
        pub enum InvalidUsername {
            CannotBeEmpty,
            TooLong { len: usize },
        }

        let bob = Username::new("Bob".to_string());
    }

    #[test]
    fn test_newtype() {
        struct Username(String);
        struct Password(String);
        struct LoginError;

        fn login(username: &Username, password: &Password) -> Result<(), LoginError> {
            // [...]
            Ok(())
        }

        let password = Password("password".into());
        let username = Username("username".into());
        // login(password, username); // 🛠️❌
    }
}
