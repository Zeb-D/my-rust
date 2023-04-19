#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    macro_rules! add1{
        ($a :expr) => {$a};
        ($a:expr, $b:expr) =>{$a+$b};
        ($a:expr,$($b: tt)*) =>{
            $a + add1!($($b)*)
        }
    }

    macro_rules! make_public{
        (
            $(#[$meta: meta])*
            $vis: vis struct $struct_name: ident{
                $(
                    $field_vis: vis $field_name: ident: $field_type: ty
                ),*$(,)*
            }
        ) => {
                $(#[$meta])*
                pub struct $struct_name{
                    $(
                        pub $field_name: $field_type,
                    )*
                }
        }
    }

    macro_rules! convert {
        ($($x: tt)*) => {
            ()
        };
    }

    macro_rules! count {
        ($($x: expr), *) => {
            <[()]>::len(&[$(convert!($x)),*])
        };
    }

    macro_rules! hash_map {
        ($($key: expr => $value: expr),*$(,)?) => {
            {
                let __cap = count!($($key),*);
                println!("cap {}",__cap);
                let mut __map = HashMap::with_capacity(__cap);
                $(
                    __map.insert($key,$value);
                )*
                __map
            }
        };
    }

    make_public!(
        #[derive(Debug)]
        pub struct Test{
            a: i64,
            pub b: bool,
        }
    );

    //上面的等价于这种方式
    #[derive(Debug)]
    struct TestB{
        a: i64,
        pub b: bool,
    }


    #[test]
    fn hello() {
        println!("Cargo, Hello, world!");
    }

    #[test]
    fn add1() {
        println!("{}",add1!(3,1,3,6));
        println!("{}",add1!(1));
        println!("{}",add1!(3,6));
    }

    #[test]
    fn make_public() {
        let a = Test{
            a: 123,
            b: true,
        };
        println!("{:?},{:?}",a,a.a);
    
        let b = TestB{
            a: 321,
            b: false,
        };
        println!("{:?},{:?}",b,b.a);
    }
    
    #[test]
    fn hash_map() {
        let map = hash_map!(
            "yd" => 29,
            "yy" => 28,
        );
    
        for i in map{
            println!("{} {}",i.1, i.0);
        }
    
        let t = [(1,1),(2,2)];
        println!("{:?}",convert!(t));
        println!("{}",count!(t));
    }

}

// fn main() {
//     println!("Cargo, Hello, world!");
//     // cargo related command refer to: https://github.com/Zeb-D/my-review/blob/master/rust/cargo--浅学.md
//     println!("{}",add1!(3,1,3,6));
//     println!("{}",add1!(1));
//     println!("{}",add1!(3,6));

//     let a = Test{
//         a: 123,
//         b: true,
//     };
//     println!("{:?},{:?}",a,a.a);

//     let b = TestB{
//         a: 321,
//         b: false,
//     };
//     println!("{:?},{:?}",b,b.a);

//     let map = hash_map!(
//         "yd" => 29,
//         "yy" => 28,
//     );

//     for i in map{
//         println!("{} {}",i.1, i.0);
//     }

//     let t = [(1,1),(2,2)];
//     println!("{:?}",convert!(t));
//     println!("{}",count!(t));

// }
