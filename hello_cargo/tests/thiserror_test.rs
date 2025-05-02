#[cfg(test)]
mod tests {
    use thiserror::Error;

    #[derive(Error, Debug)]
    enum MyError {
        #[error("文件读取失败: {0}")]
        Io(#[from] std::io::Error),
        #[error("数值解析错误: {0}")]
        ParseInt(#[from] std::num::ParseIntError),
    }

    fn read_number(path: &str) -> Result<i32, MyError> {
        let content = std::fs::read_to_string(path)?;
        let number = content.trim().parse::<i32>()?;
        Ok(number)
    }

    #[test]
    fn test_thiserror(){
        println!("{:?}",read_number(""));
        let ret = read_number("lorem_ipsum.txt");
        println!("{:?}",ret);
        println!("this error test");
    }
}
