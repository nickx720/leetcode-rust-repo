impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut visited: Vec<bool> = vec![false; nums.len()];
        let mut res = 0;
        for i in 0..nums.len() {
            if !visited[i] {
                let mut start = nums[i] as usize;
                let mut count = 0;
                loop {
                    start = nums[start] as usize;
                    count += 1;
                    visited[i] = true;
                    if start as i32 == nums[i] {
                        break;
                    }
                }
                res = res.max(count);
            }
        }
        res
    }
}
