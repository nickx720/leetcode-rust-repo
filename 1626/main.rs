impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut data = ages
            .iter()
            .zip(scores.iter())
            .map(|v| v)
            .collect::<Vec<_>>();
        data.sort();
        let n = data.len();
        let mut dp = vec![0; n];
        for i in 0..n {
            let cur_score = data[i].1;
            dp[i] = *cur_score;
            for j in 0..i {
                if data[j].1 <= cur_score {
                    dp[i] = std::cmp::max(dp[i], dp[j] + cur_score)
                }
            }
        }
        if let Some(&output) = dp.iter().max() {
            output
        } else {
            0
        }
    }
}
