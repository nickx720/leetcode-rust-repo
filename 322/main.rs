impl Solution {
    fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp: Vec<i32> = vec![amount + 1; (amount + 1) as usize];
        dp[0] = 0;
        for i in 1..=amount {
            for j in 0..coins.len() {
                if coins[j] <= i {
                    //    + 1 because we are currently using that coin gives the count
                    dp[i as usize] = std::cmp::min(dp[i as usize], dp[(i - coins[j]) as usize] + 1);
                }
            }
        }
        let output = if dp[amount as usize] == amount + 1 {
            -1
        } else {
            dp[amount as usize]
        };
        println!("{:?}", dp);
        output
    }
}
