impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::new();
        for i in 0..prices.len() {
            let mut ii = if i + 1 < prices.len() {
                i + 1
            } else {
                output.push(prices[i]);
                break;
            };
            loop {
                if prices[ii] <= prices[i] {
                    output.push(prices[i] - prices[ii]);
                    break;
                } else if prices[ii] >= prices[i] && ii + 1 < prices.len() {
                    ii += 1;
                    continue;
                } else {
                    output.push(prices[i]);
                    break;
                }
            }
        }
        output
    }
}
