// https://google.github.io/comprehensive-rust/idiomatic/leveraging-the-type-system/borrow-checker-invariants.html

#[cfg(test)]
mod tests {

    #[test]
    fn test_borrow_phantomdata() {
        use std::marker::PhantomData;
        use std::os::raw::c_int;

        mod libc_ffi {
            use std::os::raw::{c_char, c_int};
            pub unsafe fn open(path: *const c_char, oflag: c_int) -> c_int {
                3
            }
            pub unsafe fn close(fd: c_int) {}
        }

        struct OwnedFd {
            fd: c_int,
        }

        impl OwnedFd {
            fn try_from_fd(fd: c_int) -> Option<Self> {
                if fd < 0 {
                    return None;
                }
                Some(OwnedFd { fd })
            }

            fn as_fd<'a>(&'a self) -> BorrowedFd<'a> {
                BorrowedFd {
                    fd: self.fd,
                    _phantom: PhantomData,
                }
            }
        }

        impl Drop for OwnedFd {
            fn drop(&mut self) {
                unsafe { libc_ffi::close(self.fd) };
            }
        }

        struct BorrowedFd<'a> {
            fd: c_int,
            _phantom: PhantomData<&'a ()>,
        }

        // Create a file with a raw syscall with write-only and create permissions.
        let fd = unsafe { libc_ffi::open(c"c_str.txt".as_ptr(), 065) };
        // Pass the ownership of an integer file descriptor to an `OwnedFd`.
        // `OwnedFd::drop()` closes the file descriptor.
        let owned_fd = OwnedFd::try_from_fd(fd).expect("Could not open file with syscall!");

        // Create a `BorrowedFd` from an `OwnedFd`.
        // `BorrowedFd::drop()` does not close the file because it doesn't own it!
        let borrowed_fd: BorrowedFd<'_> = owned_fd.as_fd();
        // std::mem::drop(owned_fd); // ❌🔨
        std::mem::drop(borrowed_fd);
        let second_borrowed = owned_fd.as_fd();
        // owned_fd will be dropped here, and the file will be closed.
    }

    #[test]
    fn test_borrow_check_phantomdata_lifetimes() {
        use std::marker::PhantomData;

        pub type DatabaseHandle = u8; // maximum 255 databases open at the same time

        fn database_open(name: *const std::os::raw::c_char) -> DatabaseHandle {
            unimplemented!()
        }

        struct DatabaseConnection(DatabaseHandle);

        struct Transaction<'a> {
            connection: DatabaseConnection,
            _phantom: PhantomData<&'a mut DatabaseConnection>,
        }

        impl DatabaseConnection {
            fn new_transaction(&mut self) -> Transaction<'_> {
                Transaction {
                    connection: DatabaseConnection(self.0),
                    _phantom: PhantomData,
                }
            }
        }

        let mut con = DatabaseConnection(12);
        {
            let tx = con.new_transaction();
        }

        let tx = con.new_transaction();
    }

    #[test]
    fn test_mut_ref() {
        pub struct QueryResult;
        pub struct DatabaseConnection {/* fields omitted */}

        impl DatabaseConnection {
            pub fn new() -> Self {
                Self {}
            }
            pub fn results(&self) -> &[QueryResult] {
                &[] // fake results
            }
        }

        pub struct Transaction<'a> {
            connection: &'a mut DatabaseConnection,
        }

        impl<'a> Transaction<'a> {
            pub fn new(connection: &'a mut DatabaseConnection) -> Self {
                Self { connection }
            }
            pub fn query(&mut self, _query: &str) {
                // Send the query over, but don't wait for results.
            }
            pub fn commit(&self) {
                // Finish executing the transaction and retrieve the results.
            }
        }

        // 这个地方很神奇：借用后 再释放 就可以回到原先的控制权
        {
            let mut db = DatabaseConnection::new();

            // The transaction `tx` mutably borrows `db`.
            let mut tx = Transaction::new(&mut db);
            tx.query("SELECT * FROM users");

            // This won't compile because `db` is already mutably borrowed by `tx`.
            // let results = db.results(); // ❌🔨

            // The borrow of `db` ends when `tx` is consumed by `commit()`.
            tx.commit();

            // Now it is possible to borrow `db` again.
            let results = db.results();
        }
    }

    #[test]
    fn test_borrow_lifetime() {
        // An internal data type to have something to hold onto.
        #[derive(Copy, Clone)]
        pub struct Internal;
        // The "outer" data.
        #[derive(Copy, Clone)]
        pub struct Data(Internal);

        fn shared_use(value: &Data) -> &Internal {
            &value.0
        }
        fn exclusive_use(value: &mut Data) -> &mut Internal {
            &mut value.0
        }
        fn deny_future_use(value: Data) {}

        fn demo_exclusive() {
            let mut value = Data(Internal);
            let shared = shared_use(&value);
            // let exclusive = exclusive_use(&mut value); // ❌🔨
            let shared_again = shared;
        }

        fn demo_denied() {
            let value = Data(Internal);
            deny_future_use(value);
            let shared = shared_use(&value); // ❌🔨 这里不会编译报错，是因为clone 实现了按位复制功能
        }
    }
    #[test]
    fn borrow_checker_invariants() {
        /// Doors can be open or closed, and you need the right key to lock or unlock
        /// one. Modelled with a Shared key and Owned door.
        pub struct DoorKey {
            pub key_shape: u32,
        }
        pub struct LockedDoor {
            lock_shape: u32,
        }
        pub struct OpenDoor {
            lock_shape: u32,
        }

        fn open_door(key: &DoorKey, door: LockedDoor) -> Result<OpenDoor, LockedDoor> {
            if door.lock_shape == key.key_shape {
                Ok(OpenDoor {
                    lock_shape: door.lock_shape,
                })
            } else {
                Err(door)
            }
        }

        fn close_door(key: &DoorKey, door: OpenDoor) -> Result<LockedDoor, OpenDoor> {
            if door.lock_shape == key.key_shape {
                Ok(LockedDoor {
                    lock_shape: door.lock_shape,
                })
            } else {
                Err(door)
            }
        }

        let key = DoorKey { key_shape: 7 };
        let closed_door = LockedDoor { lock_shape: 7 };
        let opened_door = open_door(&key, closed_door);
        if let Ok(_opened_door) = opened_door {
            println!(
                "Opened the door with key shape '{}'",
                _opened_door.lock_shape
            );
        } else {
            eprintln!(
                "Door wasn't opened! Your key only opens locks with shape '{}'",
                key.key_shape
            );
        }
    }
}
