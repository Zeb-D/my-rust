#[cfg(test)]
mod tests {
    #[test]
    fn test_scopes() {
        // 使用裸代码块同时执行多个任务
        let precompute = {
            let a = (-34i64).abs();
            let b = 345i64.pow(3);
            let c = 3;
            a + b + c
        };
        println!("{}", precompute);
        // match表达式
        let result_msg = match precompute {
            42 => "done",
            a if a % 2 == 0 => "continue",
            _ => panic!("Oh no !"),
        };
        println!("{}", result_msg);
    }

    #[derive(Debug)]
    struct Items(u32);
    #[test]
    fn test_scopes_ref() {
        let items = Items(42);
        println!("{:?}", items);
        let items_ptr = &items;
        let ref items_ref = items;
        assert_eq!(items_ptr as *const Items, items_ref as *const Items);

        let mut a = Items(20);
        // 通过作用域将b对a的改动限制在内部代码块中
        {
            // 也可以像这样使用可变引用
            let ref mut b = a; // same as:
            let b = &mut a;
            b.0 += 25;
        }
        println!("{:?}", items);
        println!("{:?}", a);
    }

    struct Person(String);
    #[test]
    fn test_scopes_match() {
        let a = Person("Richard Feynman".to_string());
        match a {
            Person(ref name) => println!("{} was a great physicist !", name),
            _ => panic!("Oh no !"),
        }
        let b = a;
    }

    struct  Container{
        items_count: u32
    }
    fn increment_item(Container {mut items_count}: &mut Container) {
        items_count += 1;
    }
    fn calculate_cost(Container {items_count}: &Container) -> u32 {
        let rate = 2;
        rate * items_count
    }
    #[test]
    fn test_scopes_destructure() {
        let mut container = Container {
            items_count: 10
        };
        increment_item(&mut container);
        let total_cost = calculate_cost(&container);
        println!("Total cost: {}", total_cost);
    }
}
