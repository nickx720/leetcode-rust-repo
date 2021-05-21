impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut max_count = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] >= nums[i + 1] {
                let diff = (nums[i + 1] - nums[i]).abs() + 1;
                max_count += diff;
                nums[i + 1] += diff;
            }
        }
        max_count
    }
}
