// https://google.github.io/comprehensive-rust/idiomatic/leveraging-the-type-system/token-types.html
#[cfg(test)]
mod tests {

    #[test]
    fn test_phantom_data_lifetime() {
        use std::marker::PhantomData;

        #[derive(Default)]
        struct InvariantLifetime<'id>(PhantomData<&'id ()>); // The main focus

        struct Wrapper<'a> {
            value: u8,
            invariant: InvariantLifetime<'a>,
        }

        fn lifetime_separator<T>(value: u8, f: impl for<'a> FnOnce(Wrapper<'a>) -> T) -> T {
            f(Wrapper {
                value,
                invariant: InvariantLifetime::default(),
            })
        }

        fn try_coerce_lifetimes<'a>(left: Wrapper<'a>, right: Wrapper<'a>) {
            println!("left {}, right {}", left.value, right.value)
        }

        {
            let a = lifetime_separator(1, |wrapped_1| {
                lifetime_separator(2, |wrapped_2| {
                    try_coerce_lifetimes(wrapped_1, wrapped_2);
                });
            });
            print!("{a:?}");
        }
    }

    #[test]
    fn test_var_check() {
        struct Bytes {
            bytes: Vec<u8>,
        }
        struct ProvenIndex(usize);

        impl Bytes {
            fn get_index(&self, ix: usize) -> Option<ProvenIndex> {
                if ix < self.bytes.len() {
                    Some(ProvenIndex(ix))
                } else {
                    None
                }
            }
            fn get_proven(&self, token: &ProvenIndex) -> u8 {
                unsafe { *self.bytes.get_unchecked(token.0) }
            }
        }

        let data_1 = Bytes {
            bytes: vec![0, 1, 2],
        };
        if let Some(token_1) = data_1.get_index(2) {
            data_1.get_proven(&token_1); // Works fine!

            let data_2 = Bytes {
                bytes: vec![0, 1, 3, 4],
            };
            data_2.get_proven(&token_1);

            // let data_2 = Bytes { bytes: vec![0, 1] };
            // data_2.get_proven(&token_1); // Panics! Can we prevent this?
        }
    }

    #[test]
    fn test_var_check_phantom_data<'a>() {
        use std::marker::PhantomData;

        #[derive(Default)]
        struct InvariantLifetime<'id>(PhantomData<*mut &'id ()>);
        struct ProvenIndex<'id>(usize, InvariantLifetime<'id>);

        struct Bytes<'id>(Vec<u8>, InvariantLifetime<'id>);

        impl<'id> Bytes<'id> {
            fn new<T>(
                // The data we want to modify in this context.
                bytes: Vec<u8>,
                // The function that uniquely brands the lifetime of a `Bytes`
                f: impl for<'a> FnOnce(Bytes<'a>) -> T,
            ) -> T {
                f(Bytes(bytes, InvariantLifetime::default()))
            }

            fn get_index(&self, ix: usize) -> Option<ProvenIndex<'id>> {
                if ix < self.0.len() {
                    Some(ProvenIndex(ix, InvariantLifetime::default()))
                } else {
                    None
                }
            }

            fn get_proven(&self, ix: &ProvenIndex<'id>) -> u8 {
                debug_assert!(ix.0 < self.0.len());
                unsafe { *self.0.get_unchecked(ix.0) }
            }
        }

        let data = vec![10, 20, 30, 40];

        // 使用 `Bytes::new` 创建带有特定生命周期的 `Bytes` 实例
        let result = Bytes::new(data, |bytes| {
            // 获取有效索引
            let ix1 = bytes.get_index(1).expect("index 1 should exist");
            let ix2 = bytes.get_index(3).expect("index 3 should exist");

            // 通过索引读取数据
            assert_eq!(bytes.get_proven(&ix1), 20);
            assert_eq!(bytes.get_proven(&ix2), 40);

            // 越界访问应返回 None
            assert!(bytes.get_index(4).is_none());
            assert!(bytes.get_index(10).is_none());

            // 闭包返回值（任意类型，此处为 u32）
            42u32
        });

        assert_eq!(result, 42);

        let result = Bytes::new(vec![4, 5, 1], |mut bytes_1| {
            Bytes::new(vec![4, 2], |mut bytes_2| {
                let index_1 = bytes_1.get_index(2).unwrap();
                let index_2 = bytes_2.get_index(1).unwrap();
                bytes_1.get_proven(&index_1);
                bytes_2.get_proven(&index_2);
                // bytes_2.get_proven(&index_1); // ❌🔨
                "Computations done!"
            })
        });
        println!("{result}");
    }

    #[test]
    fn test_check_perm() {
        mod admin {
            pub struct AdminToken(());

            pub fn get_admin(password: &str) -> Option<AdminToken> {
                if password == "Password123" {
                    Some(AdminToken(()))
                } else {
                    None
                }
            }
        }

        // We don't have to check that we have permissions, because
        // the AdminToken argument is equivalent to such a check.
        pub fn add_moderator(_: &admin::AdminToken, user: &str) {}

        if let Some(token) = admin::get_admin("Password123") {
            add_moderator(&token, "CoolUser");
        } else {
            eprintln!("Incorrect password! Could not prove privileges.")
        }

        // let a = admin::AdminToken(());  依赖这个 pub struct AdminToken(pub ());
    }

    #[test]
    fn test() {
        pub mod token {
            // A public type with private fields behind a module boundary.
            pub struct Token {
                proof: (), // 私有变量，无法暴露构造
            }

            pub fn get_token() -> Option<Token> {
                Some(Token { proof: () })
            }
        }

        pub fn protected_work(token: token::Token) {
            println!("We have a token, so we can make assumptions.")
        }

        if let Some(token) = token::get_token() {
            // We have a token, so we can do this work.
            protected_work(token);
        } else {
            // We could not get a token, so we can't call `protected_work`.
        }

        // let t = token::Token {};
    }
}
