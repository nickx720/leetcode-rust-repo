impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let (length,mut left, mut right) = (nums.len()-1,-1,-1);
        let (mut max, mut min) = (nums[0],nums[length]);
        for i in 1..=length{
            let a = nums[i];
            let b = nums[length-i];
            if a < max {
                right = i as i32;
            } else {
                max = a;
            }
            if b > min {
                left = i as i32;
            } else {
                min = b;
            }
        }
        let length = length as i32;
        std::cmp::max(0, left+right-length +1)

    }
}
