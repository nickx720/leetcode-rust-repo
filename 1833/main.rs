impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut costs = costs;
    costs.sort();
    let mut total_cost = 0;
    for i in 0..costs.len() {
        let sum: i32 = costs[..=i].to_vec().iter().sum();
        if sum <= coins {
            total_cost += 1;
        } else {
            break;
        }
    }
    total_cost
    }
}