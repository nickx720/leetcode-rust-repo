impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let (mut buying, mut selling) = (0,-prices[0]);
        for item in prices.iter().skip(1){
            buying = std::cmp::max(buying,selling+item-fee);
            selling = std::cmp::max(selling,buying-item);
        }
        buying

    }
}
