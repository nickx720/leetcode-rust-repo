impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let value = nums.as_mut_slice();
        let mut i = value.len() - 1;
        while i >= 1 && value[i - 1] >= value[i] {
            i -= 1;
        }
        if i > 0 {
            // since we are calculating from right, find the first least value, not the highest if coming from the left
            let mut j = value.len() - 1;
            while value[j] <= value[i - 1] {
                j -= 1;
            }
            value.swap(i - 1, j);
        }
        value[i..].reverse();
    }
}
