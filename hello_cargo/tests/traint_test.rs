#[cfg(test)]
mod tests {
    trait Driver {
        fn drive(&self) {
            println!("Driver's driving!");
        }
    }
    struct MyCar;
    impl MyCar {
        fn drive(&self) {
            println!("I'm driving!");
        }
    }
    impl Driver for MyCar {
        // fn drive(&self) {
        //     MyCar::drive(self);
        // }
    }

    fn drive(d:impl Driver) {
        d.drive()
    }

    // 范型化
    fn drive1<T: Driver>(d: T) {
        d.drive()
    }

    #[test]
    fn trait_test() {
        let car = MyCar;
        car.drive();
        let car1: &dyn Driver = &MyCar;
        car1.drive();

        drive(car);
        let car2: MyCar = MyCar;
        drive1(car2)
    }
}
