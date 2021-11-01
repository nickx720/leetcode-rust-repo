impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        for index in 0..nums.len(){
            if index % 10 == nums[index] as usize{
                return index as i32
            }
        }
        -1

    }
}
