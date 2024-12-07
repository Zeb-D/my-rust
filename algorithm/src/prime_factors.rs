// 混合素数分解：用于将一个数分解为素数因子的算法，可以利用Rust的内存安全和性能优势高效地运行。

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut divisor = 2;
    while n > 1 {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }
        divisor += 1;
        if divisor * divisor > n && n > 1 {
            factors.push(n);
            break;
        }
    }
    factors
}
