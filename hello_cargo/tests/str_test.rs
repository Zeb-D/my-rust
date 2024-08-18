#[cfg(test)]
mod tests {

    fn get_str_literal() -> &'static str {
        "from function"
    }

    #[test]
    fn test_str() {
        let my_str: &str = "This is borrowed";
        let from_func = get_str_literal();
        println!("{} {}", my_str, from_func);
    }

    #[test]
    fn test_chars() {
        let my_str = String::from("Strings are cool");
        let first_three = &my_str[0..3];
        println!("{:?}", first_three);

        // 使用chars方法对字符串的所有字符进行迭代访问
        let hello = String::from("Hello");
        for c in hello.chars() {
            println!("{}", c);
        }
    }

    fn say_hello(to_whom: &str) {
        println!("Hey {}!", to_whom)
    }
    #[test]
    fn test_string2str() {
        let string_slice: &'static str = "you";
        let string: String = string_slice.into();
        // say_hello方法也适用于&String类型
        // 在内 部，&String会自动被强制转换为&str，因为String为str类型实现了类型强制性特征Deref，
        // 该特征确保了&String到&str的转换。
        say_hello(string_slice);
        say_hello(&string);
    }

    #[test]
    fn test_str_add_str() {
        let foo = "Foo";
        let bar = "Bar";
        // 涉及隐式的堆分配操作，该操作隐藏在运算符“+”后面。 Rust不鼓励隐式堆分配。
        // 通过显式地将第1个字 符串转换成包含所有权的字符串来实现两个字符串的拼接
        let baz = foo.to_string() + bar;
        println!("{}", baz)
    }

    const HEADER: &'static [u8; 4] = b"Obj\0";

    #[test]
    fn test_constants() {
        println!("{:?}", HEADER);
        println!("{:?}", HEADER.to_vec());
        println!("{}", String::from_utf8(HEADER.to_vec()).unwrap())
    }

    #[test]
    fn test_static() {
        // 静态值是相应的全局值，因为它们具有固定的内存位置，并且在整 个程序中作为单个(唯一)实例存在。
        // 读取和写入静态值都必须在某个unsafe代码块中完成。
        static mut BAZ: u32 = 4;
        static FOO: u8 = 9;
        unsafe {
            println!("baz is {}", BAZ);
            BAZ = 42;
            println!("baz is now {}", BAZ);
            println!("foo is {}", FOO);
        }
    }
}
