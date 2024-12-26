// https://mp.weixin.qq.com/s/INgWdj4fxnEhBrl0etp-Ug Rust Any竟能实现这些黑魔法，看完秒变高手！
// 最佳实践
//      使用TypeId进行快速类型比较
//      合理使用downcast操作
//      实现Send + Sync实现线程安全
//      缓存类型信息提升性能
//      使用泛型约束增加类型安全
// 性能优化要点
//      避免频繁的类型检查
//      使用类型缓存
//      优化向下转换路径
//      合理使用静态分发
//      减少动态分发开销
#[cfg(test)]
mod tests {
    #[test]
    fn any_test() {
        use std::any::{Any, TypeId};

        fn is_string(s: &dyn Any) -> bool {
            TypeId::of::<String>() == s.type_id()
        }

        assert_eq!(is_string(&0), false);
        assert_eq!(is_string(&"cookie monster".to_string()), true);
    }

    use std::any::Any;

    // 1. 类型识别基础
    fn basic_type_check() {
        // 基本类型检查
        let num: Box<dyn Any> = Box::new(42);
        let string: Box<dyn Any> = Box::new("Hello".to_string());
        assert!(num.is::<i32>());
        assert!(string.is::<String>());
        // 获取类型名
        println!("Type: {}", std::any::type_name::<i32>());
        // 类型转换
        let any_obj: Box<dyn Any> = Box::new(42);
        if let Some(num) = any_obj.downcast_ref::<i32>() {
            println!("Got number: {}", num);
        }
    }
    #[test]
    fn test_basic_type_check() {
        basic_type_check()
    }

    use std::sync::{Arc, RwLock};
    use std::thread;
    use std::time::Duration;

    type ThreadSafeAny = dyn Any + Send + Sync; // 线程安全的Any对象

    // 2. Any + Send + Sync
    fn thread_safe_any() {
        let data: Arc<ThreadSafeAny> = Arc::new(42);

        // 在线程间共享
        let data_clone = data.clone();
        thread::spawn(move || {
            if let Some(num) = data_clone.downcast_ref::<i32>() {
                println!("Number in thread: {}", num);
            }
        });

        let data_clone = data.clone();
        thread::spawn(move || {
            if let Some(num) = data_clone.downcast_ref::<i32>() {
                println!("Number in thread: {}", num);
            }
        });
    }
    #[test]
    fn test_thread_safe_any() {
        thread_safe_any();
        thread::sleep(Duration::from_secs(2));
    }

    // 基于 thread_safe_any() 实现的一个可读写的操作
    fn thread_safe_any_rw() {
        // 使用 Arc 和 Mutex 封装数据，这样可以确保多线程访问时的安全
        let data: Arc<RwLock<dyn Any + Send + Sync>> = Arc::new(RwLock::new(42));

        // 克隆数据进行线程间共享
        let data_clone = Arc::clone(&data);
        thread::spawn(move || {
            // 尝试读取数据并进行类型转换
            let data_lock = data_clone.try_read().unwrap();
            if let Some(num) = data_lock.downcast_ref::<i32>() {
                println!("Number in thread 1: {}", num);
            }
        });

        // 另一个线程修改数据
        let data_clone = Arc::clone(&data);
        thread::spawn(move || {
            let mut data_lock = data_clone.try_write().unwrap();
            // 修改数据，首先转换为 i32 类型
            if let Some(num) = data_lock.downcast_mut::<i32>() {
                *num = 100; // 修改数据
                println!("Number modified in thread 2: {}", num);
            }
        });

        // 主线程继续操作
        thread::sleep(Duration::from_secs(2)); // 等待线程完成
        let data_clone = Arc::clone(&data);
        let data_lock = data_clone.try_read().unwrap();
        if let Some(num) = data_lock.downcast_ref::<i32>() {
            println!("Number in main thread : {}", num);
        }
    }
    #[test]
    fn test_thread_safe_any_rw() {
        thread_safe_any_rw()
    }

    #[test]
    fn test_type_id() {
        println!("{}", std::any::type_name::<i32>());
        println!("{}", std::any::type_name::<ThreadSafeAny>());
        println!("{}", std::any::type_name::<String>());
        println!("{:?}", std::any::TypeId::of::<ThreadSafeAny>());
        println!("{:?}", std::any::TypeId::of::<i32>());
    }

    #[derive(Debug)]
    struct TypeErasedContainer {
        data: Box<dyn Any>,
        type_id: std::any::TypeId,
    }

    impl TypeErasedContainer {
        fn new<T: 'static>(value: T) -> Self {
            Self {
                data: Box::new(value),
                type_id: std::any::TypeId::of::<T>(),
            }
        }

        fn get<T: 'static>(&self) -> Option<&T> {
            if self.type_id == std::any::TypeId::of::<T>() {
                self.data.downcast_ref()
            } else {
                None
            }
        }
    }
    // 1. 类型擦除与恢复
    #[test]
    fn test_type_erasure_example() {
        let container = TypeErasedContainer::new(String::from("Hello"));

        if let Some(string) = container.get::<String>() {
            println!("Retrieved: {}", string);
        }
        // 写入和获取 的类型不对
        if let None = container.get::<i32>() {
            println!("Retrieved: empty");
        }
    }

    use std::collections::HashMap;

    // 2. 动态类型系统
    struct DynamicTypeSystem {
        objects: HashMap<String, Box<dyn Any>>,
    }

    impl DynamicTypeSystem {
        fn new() -> Self {
            Self {
                objects: HashMap::new(),
            }
        }

        fn insert<T: 'static>(&mut self, name: &str, value: T) {
            self.objects.insert(name.to_string(), Box::new(value));
        }
        fn get<T: 'static>(&self, name: &str) -> Option<&T> {
            self.objects.get(name)?.downcast_ref()
        }
        fn get_mut<T: 'static>(&mut self, name: &str) -> Option<&mut T> {
            self.objects.get_mut(name)?.downcast_mut()
        }
    }
    #[test]
    fn test_dynamic_type_system() {
        let mut system = DynamicTypeSystem::new();
        system.insert("hello", String::from("world"));
        // 尝尝新味道
        let container = TypeErasedContainer::new(String::from("Hello"));
        system.insert("container", container);
        println!(
            "{:?}",
            system.get::<TypeErasedContainer>("container").unwrap()
        );
        println!("{:?}", system.get_mut::<String>("hello").unwrap());
    }

    use std::marker::PhantomData;

    // 1. 类型安全的消息系统
    trait Message: Any + Send {}
    struct MessageBus {
        handlers: HashMap<TypeId, Box<dyn Fn(&dyn Any)>>,
    }
    use std::any::TypeId;
    impl MessageBus {
        fn new() -> Self {
            Self {
                handlers: HashMap::new(),
            }
        }
        fn register<M: Message + 'static>(&mut self, handler: impl Fn(&M) + 'static) {
            let type_id = TypeId::of::<M>();
            let boxed_handler = Box::new(move |any: &dyn Any| {
                if let Some(msg) = any.downcast_ref::<M>() {
                    handler(msg);
                }
            });
            self.handlers.insert(type_id, boxed_handler);
        }
        fn dispatch(&self, message: &dyn Any) {
            if let Some(handler) = self.handlers.get(&message.type_id()) {
                handler(message);
            } else {
                println!("Unknown type {:?}", message);
            }
        }
    }

    // 定义一个具体的消息结构体示例，实现Message trait
    struct ConcreteMessage;
    impl Message for ConcreteMessage {}
    type I32 = i32;
    impl Message for I32 {}
    #[test]
    fn test_message_bus() {
        let mut bus = MessageBus::new();
        let handler_called = std::sync::Arc::new(std::sync::Mutex::new(false));
        let handler_called_clone = handler_called.clone();
        // 定义一个简单的处理函数，用于设置标记表示被调用
        let handler = move |_: &ConcreteMessage| {
            println!("-1->{}", handler_called_clone.lock().unwrap().clone());
            *handler_called_clone.lock().unwrap() = true;
        };
        bus.register::<ConcreteMessage>(handler);
        bus.dispatch(&23);
        bus.dispatch(&ConcreteMessage);

        // I32
        let h = move |i: &I32| {
            println!("-2->{}", i);
        };
        bus.register::<I32>(h);
        bus.dispatch(&123);
    }

    // 2. 插件系统
    trait Plugin: Any {
        fn name(&self) -> &'static str;
        fn execute(&self);
        fn as_any(&self) -> &dyn Any;
    }
    struct PluginManager {
        plugins: HashMap<String, Box<dyn Plugin>>,
    }
    impl PluginManager {
        fn new() -> Self {
            Self {
                plugins: HashMap::new(),
            }
        }
        fn register<P: Plugin + 'static>(&mut self, plugin: P) {
            let name = plugin.name().to_string();
            self.plugins.insert(name, Box::new(plugin));
        }
        fn get_plugin<P: Plugin + 'static>(&self, name: &str) -> Option<&P> {
            self.plugins
                .get(name)
                .and_then(|p| p.as_any().downcast_ref::<P>())
        }
    }

    // 定义一个简单的具体插件结构体示例，实现Plugin trait
    struct ConcretePlugin {
        plugin_name: &'static str,
    }
    impl Plugin for ConcretePlugin {
        fn name(&self) -> &'static str {
            self.plugin_name
        }
        fn execute(&self) {
            // 这里可以添加具体执行逻辑，目前为空实现
            println!("{:?}", self.plugin_name);
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    #[test]
    fn test_plugin_manager() {
        let mut manager = PluginManager::new();
        let plugin = ConcretePlugin {
            plugin_name: "test_plugin",
        };
        manager.register(plugin);
        assert_eq!(manager.plugins.len(), 1);
        manager
            .get_plugin::<ConcretePlugin>("test_plugin")
            .unwrap()
            .execute();
    }

    // 1. 类型缓存
    struct TypeCache<T: 'static> {
        type_id: std::any::TypeId,
        _phantom: PhantomData<T>,
    }
    impl<T: 'static> TypeCache<T> {
        fn new() -> Self {
            Self {
                type_id: std::any::TypeId::of::<T>(),
                _phantom: PhantomData,
            }
        }
        #[inline(always)]
        fn is_type(&self, any: &dyn Any) -> bool {
            any.type_id() == self.type_id
        }
    }
    #[test]
    fn test_type_cache_is_type_same_type() {
        let cache = TypeCache::<ConcretePlugin>::new();
        assert_eq!(cache.type_id, TypeId::of::<ConcretePlugin>());
        let test_value = ConcretePlugin { plugin_name: "" };
        assert!(cache.is_type(&test_value as &dyn Any));

        struct AnotherStruct;
        let another_value = AnotherStruct;
        assert!(!cache.is_type(&another_value as &dyn Any));
    }

    // 2. 快速类型检查
    struct FastTypeCheck {
        type_checks: Vec<Box<dyn Fn(&dyn Any) -> bool>>,
    }
    impl FastTypeCheck {
        fn new() -> Self {
            Self {
                type_checks: Vec::new(),
            }
        }
        fn add_type<T: 'static>(&mut self) {
            let type_id = TypeId::of::<T>();
            self.type_checks
                .push(Box::new(move |any: &dyn Any| any.type_id() == type_id));
        }
        #[inline]
        fn check_all(&self, value: &dyn Any) -> bool {
            self.type_checks.iter().any(|check| check(value))
        }
    }
    #[test]
    fn test_check_all() {
        struct TestType1;
        struct TestType2;

        let mut fast_type_check = FastTypeCheck::new();
        fast_type_check.add_type::<TestType1>();

        let value1 = TestType1;
        let value2 = TestType2;

        assert!(fast_type_check.check_all(&value1 as &dyn Any));
        assert!(fast_type_check.check_all(&value1 as &dyn Any)); // double check if life's time
        assert!(!fast_type_check.check_all(&value2 as &dyn Any));
    }

    // 2. 反射系统
    struct Reflector {
        type_info: HashMap<TypeId, TypeInfo>,
    }
    struct TypeInfo {
        name: &'static str,
        methods: Vec<MethodInfo>,
    }
    struct MethodInfo {
        name: &'static str,
        invoke: Box<dyn Fn(&dyn Any) -> Box<dyn Any>>,
    }
    impl Reflector {
        fn register_type<T: 'static>(&mut self, type_info: TypeInfo) {
            self.type_info.insert(TypeId::of::<T>(), type_info);
        }
        fn get_type_info(&self, value: &dyn Any) -> Option<&TypeInfo> {
            self.type_info.get(&value.type_id())
        }
    }
    #[test]
    fn test_register_reflector() {
        let mut reflector = Reflector {
            type_info: HashMap::new(),
        };
        struct TestType;
        let type_info = TypeInfo {
            name: "TestType",
            methods: Vec::new(),
        };

        reflector.register_type::<TestType>(type_info);

        assert!(reflector.type_info.contains_key(&TypeId::of::<TestType>()));
        let test_value = TestType;
        let result = reflector.get_type_info(&test_value as &dyn Any);
        assert!(result.is_some()); // must be true
        if let Some(info) = result {
            assert_eq!(info.name, "TestType");
        }

        struct Abc;
        let abc = Abc;
        let result = reflector.get_type_info(&abc as &dyn Any);
        assert!(!result.is_some()); // must be false
    }
}
