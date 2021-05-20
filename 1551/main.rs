impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let mut median = n / 2;
        let mut count = 0;
        while median != 0 {
            median -= 1;
            count += n - (median * 2 + 1);
        }
        count
    }
}
