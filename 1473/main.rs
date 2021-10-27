impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut count = k;
        for num in nums {
            if num == 1 {
                if count < k {
                    return false;
                }
                count = 0;
            } else {
                count+=1;
            }
        }
        true

    }
}
