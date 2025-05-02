#[cfg(test)]
mod tests {
    use anyhow::{Context, Result};

    fn read_number(path: &str) -> Result<i32> {
        let content = std::fs::read_to_string(path).context("文件读取失败")?;
        let number = content.trim().parse::<i32>().context("数值解析失败")?;
        Ok(number)
    }

    fn process_file(path: &str) {
        match read_number(path) {
            Ok(n) => println!("数值: {n}"),
            Err(e) => println!("处理文件{path}失败: {e}"),
        }
    }

    #[test]
    fn test_anyhow() {
        println!("{:?}",read_number(""));
        let ret = read_number("lorem_ipsum.txt");
        println!("{:?}",ret);
        println!("any how test")
    }
}
