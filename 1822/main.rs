impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        if let Some(_) = nums.iter().find(|&&x| x == 0) {
            return 0;
        }
        match nums.iter().fold(0, |mut acc, &x| {
            if x < 0 {
                acc += 1;
                acc
            } else {
                acc
            }
        }) {
            num if num % 2 != 0 => -1,
            _ => 1,
        }
    }
}
