#[cfg(test)]
mod tests {
    #[repr(C)]
    union Metric {
        rounded: u32,
        precise: f32,
    }

    #[test]
    fn union_c_test() {
        let mut a: Metric = Metric { rounded: 323 };
        unsafe {
            println!("{}", a.rounded);
        }
        unsafe {
            println!("{}", a.precise);
        }
        a.precise = 33.3;
        unsafe {
            println!("{}", a.precise);
        }

        // union expressions should have exactly one field
        // let mut a: Metric = Metric {
        //     rounded: 323,
        //     precise: 2.2,
        // };
    }
}
