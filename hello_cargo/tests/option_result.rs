fn get_nth(items: &Vec<usize>, nth: usize) -> Option<usize> {
    if nth < items.len() {
        Some(items[nth])
    } else {
        None
    }
}

fn double(v: usize) -> usize {
    v * v
}

#[cfg(test)]
mod tests {
    use crate::{double, get_nth};

    #[test]
    fn test_get_nth() {
        let items = vec![1, 2, 3, 4, 5];
        println!("items:{}", items.len());
        let doubled = get_nth(&items, 4).map(double);
        println!("doubled:{}", doubled.unwrap());
        let doubled = get_nth(&items, 7).map(double);
        println!("doubled:{}", doubled.unwrap_or(0));
    }

    use std::string::FromUtf8Error;

    fn str_upper_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {
        let ret = String::from_utf8(str).map(|s| s.to_uppercase())?;
        println!("Conversion succeeded: {}", ret);
        Ok(ret)
    }

    #[test]
    fn test_str_upper_match() {
        let str = vec![b'h', b'e', b'l', b'l', b'o'];
        let ret = str_upper_match(str);
        println!("ret:{}", ret.unwrap());
        let valid_str = str_upper_match(vec![121, 97, 89]);
        println!("{:?}", valid_str);
    }
}
