#[cfg(test)]
mod tests {

    #[test]
    fn test_many_methods() {
        mod ext {
            pub trait Ext1 {
                fn is_palindrome(&self) -> bool;
            }

            pub trait Ext2 {
                fn is_palindrome(&self) -> bool;
            }

            impl Ext1 for str {
                fn is_palindrome(&self) -> bool {
                    self.chars().eq(self.chars().rev())
                }
            }

            impl Ext2 for str {
                fn is_palindrome(&self) -> bool {
                    self.chars().eq(self.chars().rev())
                }
            }
        }

        pub use ext::{Ext1, Ext2};

        // The compiler rejects the code because it cannot determine which method to invoke.
        // assert!("dad".is_palindrome());
    }

    #[test]
    fn test_method_conflict() {
        pub trait CountOnesExt {
            fn count_ones(&self) -> u32;
        }

        impl CountOnesExt for i32 {
            fn count_ones(&self) -> u32 {
                println!("end {self}");
                let value = *self;
                (0..32).filter(|i| ((value >> i) & 1i32) == 1).count() as u32
            }
        }

        // Which `count_ones` method is invoked?
        // The one from `CountOnesExt`? Or the inherent one from `i32`?
        assert_eq!((-1i32).count_ones(), 32);

        assert_eq!((&mut -1i32).count_ones(), 32);

        let i = &-1i32;
        assert_eq!(i.count_ones(), 32);
    }

    #[test]
    fn test_extend_foreign_type() {
        // 🛠️❌
        // impl str {
        //     pub fn is_palindrome(&self) -> bool {
        //         self.chars().eq(self.chars().rev())
        //     }
        // }

        pub trait StrExt {
            fn is_palindrome(&self) -> bool;
        }

        impl StrExt for str {
            fn is_palindrome(&self) -> bool {
                self.chars().eq(self.chars().rev())
            }
        }

        assert!("dad".is_palindrome());
        assert!(!"grandma".is_palindrome());
    }
}
