impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut output :Vec<i32> = Vec::new();
        for index in 0..=nums.len() { 
            output.push(nums[0..index].iter().sum());
        }
        let output = output.iter().min().unwrap().abs()+1;
        output

    }
}
