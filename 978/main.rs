impl Solution {
    pub fn max_turbulence_size(a: Vec<i32>) -> i32 {
        let mut max_len = std::cmp::min(1, a.len() as i32);
    let mut dp: Vec<Vec<i32>> = vec![vec![1; 2]; a.len()];
    for i in 1..a.len() {
        if a[i - 1] < a[i] {
            dp[i][0] = dp[i-1][1] + 1;
        } else if a[i - 1] > a[i] {
            dp[i][1] = dp[i-1][0] + 1;
        }
        let max_dp = std::cmp::max(dp[i][0], dp[i][1]);
        max_len = std::cmp::max(max_len, max_dp);
    }
    max_len
    }
}