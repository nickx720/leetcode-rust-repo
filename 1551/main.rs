impl Solution {
    // https://medium.com/tech-life-fun/leetcode-1551-minimum-operations-to-make-array-equal-graphically-explained-python3-solution-99a6f71bbeec
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
