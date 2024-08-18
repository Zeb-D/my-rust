#[cfg(test)]
mod tests {

    const fn const_fn(a: u32) -> u32 {
        42 * a
    }

    #[test]
    fn const_fn_test() {
        const doubble42: u32 = const_fn(2);
        assert_eq!(doubble42, 84);
        assert_eq!(const_fn(3), 126);
    }

    const fn read_header(a: &[u8]) -> (u8, u8, u8, u8) {
        (a[0], a[1], a[2], a[3])
    }

    const FILE_HEADER: (u8, u8, u8, u8) = read_header(include_bytes!("./fn_test.rs"));

    #[test]
    fn const_fn_file_test() {
        println!("{:?}", FILE_HEADER);
        print!("{}", String::from_utf8_lossy(include_bytes!("./fn_test.rs")));
    }
}
