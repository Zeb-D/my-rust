#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    //
    fn read_number(path: &str) -> Result<i32, Box<dyn Error>> {
        // 使用 match 处理文件读取
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => return Err(Box::new(e)), // 将io::Error装箱
        };

        // 使用 match 处理解析
        match content.trim().parse::<i32>() {
            Ok(number) => Ok(number),
            Err(e) => Err(Box::new(e)), // 将ParseIntError装箱
        }
    }

    #[test]
    fn test_error() {
        let ret = read_number("lorem_ipsum.txt");
        println!("{:?}", ret);
        println!("{:?}", read_number(""));
        println!("origin error test")
    }
}
