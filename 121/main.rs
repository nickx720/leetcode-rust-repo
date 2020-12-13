impl Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_val = std::i32::MAX;
        let mut max_val = 0;
        for index in 0..prices.len() {
            if prices[index] < min_val {
                min_val = prices[index];
            } else if (prices[index] - min_val) > max_val {
                max_val = prices[index] - min_val;
            }
        }
        max_val
    }
}
