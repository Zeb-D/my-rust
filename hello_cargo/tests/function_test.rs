// https://mp.weixin.qq.com/s/lGD6P2DyXNfzPjrihFBq6g Rust: 从简单函数到高级抽象，一步一步打造灵活的计算器
// 从一个简单的计算器开始，逐步引入Rust的高级特性，直至实现一个功能强大、灵活且可扩展的计算器。
// 无论是函数、枚举、结构体，还是traits、生命周期、泛型、闭包和异步编程，Rust的每一个特性都赋予了程序更高的灵活性和安全性。
#[cfg(test)]
mod tests {
    #[test]
    fn test_base_function() {
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        fn subtract(a: i32, b: i32) -> i32 {
            a - b
        }
        let x = 10;
        let y = 5;
        println!("{} + {} = {}", x, y, add(x, y));
        println!("{} - {} = {}", x, y, subtract(x, y));
    }

    enum Operation {
        Add,
        Subtract,
        Multiply,
        Divide,
    }

    #[test]
    fn test_enum_function() {
        fn calculate(a: i32, b: i32, operation: Operation) -> i32 {
            match operation {
                Operation::Add => a + b,
                Operation::Subtract => a - b,
                Operation::Multiply => a * b,
                Operation::Divide => {
                    if b != 0 {
                        a / b
                    } else {
                        eprintln!("Error: Division by zero is not allowed!");
                        0
                    }
                }
                _ => {
                    eprintln!("Error: Unsupported operation!");
                    0
                }
            }
        }

        let x = 20;
        let y = 4;
        println!("{} + {} = {}", x, y, calculate(x, y, Operation::Add));
        println!("{} - {} = {}", x, y, calculate(x, y, Operation::Subtract));
        println!("{} * {} = {}", x, y, calculate(x, y, Operation::Multiply));
        println!("{} / {} = {}", x, y, calculate(x, y, Operation::Divide));
    }

    #[test]
    fn test_struct_function() {
        struct Calculation {
            a: i32,
            b: i32,
            operation: Operation,
        }

        impl Calculation {
            fn calculate(&self) -> i32 {
                match self.operation {
                    Operation::Add => self.a + self.b,
                    Operation::Subtract => self.a - self.b,
                    Operation::Multiply => self.a * self.b,
                    Operation::Divide => {
                        if self.b != 0 {
                            self.a / self.b
                        } else {
                            eprintln!("Error: Division by zero!");
                            0
                        }
                    }
                }
            }
        }

        let calc1 = Calculation {
            a: 15,
            b: 3,
            operation: Operation::Add,
        };
        let calc2 = Calculation {
            a: 15,
            b: 3,
            operation: Operation::Divide,
        };
        println!("Result of calc1: {}", calc1.calculate());
        println!("Result of calc2: {}", calc2.calculate());
    }

    // 假设用户输入了操作符，我们可以用借用来避免不必要的拷贝，并确保数据的生命周期安全。Rust的生命周期（lifetime）机制会在编译时检查所有引用，
    // 确保它们在使用期间不会变得无效，从而避免潜在的悬挂引用（dangling references）。
    #[test]
    fn test_lifetime_function() {
        struct Calculation<'a> {
            a: i32,
            b: i32,
            operation: &'a Operation,
        }

        impl<'a> Calculation<'a> {
            fn calculate(&self) -> i32 {
                match self.operation {
                    Operation::Add => self.a + self.b,
                    Operation::Subtract => self.a - self.b,
                    Operation::Multiply => self.a * self.b,
                    Operation::Divide => {
                        if self.b != 0 {
                            self.a / self.b
                        } else {
                            eprintln!("Error: Division by zero!");
                            0
                        }
                    }
                    _ => {
                        eprintln!("Error: Unsupported operation!");
                        0
                    }
                }
            }
        }

        // 我们为operation字段加上了生命周期标注，确保其引用在Calculation对象的生命周期内有效。
        let op = Operation::Add;
        let calc = Calculation {
            a: 10,
            b: 2,
            operation: &op,
        };
        println!("Result: {}", calc.calculate());
        {
            let calc = Calculation {
                a: 10,
                b: 2,
                operation: &op,
            };
            println!("Result: {}", calc.calculate());
        }
        let calc = Calculation {
            a: 12,
            b: 2,
            operation: &op,
        };
        println!("Result: {}", calc.calculate());
    }

    // Rust的traits使得我们可以为不同类型定义共享行为，让我们的计算器可以处理多种不同的计算方式
    #[test]
    fn test_trait_function() {
        trait Calculator {
            fn calculate(&self) -> i32;
        }
        struct AddCalculator {
            a: i32,
            b: i32,
        }
        struct MultiplyCalculator {
            a: i32,
            b: i32,
        }
        impl Calculator for AddCalculator {
            fn calculate(&self) -> i32 {
                self.a + self.b
            }
        }
        impl Calculator for MultiplyCalculator {
            fn calculate(&self) -> i32 {
                self.a * self.b
            }
        }

        let add_calc = AddCalculator { a: 7, b: 3 };
        let mult_calc = MultiplyCalculator { a: 7, b: 3 };
        println!("Addition result: {}", add_calc.calculate());
        println!("Multiplication result: {}", mult_calc.calculate());
    }

    // 泛型使得代码更加灵活，可以处理不同类型的数据。
    #[test]
    fn test_generic_function() {
        use std::ops::{Add, Sub};

        struct Calculation<T> {
            a: T,
            b: T,
        }
        impl<T> Calculation<T>
        where
            T: Add<Output = T> + Sub<Output = T> + Copy,
        {
            fn add(&self) -> T {
                self.a + self.b
            }
            fn subtract(&self) -> T {
                self.a - self.b
            }
        }

        let int_calc = Calculation { a: 10, b: 5 };
        let float_calc = Calculation { a: 3.5, b: 1.2 };
        println!("Integer addition: {}", int_calc.add());
        println!("Float subtraction: {}", float_calc.subtract());
    }

    // Rust的闭包提供了极大的灵活性，用户可以根据需要定义自定义操作。
    // 通过闭包，用户可以动态地定义操作，计算器的灵活性大幅提升。接下来，使用迭代器和宏将使得计算器更加简洁高效。
    #[test]
    fn test_dyn_fn_function() {
        struct DynamicCalculation<'a> {
            a: i32,
            b: i32,
            operation: Box<dyn Fn(i32, i32) -> i32 + 'a>,
        }

        let add = |x, y| x + y;
        let multiply = |x, y| x * y;
        let calc1 = DynamicCalculation {
            a: 5,
            b: 3,
            operation: Box::new(add),
        };
        let calc2 = DynamicCalculation {
            a: 5,
            b: 3,
            operation: Box::new(multiply),
        };

        println!("Result of calc1: {}", (calc1.operation)(calc1.a, calc1.b));
        println!("Result of calc2: {}", (calc2.operation)(calc2.a, calc2.b));
    }

    macro_rules! operation {
        ($name:ident, $op:tt) => {
            fn $name(a: i32, b: i32) -> i32 {
                a $op b
            }
        };
    }
    // Rust的迭代器和宏是提升代码可读性和重用性的强大工具。通过迭代器，我们可以以声明式的方式处理数据，而宏则帮助我们自动化重复的代码模式。
    #[test]
    fn test_macro_rules_function() {
        operation!(add, +);
        operation!(subtract, -);
        operation!(multiply, *);
        operation!(aaa, ^);
        println!("3 + 2 = {}", add(3, 2));
        println!("3 - 2 = {}", subtract(3, 2));
        println!("3 ^ 2 = {}", aaa(3, 2));
    }

    use tokio::task;

    struct AsyncCalculation {
        a: i32,
        b: i32,
    }

    impl AsyncCalculation {
        // 使用 clone 或者将值传递给闭包
        async fn calculate(self) -> i32 {
            // 直接将 `self` 的所有权移入闭包
            task::spawn_blocking(move || self.a + self.b).await.unwrap()
        }
    }

    // 异步编程是Rust的又一亮点，它能够让计算任务在等待过程中不阻塞其他操作，提高程序的响应性和并发能力。
    #[tokio::test]
    async fn test_tokio_function() {
        let calc = AsyncCalculation { a: 10, b: 5 };
        let result = calc.calculate().await;
        println!("Calculation result: {}", result);
    }
}
