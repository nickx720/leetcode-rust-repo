impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let len = nums.len();
        for (ind, item) in nums.clone().into_iter().enumerate() {
            for check in ind + 1..len {
                if item == nums[check] {
                    count += 1;
                }
            }
        }
        count
    }
}
