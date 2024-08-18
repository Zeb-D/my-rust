#[cfg(test)]
mod tests {
    #[test]
    fn test_iterator() {
        let v = vec![1, 2, 3];
        for vi in v {
            println!("{}", vi)
        }
        use rayon::prelude::*;
        let v: Vec<_> = (0..10).collect();
        v.par_iter().for_each(|&vi| println!("{}", vi));
    }

    use std::usize;
    struct Primes { // 素数
        limit: usize,
    }
    struct PrimesIter {
        index: usize,
        computed: Vec<bool>,
    }

    impl Primes {
        fn iter(&self) -> PrimesIter {
            PrimesIter {
                index: 2,
                computed: compute_primes(self.limit),
            }
        }
        fn new(limit: usize) -> Primes {
            Primes { limit }
        }
    }
    fn compute_primes(limit: usize) -> Vec<bool> {
        let mut sieve = vec![true; limit];
        let mut m = 2;
        while m * m < limit {
            if sieve[m] {
                for i in (m * 2..limit).step_by(m) {
                    sieve[i] = false;
                }
            }
            m += 1;
        }
        sieve
    }

    impl Iterator for PrimesIter {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            loop {
                self.index += 1;
                if self.index > self.computed.len() - 1 {
                    return None;
                } else if self.computed[self.index] {
                    return Some(self.index);
                } else {
                    continue;
                }
            }
        }
    }

    #[test]
    fn custome_iterator_test() {
        let primes = Primes::new(100);
        for i in primes.iter() {
            print!("{},", i);
        }
    }
}
