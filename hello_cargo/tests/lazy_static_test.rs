// https://mp.weixin.qq.com/s/u2rDzgSohpzQIQdLjGtF7g 深入Rust静态变量黑魔法：lazy_static让你的代码效率提升10倍！
// lazy_static是Rust中处理静态初始化的利器，它能够：简化复杂初始化逻辑提供线程安全的全局状态优化程序性能
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

    // 1. 配置管理
    use std::collections::HashMap;
    lazy_static! {
            static ref CONFIG: Mutex<HashMap<String, String>> = {
                let mut config = HashMap::new();        // 从文件或环境变量加载配置
                config.insert("DATABASE_URL".to_string(), "postgres://localhost:5432".to_string());
                config.insert("API_KEY".to_string(), "secret_key".to_string());
                Mutex::new(config)
            };
    }
    fn update_config(key: &str, value: &str) {
        let mut config = CONFIG.lock().unwrap();
        config.insert(key.to_string(), value.to_string());
    }
    fn get_config(key: &str) -> Option<String> {
        let config = CONFIG.lock().unwrap();
        config.get(key).cloned()
    }
    #[test]
    fn test_hashmap_config() {
        println!("{:?}", get_config("1112"));
        println!("{:?}", get_config("DATABASE_URL"));
        update_config("1112", "1112123");
        println!("{:?}", get_config("1112"));
    }

    // 2. 正则表达式缓存
    use regex::Regex;
    lazy_static! {
        static ref EMAIL_RE: Regex =
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        static ref PHONE_RE: Regex = Regex::new(r"^\d{11}$").unwrap();
    }
    fn validate_email(email: &str) -> bool {
        EMAIL_RE.is_match(email)
    }
    fn validate_phone(phone: &str) -> bool {
        PHONE_RE.is_match(phone)
    }
    #[test]
    fn test_validate() {
        println!("{}", validate_email("1213123@11.cc"));
        println!("{}", validate_phone("1213123"));
    }

    // 2. 复杂初始化逻辑
    use std::sync::RwLock;
    struct ComplexData {
        cache: HashMap<String, Vec<u32>>,
        config: HashMap<String, String>,
    }
    impl ComplexData {
        fn new() -> Self {
            let mut cache = HashMap::new();
            let mut config = HashMap::new();
            // 模拟复杂的初始化过程
            for i in 0..1000 {
                cache.insert(format!("key_{}", i), vec![i as u32; 5]);
            }
            // 加载配置
            config.insert("max_size".to_string(), "1000".to_string());
            config.insert("timeout".to_string(), "30".to_string());
            Self { cache, config }
        }
    }
    lazy_static! {
        static ref COMPLEX_DATA: RwLock<ComplexData> = RwLock::new(ComplexData::new());
    }
    fn read_cache(key: &str) -> Option<Vec<u32>> {
        let data = COMPLEX_DATA.read().unwrap();
        data.cache.get(key).cloned()
    }
    fn write_cache(key: &str, value: Vec<u32>) {
        let mut data = COMPLEX_DATA.write().unwrap();
        data.cache.insert(key.to_string(), value);
    }
    #[test]
    fn test_complexdata() {
        println!("{:?}",read_cache("111"));
        write_cache("111",vec![1,2,3]);
        println!("{:?}",read_cache("111"));
    }
}
