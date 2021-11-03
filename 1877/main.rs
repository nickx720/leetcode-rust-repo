impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut output = Vec::new();
        for i in 0..nums.len()/2{
            let pair = nums[i] + nums[nums.len()-1-i];
            output.push(pair);
        }
        *output.iter().max().unwrap()

    }
}
