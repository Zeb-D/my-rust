pub mod adder{

    // Define this in a crate called `adder`.
    pub fn add(a: i32, b: i32) -> i32 {
    a + b
    }
}

#[cfg(test)]
mod tests {
    use crate::adder;
    use pretty_assertions::assert_eq; // crate for test-only use. Cannot be used in non-test code.

    #[test]
    fn test_add() {
        assert_eq!(adder::add(2, 3), 5);
    }
}