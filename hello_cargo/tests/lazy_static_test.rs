#[cfg(test)]
mod tests {

    use lazy_static::lazy_static;
    use std::sync::Mutex;

    // lazy_static!宏，可用于初始化任何能够从程 序中的任何位置全局访问的动态类型
    // 能够在多线程环境中被修改，要加入这个依赖 lazy_static = "1.4.0"
    lazy_static! {
        static ref ITEMS: Mutex<Vec<u64>> = {
            let mut v = vec![];
            v.push(9);
            v.push(2);
            v.push(1);
            Mutex::new(v)
        };
    }

    #[test]
    fn test_lazy_static() {
        println!("{:?}", *ITEMS.lock().unwrap());
    }
}
