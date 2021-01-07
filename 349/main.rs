impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set_1: std::collections::HashSet<_> = nums1.iter().cloned().collect();
        let set_2: std::collections::HashSet<_> = nums2.iter().cloned().collect();
        let mut output: Vec<i32> = Vec::new();
        for &x in set_1.intersection(&set_2) {
            output.push(x);
        }
        output
    }
}
