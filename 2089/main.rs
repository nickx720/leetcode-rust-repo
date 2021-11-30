impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        nums.iter()
            .enumerate()
            .filter(|(_, &x)| x == target)
            .map(|(x, _)| x as i32)
            .collect::<Vec<_>>()
    }
}
