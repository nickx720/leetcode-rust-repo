impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let (len, mut ans, mut i) = (nums.len(),1,1);
        while i < len && nums[i] == nums[i-1] {
            i+=1;
        }
        if i == len {
            return 1;
        }
        let mut up = nums[i-1] > nums[i];
        for i in i..len {
            if (up && nums[i] < nums[i-1]) || (!up && nums[i] > nums[i-1]){
                up = !up;
                ans+=1;
            }
        }
        ans

    }
}
