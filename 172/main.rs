impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut count = 0;
        let mut i = 5;
        while n / i >= 1 {
            count += n / i;
            i *= 5;
        }
        count
    }
}
