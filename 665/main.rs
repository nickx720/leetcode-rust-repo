impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut count = 0;
        let mut nums = nums;
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                if i == 1 || nums[i - 2] <= nums[i] {
                    nums[i - 1] = nums[i];
                    count += 1;
                } else {
                    nums[i] = nums[i - 1];
                    count += 1;
                }
            }
        }
        count <= 1
    }
}
