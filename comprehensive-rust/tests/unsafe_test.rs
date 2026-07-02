#[cfg(test)]
mod tests {
    use std::sync::Arc;

    #[test]
    fn test_unsafe_std() {
        let pid = unsafe { libc::getpid() };
        println!("{pid}");


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
