// https://google.github.io/comprehensive-rust/idiomatic/foundations-api-design/predictable-api/common-traits.html

#[cfg(test)]
mod tests {

    #[test]
    fn test_serde() {
        // #[derive(Serialize, Deserialize)]
        // struct ExtraData {
        //     fav_color: String,
        //     name_of_dog: String,
        // }
        //
        // #[derive(Serialize, Deserialize)]
        // struct Data {
        //     name: String,
        //     age: usize,
        //     extra_data: ExtraData,
        // }
    }

    #[test]
    fn test_from() {
        pub struct Wrapper(String);

        impl From<&str> for Wrapper {
            fn from(value: &str) -> Self {
                Wrapper(value.to_owned())
            }
        }

        impl From<i32> for Wrapper {
            fn from(value: i32) -> Self {
                Wrapper(value.to_string())
            }
        }

        // `Into` is more natural to use as a trait bound.
        fn into_string<S: Into<String>>(s: S) {}
        fn string_from<T>(t: T)
        where
            String: From<T>,
        {
        }

        // `Wrapper` can be construct from `&str` and `i32`.
        let a = Wrapper::from("Hello, obvious!");
        let b = Wrapper::from(-123);

        // A From implementation implies an Into implementation.
        let c: Wrapper = "Hello, implementation!".into();

        #[derive(Debug)]
        pub struct InvalidNumber;

        #[derive(Debug)]
        pub struct DivisibleByTwo(usize);

        impl TryFrom<usize> for DivisibleByTwo {
            type Error = InvalidNumber;

            fn try_from(value: usize) -> Result<Self, InvalidNumber> {
                if value.rem_euclid(2) == 0 {
                    Ok(DivisibleByTwo(value))
                } else {
                    Err(InvalidNumber)
                }
            }
        }

        let success: Result<DivisibleByTwo, _> = 4.try_into();
        dbg!(success);
        let fail: Result<DivisibleByTwo, _> = 5.try_into();
        dbg!(fail);
    }

    #[test]
    fn test_copy() {
        #[derive(Debug, Clone, Copy)]
        pub struct Copyable(u8, u16, u32, u64);
        let copyable = Copyable(1, 2, 3, 4);
        let copy = copyable; // Implicit copy operation
        dbg!(copyable);
        dbg!(copy);
    }

    #[test]
    fn test_clone() {
        use std::collections::BTreeSet;
        use std::rc::Rc;

        #[derive(Clone)]
        pub struct LotsOfData<'a> {
            string: String,
            vec: &'a Vec<u8>,
            set: BTreeSet<u8>,
        }

        let lots_of_data = LotsOfData {
            string: "String".to_string(),
            vec: &vec![1; 255],
            set: BTreeSet::from_iter([1, 2, 3, 4, 5, 6, 7, 8]),
        };

        // Deep copy of all of the data in `lots_of_data`.
        let lots_of_data_cloned = lots_of_data.clone();

        let reference_counted = Rc::new(lots_of_data);

        // Copies the reference-counted pointer, not the value.
        let reference_copied = reference_counted.clone();
    }

    #[test]
    fn test_hash() {
        use std::collections::HashMap;

        #[derive(PartialEq, Eq, Hash)]
        pub struct User {
            id: u32,
            name: String,
        }

        let user = User {
            id: 1,
            name: "Alice".into(),
        };
        let mut map = HashMap::new();
        map.insert(user, "value");
    }

    #[test]
    fn test_ord() {
        #[derive(PartialEq, PartialOrd)]
        pub struct Partially(f32);

        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        pub struct Totally {
            id: u32,
            name: String,
        }

        let a = Totally {
            id: 0,
            name: "alice".into(),
        };
        let b = Totally {
            id: 1,
            name: "alice".into(),
        };
        let c = Totally {
            id: 0,
            name: "charlie".into(),
        };

        dbg!(a.cmp(&b));
        dbg!(a.cmp(&c));
    }

    #[test]
    fn test_eq() {
        #[derive(PartialEq, Eq)]
        pub struct User {
            name: String,
            favorite_number: i32,
            time: i32,
        }
        let alice = User {
            name: "alice".to_string(),
            favorite_number: 1_000_042,
            time: 0,
        };
        let bob = User {
            name: "bob".to_string(),
            favorite_number: 42,
            time: 2,
        };
        let bob1 = User {
            name: "bob".to_string(),
            favorite_number: 42,
            time: 1,
        };

        dbg!(alice == alice);
        dbg!(alice == bob);
        dbg!(bob1 == bob);

        #[derive(Debug)]
        pub struct People {
            name: String,
            favorite_number: i32,
            time: i32,
        }

        // 手动实现 PartialEq：只比较 name 和 favorite_number，忽略 time
        impl PartialEq for People {
            fn eq(&self, other: &Self) -> bool {
                self.name == other.name && self.favorite_number == other.favorite_number
            }
        }

        // 如果这种相等关系满足自反、对称、传递，可以标记 Eq
        impl Eq for People {}

        let alice = People {
            name: "alice".to_string(),
            favorite_number: 1_000_042,
            time: 0,
        };
        let bob = People {
            name: "bob".to_string(),
            favorite_number: 42,
            time: 2,
        };
        let bob1 = People {
            name: "bob".to_string(),
            favorite_number: 42,
            time: 1,
        };

        dbg!(alice == alice); // true (相同字段)
        dbg!(alice == bob); // false (name/favorite_number 不同)
        dbg!(bob1 == bob); // true (因为只比较 name 和 favorite_number，忽略 time)
    }

    #[test]
    fn test_display() {
        #[derive(Debug)]
        pub enum NetworkError {
            HttpCode(u16),
            WhaleBitTheUnderseaCable,
        }

        impl std::fmt::Display for NetworkError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    NetworkError::HttpCode(code) => write!(f, "HTTP Error code {code}"),
                    NetworkError::WhaleBitTheUnderseaCable => {
                        write!(f, "Whale attack detected – call Ishmael")
                    }
                }
            }
        }

        let http = NetworkError::HttpCode(404);
        let whale = NetworkError::WhaleBitTheUnderseaCable;

        println!("http debug: {:?}", http);
        println!("http display: {}", http);
        println!("whale debug: {:?}", whale);
        println!("whale display: {}", whale);

        let num = 42;
        println!("Hex: {:x}, Binary: {:b}, Ox: {:o}", num, num, num);
        // 输出: Hex: 2a, Binary: 101010

        let pi = 3.14159;
        println!("{:.2}", pi); // 输出: 3.14
    }

    #[test]
    fn test_debug() {
        #[derive(Debug)]
        pub struct Date {
            day: u8,
            month: u8,
            year: i64,
        }

        #[derive(Debug)]
        pub struct User {
            name: String,
            date_of_birth: Date,
        }

        pub struct PlainTextPassword {
            password: String,
            hint: String,
        }

        impl std::fmt::Debug for PlainTextPassword {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("PlainTextPassword")
                    .field(r#"hint"#, &self.hint)
                    .field("password", &"[omitted]")
                    .finish()
            }
        }

        let user = User {
            name: "Alice".to_string(),
            date_of_birth: Date {
                day: 31,
                month: 10,
                year: 2002,
            },
        };

        println!("{user:?}");
        println!("{user:#?}");
        println!(
            "{:?}",
            PlainTextPassword {
                password: "Password123".to_string(),
                hint: "Used it for years".to_string()
            }
        );
    }
}
