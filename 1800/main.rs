impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        if nums.len()== 1{
            return nums[0]
        }
        let mut output:Vec<Vec<i32>> = Vec::new();
        let mut index=0;
        for i in 0..nums.len()-1{
            if nums[i] >= nums[i+1] {
                output.push(nums[index..=i].to_vec());
                index=i+1;
            }
            if i == nums.len()-2 {
                output.push(nums[index..].to_vec());
            }

        }
        let mut output = output.iter().map(|x| x.iter().sum()).collect::<Vec<i32>>();
        output.sort();
        match output.last() {
            Some(&value)=> value,
            _=> unreachable!()
        }
    }
}
