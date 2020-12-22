impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let n = days.len();
        let mut dp: Vec<i32> = vec![0; n + 1];
        for i in (0..=n - 1).rev() {
            let mut d7 = i;
            let mut d30 = i;
            while d7 < n && days[d7] < days[i] + 7 {
                d7 += 1;
            }
            while d30 < n && days[d30] < days[i] + 30 {
                d30 += 1;
            }
            dp[i] = std::cmp::min(
                costs[0] + dp[i + 1],
                std::cmp::min(costs[1] + dp[d7], costs[2] + dp[d30]),
            )
        }
        dp[0]
    }
}
