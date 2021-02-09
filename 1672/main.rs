impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut inter_mediate: Vec<i32> = Vec::new();
        for account in &accounts {
            inter_mediate.push(account.into_iter().sum());
        }
        let mut highest = 0;
        for item in &inter_mediate {
            highest = std::cmp::max(highest, *item);
        }
        highest
    }
}
