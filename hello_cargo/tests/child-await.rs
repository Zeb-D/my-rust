#[cfg(test)]
mod tests {
    use std::process::Command;
    #[test]
    fn main() {
        let mut child = Command::new("sleep").arg("5").spawn().unwrap();
        println!("reached start of main");
        let _result = child.wait().unwrap();

        println!("reached end of main");
        Command::new("sleep").arg("5").output().unwrap_or_else(|e| {
            panic!("await to execute process: {}", e) //立即等结果
        });
    }
}