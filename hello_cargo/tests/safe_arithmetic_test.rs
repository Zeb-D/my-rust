#[cfg(test)]
mod tests {

    #[test]
    fn test_safe_arithmetic() {
        let foo: u32 = 5;
        let bar: i32 = 6;
        // let difference = foo - bar; // 类型不对直接报错
        let difference = foo as i32 - bar;
        println!("{}", difference);
    }
}
