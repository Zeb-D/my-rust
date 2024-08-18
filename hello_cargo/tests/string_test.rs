#[cfg(test)]
mod tests {

    #[test]
    fn test_string() {
        // Rust中有两种字符串:包含所有权的字符 串(String)和借用字符串(&str)
        // a通过调用 to_string方法创建字符串，该方法来自ToString特征
        let a: String = "Hello".to_string();
        // 通过调用from方法创建另 一个字符串b，这是String上的一个关联方法
        let b = String::from("hello");
        // c是通过 ToOwned特征的to_owned特征方法创建的，该特征是&str类型——基于 字面字符串而实现。
        let c = "world".to_owned(); 
        // 通过复制已有的字符串c创建的。创 建字符串的第4种方法开销昂贵，我们应该尽量避免采用这种方法，
        // 因 为它涉及通过迭代来复制底层字节。
        let d = c.clone();
        println!("{},{},{},{}", a, b, c, d)
    }

    #[test]
    fn test_all(){
        let mut empty_string = String::new();
        let empty_string_with_capacity = String::with_capacity(50);
        let string_from_bytestring: String = String::from_utf8(vec![82, 85, 83,84]).expect("Creating String from bytestring failed");
        println!("Length of the empty string is {}", empty_string.len());
        println!("Length of the empty string with capacity is {}",
        empty_string_with_capacity.len());
        println!("Length of the string from a bytestring is {}",
        string_from_bytestring.len());
        println!("Bytestring says {}", string_from_bytestring);
        empty_string.push('1');
        println!("1) Empty string now contains {}", empty_string);
        empty_string.push_str("2345");
        println!("2) Empty string now contains {}", empty_string);
        println!("Length of the previously empty string is now {}",
        empty_string.len());
    }
}
