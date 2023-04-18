mod common;


#[test]
fn test_add() {
    common::setup();
    // assert_eq!(src::add(3, 2), 5); //todo 这里怎么去调用src/xxx.rsd的代码？？
    
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    #[test]
    fn test_add() {
        assert_eq!(5 , 5);
    }
}