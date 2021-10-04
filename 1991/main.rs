impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        for index in 0..nums.len(){
            let left: i32 = nums[..index].iter().sum();
            let right :i32 = nums[index+1..].iter().sum();
            if left == right {
                return index as i32;
            }
        }
        -1
    }
}
