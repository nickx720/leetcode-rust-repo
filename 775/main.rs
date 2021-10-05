impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        if nums.iter().enumerate().any(|(index,item)| index as i32 - item > 1 || index as i32 - item < -1 ) {
            false
        } else {
            true
        }

    }
}
