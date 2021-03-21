impl Solution {
    fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == std::i32::MIN && divisor == -1 {
            std::i32::MAX
        } else {
            let mut a = dividend.abs();
            let b = divisor.abs();
            let mut result = 0;
            while a - b >= 0 {
                let mut x = 0;
                while a - (b << 1 << x) >= 0 {
                    x += 1;
                }
                result += 1 << x;
                a -= b << x;
            }
            if (dividend >= 0) == (divisor >= 0) {
                result
            } else {
                -result
            }
        }
    }
}
