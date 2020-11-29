impl Solution {
    fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut output = nums;
        output.sort();
        output.reverse();
        output[k as usize - 1]
    }
}
