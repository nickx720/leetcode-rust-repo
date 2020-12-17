impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let n = s.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; n];
        let s = s.chars().collect::<Vec<char>>();
        for i in (0..n - 1).rev() {
            for j in i + 1..n {
                if s[i] == s[j] {
                    dp[i][j] = dp[i + 1][j - 1];
                } else {
                    dp[i][j] = std::cmp::min(dp[i + 1][j], dp[i][j - 1]) + 1;
                }
            }
        }
        dp[0][n - 1]
    }
}
