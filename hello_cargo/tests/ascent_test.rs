#[cfg(test)]
mod tests{
    use ascent::ascent;
    ascent! {
       relation edge(i32, i32);
       relation path(i32, i32);
       
       path(x, y) <-- edge(x, y);
       path(x, z) <-- edge(x, y), path(y, z);
    }

    #[test]
    fn test_ascent(){
        let mut prog = AscentProgram::default();
        prog.edge = vec![(1, 4), (4, 5)];
        prog.run();
        println!("path: {:?}", prog.path);
    }
}