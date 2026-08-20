#[cfg(test)]
mod tests {
    // Rust — derive macros and declarative macros for code generation
    #[derive(Debug, Clone, PartialEq)] // Generates Debug, Clone, PartialEq impls at COMPILE time
    struct Point {
        x: f64,
        y: f64,
    }

    // Declarative macro (like a template)
    macro_rules! log_call {
        ($func_name:expr, $body:expr) => {{
            println!(
                "Calling {}, body-str:{}, body:{}",
                $func_name,
                stringify!($body),
                $body
            );
            $body
        }};
    }

    fn process(data: &str) -> String {
        log_call!("process", data.to_uppercase())
    }

    #[test]
    fn test() {
        let body = process("hello");
        println!("{}", body);
    }
}
