impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let characters1 = word1.chars().collect::<Vec<char>>();
        let characters2 = word2.chars().collect::<Vec<char>>();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; characters2.len() + 1]; characters1.len() + 1];
        for i in 0..characters1.len() + 1 {
            for j in 0..characters2.len() + 1 {
                if i == 0 {
                    dp[i][j] = j as i32;
                } else if j == 0 {
                    dp[i][j] = i as i32;
                } else if characters1[i - 1] == characters2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + std::cmp::min(dp[i][j - 1], dp[i - 1][j]);
                }
            }
        }
        dp[characters1.len()][characters2.len()]
    }
}
