// https://google.github.io/comprehensive-rust/idiomatic/polymorphism/refresher.html

#[cfg(test)]
mod tests {

    #[test]
    fn test_problem_solving() {
        // Question: What's the minimum useful behavior for a drawing API?
        pub trait DrawApi {
            fn arc(&self, center: [f32; 2], radius: f32, start_angle: f32, end_angle: f32);
            fn line(&self, start: [f32; 2], end: [f32; 2]);
        }

        pub struct TextDraw;

        impl DrawApi for TextDraw {
            fn arc(&self, center: [f32; 2], radius: f32, start_angle: f32, end_angle: f32) {
                println!("arc of radius ")
            }

            fn line(&self, start: [f32; 2], end: [f32; 2]) { /* ... */
            }
        }

        // Question: What's a good API for users?

        pub trait Draw {
            fn draw<T: DrawApi>(&self, surface: &mut T);
        }

        pub struct Rect {
            start: [f32; 2],
            end: [f32; 2],
        }

        impl Draw for Rect {
            fn draw<T: DrawApi>(&self, surface: &mut T) {
                surface.line([self.start[0], self.start[1]], [self.end[0], self.start[1]]);
                surface.line([self.end[0], self.start[1]], [self.end[0], self.end[1]]);
                surface.line([self.end[0], self.end[1]], [self.start[0], self.end[1]]);
                surface.line([self.start[0], self.end[1]], [self.start[0], self.start[1]]);
            }
        }
    }

    #[test]
    fn test_sealing_with_enums() {
        use std::collections::BTreeMap;
        pub enum GetSource {
            WebUrl(String),
            BytesMap(BTreeMap<String, Vec<u8>>),
        }

        impl GetSource {
            fn get(&self, url: &str) -> Option<&Vec<u8>> {
                match self {
                    Self::WebUrl(source) => unimplemented!(),
                    Self::BytesMap(map) => map.get(url),
                }
            }
        }
    }

    #[test]
    fn test_sealed_traits() {
        mod sealed {
            pub trait Sealed {}
            impl Sealed for String {}
            impl Sealed for Vec<u8> {}
            //...
        }

        pub trait APITrait: sealed::Sealed {
            /* methods */
        }
        impl APITrait for String {}
        impl APITrait for Vec<u8> {}
    }

    #[test]
    fn test_pitfall() {
        use std::any::Any;

        pub trait AddDyn: Any {
            fn add_dyn(&self, rhs: &dyn AddDyn) -> Box<dyn AddDyn>;
        }

        impl AddDyn for i32 {
            fn add_dyn(&self, rhs: &dyn AddDyn) -> Box<dyn AddDyn> {
                if let Some(downcast) = (rhs as &dyn Any).downcast_ref::<Self>() {
                    Box::new(self + downcast)
                } else {
                    Box::new(*self)
                }
            }
        }

        impl AddDyn for f32 {
            fn add_dyn(&self, rhs: &dyn AddDyn) -> Box<dyn AddDyn> {
                if let Some(downcast) = (rhs as &dyn Any).downcast_ref::<Self>() {
                    Box::new(self + downcast)
                } else {
                    Box::new(*self)
                }
            }
        }

        let i: &dyn AddDyn = &42;
        let j: &dyn AddDyn = &64;
        let k: Box<dyn AddDyn> = i.add_dyn(j);
        dbg!((k.as_ref() as &dyn Any).is::<i32>());
        dbg!((k.as_ref() as &dyn Any).downcast_ref::<i32>());

        let x: &dyn AddDyn = &64.0f32;
        let y: Box<dyn AddDyn> = x.add_dyn(j);
        dbg!((y.as_ref() as &dyn Any).is::<i32>());
        dbg!((y.as_ref() as &dyn Any).downcast_ref::<i32>());
        dbg!((y.as_ref() as &dyn Any).is::<f32>());
        dbg!((y.as_ref() as &dyn Any).downcast_ref::<f32>());
    }

    #[test]
    fn test_any_trait() {
        use std::any::Any;

        #[derive(Debug)]
        pub struct ThisImplementsAny;

        let is_an_any = ThisImplementsAny;

        let dyn_any: &dyn Any = &is_an_any;
        dbg!(dyn_any.type_id());
        dbg!(dyn_any.is::<ThisImplementsAny>());
        let is_downcast: Option<&ThisImplementsAny> = dyn_any.downcast_ref();
        dbg!(is_downcast);
    }
    #[test]
    fn test_dyn_heterogeneous() {
        // 异构
        use std::fmt::Display;
        pub struct Lambda;
        impl Display for Lambda {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "λ")
            }
        }

        let heterogeneous: Vec<Box<dyn Display>> = vec![
            Box::new(42u32),
            Box::new(String::from("Woah")),
            Box::new(Lambda),
        ];

        for item in heterogeneous {
            // We know "item" implements Display, but we know nothing else!
            println!("Display output: {}", item);
        }
    }

    #[test]
    fn test_trait_limit() {
        use std::any::Any;
        pub trait Trait: Any {}
        impl Trait for i32 {}

        dbg!(size_of::<i32>()); // 4 bytes, owned value
        dbg!(size_of::<&i32>()); // 8 bytes, reference
        dbg!(size_of::<&dyn Trait>()); // 16 bytes, wide pointer
    }

    #[test]
    fn test_dyn_vs_generics() {
        fn print_display<T: std::fmt::Display>(t: &T) {
            println!("{}", t);
        }

        fn print_display_dyn(t: &dyn std::fmt::Display) {
            println!("{}", t);
        }

        let int = 42i32;
        // Monomorphized to a unique function for i32 inputs.
        print_display(&int);
        // One per
        print_display_dyn(&int);
    }

    #[test]
    fn test_dyn_compatible() {
        pub trait Trait {
            // dyn compatible
            fn takes_self(&self);

            // dyn compatible, but you can't use this method when it's dyn
            fn takes_self_and_param<T>(&self, input: &T);

            // no longer dyn compatible
            const ASSOC_CONST: i32;

            // no longer dyn compatible
            fn clone(&self) -> Self;
        }
    }

    #[test]
    fn test_dyn_trait() {
        pub trait Trait {}

        impl Trait for i32 {}
        impl Trait for String {}

        let int: &dyn Trait = &42i32;
        let string: &dyn Trait = &String::from("Hello dyn!");
    }

    // https://google.github.io/comprehensive-rust/idiomatic/polymorphism/from-oop-to-rust.html

    #[test]
    fn test_monomorphization() {
        fn print_vec<T: std::fmt::Debug>(debug_vec: &Vec<T>) {
            for item in debug_vec {
                println!("{:?}", item);
            }
        }

        let ints = vec![1u32, 2, 3];
        let floats = vec![1.1f32, 2.2, 3.3];

        // instance one, &Vec<u32> -> ()
        print_vec(&ints);
        // instance two, &Vec<f32> -> ()
        print_vec(&floats);
    }

    #[test]
    fn test_size() {
        use std::fmt::Debug;

        pub struct AlwaysSized<T /* : Sized */>(T);

        pub struct OptionallySized<T: ?Sized>(T);

        type Dyn1 = OptionallySized<dyn Debug>;
    }

    #[test]
    fn test_trait_bound() {
        use std::fmt::Display;

        fn print_with_length<T: Display>(item: T) {
            println!("Item: {}", item);
            println!("Length: {}", item.to_string().len());
        }

        let number = 42;
        let text = "Hello, Rust!";

        print_with_length(number); // Works with integers
        print_with_length(text); // Works with strings
    }
}
