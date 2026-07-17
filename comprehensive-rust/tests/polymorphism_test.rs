// https://google.github.io/comprehensive-rust/idiomatic/polymorphism/refresher.html

#[cfg(test)]
mod tests {
    // https://google.github.io/comprehensive-rust/idiomatic/polymorphism/from-oop-to-rust.html

    #[test]
    fn test_monomorphization() {
        fn print_vec<T: std::fmt::Debug>(debug_vec: &Vec<T>) {
            for item in debug_vec {
                println!("{:?}", item);
            }
        }

        let ints = vec![1u32, 2, 3];
        let floats = vec![1.1f32, 2.2, 3.3];

        // instance one, &Vec<u32> -> ()
        print_vec(&ints);
        // instance two, &Vec<f32> -> ()
        print_vec(&floats);
    }

    #[test]
    fn test_size() {
        use std::fmt::Debug;

        pub struct AlwaysSized<T /* : Sized */>(T);

        pub struct OptionallySized<T: ?Sized>(T);

        type Dyn1 = OptionallySized<dyn Debug>;
    }

    #[test]
    fn test_trait_bound() {
        use std::fmt::Display;

        fn print_with_length<T: Display>(item: T) {
            println!("Item: {}", item);
            println!("Length: {}", item.to_string().len());
        }

        let number = 42;
        let text = "Hello, Rust!";

        print_with_length(number); // Works with integers
        print_with_length(text); // Works with strings
    }
}
