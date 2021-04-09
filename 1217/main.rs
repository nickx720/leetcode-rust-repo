impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let (mut even_count, mut odd_count) = (0, 0);
        for &item in &position {
            if item % 2 == 0 {
                even_count += 1;
            } else {
                odd_count += 1;
            }
        }
        std::cmp::min(even_count, odd_count)
    }
}
