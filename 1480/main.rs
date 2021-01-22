impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        let mut output = vec![0; nums.len()];
        for i in 0..nums.len() {
            output[i] = nums[i] + sum;
            sum += nums[i];
        }
        output
    }
}
