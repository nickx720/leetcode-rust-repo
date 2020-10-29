use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
    if n <= 1 {
        return 0;
    }
    let mut no_stocks: Vec<i32> = vec![0; n];
    let mut in_hand: Vec<i32> = vec![0; n];
    let mut sold: Vec<i32> = vec![0; n];
    no_stocks[0] = 0;
    sold[0] = 0;
    in_hand[0] = -prices[0];
    for i in 1..n {
        no_stocks[i] = max(no_stocks[i - 1], sold[i - 1]);
        in_hand[i] = max(in_hand[i - 1], no_stocks[i-1] - prices[i]);
        sold[i] = in_hand[i - 1] + prices[i]
    }
    max(no_stocks[n - 1], sold[n - 1])
    }
}