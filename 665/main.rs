impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut count = 0;
        let mut nums = nums;
        for i in 1..nums.len() {
            // Check if current element is less than previous element
            if nums[i] < nums[i - 1] {
                // if current element is greater than [i-2], make the current element equal to previous element, to ensure increasing order
                if i == 1 || nums[i - 2] <= nums[i] {
                    nums[i - 1] = nums[i];
                    count += 1;
                } else { // current element is equal to previous element
                    nums[i] = nums[i - 1];
                    count += 1;
                }
            }
        }
        count <= 1
    }
}
