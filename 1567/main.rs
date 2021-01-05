impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
       let (mut start_index, mut first_neg, mut count_neg, mut res) = (-1, -1, 0, 0);
    for i in 0..nums.len() {
        if nums[i] < 0 {
            count_neg += 1;
            if first_neg == -1 {
                first_neg = i as i32;
            }
        }
        if nums[i] == 0 {
            start_index = i as i32;
            first_neg = -1;
            count_neg = 0;
        }
        let index = i as i32;
        res = res.max(if count_neg % 2 == 0 {
            index - start_index
        } else {
            index - first_neg
        })
    }
    res 
    }
}