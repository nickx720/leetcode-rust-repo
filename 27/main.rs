impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut output: i32 = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[output as usize] = nums[i];
                output += 1;
            }
        }
        output as i32
    }
}
