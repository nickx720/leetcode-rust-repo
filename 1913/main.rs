impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let (first_max,second_max) = (nums.pop().unwrap_or(0),nums.pop().unwrap_or(0));
        let (first_min,second_min) = (nums.remove(0),nums.remove(0));
        (first_max * second_max) - (first_min * second_min)

    }
}
