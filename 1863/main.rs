impl Solution {
    // https://leetcode.com/problems/sum-of-all-subset-xor-totals/discuss/1211177/C%2B%2B-simple-solution-oror-5-lines-of-code-oror-Explained!!
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len() as u32;
        let mut res = 0;
        nums.iter().for_each(|&item| {
            res |= item;
        });
        res = res * 2i32.pow(n - 1);
        res
    }
}
