use std::thread;
fn alice() -> thread::JoinHandle<()> {
    thread::spawn(move || {
        bob();
    })
}
fn bob() {
    malice();
}
fn malice() {
    panic!("malice is panicking!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_panic_unwinding() {
        let child = alice();
        println!("This is reachable code");
        let _ = child.join(); // 异步里面的panic不会影响主线程
                              // bob(); // 直接导致主线程挂了
        use std::panic;
        panic::catch_unwind(|| {
            bob();
        })
        .ok();

        println!("Survived that panic.");
    }
}
