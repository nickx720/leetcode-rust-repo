impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_by(|a, b| b.cmp(&a));
        let mut output: Vec<i32> = Vec::new();
        for item in 0..nums.len() {
            if output.len() == 0 {
                output.push(nums[item]);
                continue;
            }
            if output.iter().sum::<i32>() > nums[item..].iter().sum::<i32>() {
                break;
            }
            output.push(nums[item]);
        }
        output
    }
}
