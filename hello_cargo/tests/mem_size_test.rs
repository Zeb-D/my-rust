#[cfg(test)]
mod tests {
    use std::ops::Drop;

    struct A {
        a: u32,
        b: Box<u64>,
    }

    struct B(i32, f64, char);

    struct N;

    enum E {
        H(u32),
        M(Box<u32>),
    }

    union U {
        u: u32,
        v: u64,
    }

    #[test]
    fn test_size() {
        println!("Box<u64>: {:?}", std::mem::size_of::<Box<u64>>()); // Box<u64>: 8
        println!("A: {:?}", std::mem::size_of::<A>()); //A: 16
        println!("B: {:?}", std::mem::size_of::<B>()); //B: 16
        println!("N: {:?}", std::mem::size_of::<N>()); //N: 0
        println!("E: {:?}", std::mem::size_of::<E>()); //E: 16
        println!("U: {:?}", std::mem::size_of::<U>()); //U: 8
    }

    #[test]
    fn test_ref_cell() {
        use std::cell::Cell;
        use std::cell::RefCell;
        use std::rc::Rc;

        println!("type u8: {}", std::mem::size_of::<u8>());
        println!("type f64: {}", std::mem::size_of::<f64>());
        println!("value 4u8: {}", std::mem::size_of_val(&4u8));
        println!("value 4: {}", std::mem::size_of_val(&4));
        println!("value 'a': {}", std::mem::size_of_val(&'a'));
        println!(
            "value \"Hello World\" as a static str slice: {}",
            std::mem::size_of_val("Hello World")
        );
        println!(
            "value \"Hello World\" as a String: {}",
            std::mem::size_of_val("Hello World").to_string()
        );
        println!("Cell(4)): {}", std::mem::size_of_val(&Cell::new(84)));
        println!("RefCell(4)): {}", std::mem::size_of_val(&RefCell::new(4)));
        println!("Rc(4): {}", std::mem::size_of_val(&Rc::new(4)));
        println!(
            "Rc<RefCell(8)>): {}",
            std::mem::size_of_val(&Rc::new(RefCell::new(4)))
        );
    }

    trait Position {}
    struct Coordinates(f64, f64);
    impl Position for Coordinates {}

    #[test]
    fn test_pointer() {
        //特征对象和指向特征的引用是胖指针，它们是普通指针大小的两倍。
        let val = Coordinates(1.0, 2.0);
        let ref_: &Coordinates = &val;
        let pos_ref: &dyn Position = &val as &dyn Position;
        let ptr: *const Coordinates = &val as *const Coordinates;
        let pos_ptr: *const dyn Position = &val as *const dyn Position;
        println!("ref_: {}", std::mem::size_of_val(&ref_)); // 8
        println!("ptr: {}", std::mem::size_of_val(&ptr)); // 8
        println!("val: {}", std::mem::size_of_val(&val)); // 16
        println!("pos_ref: {}", std::mem::size_of_val(&pos_ref)); // 16
        println!("pos_ptr: {}", std::mem::size_of_val(&pos_ptr)); // 16
    }
}
