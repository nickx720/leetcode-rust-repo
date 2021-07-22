impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        vec![nums.clone(),nums.clone()].into_iter().flatten().collect()

    }
}
