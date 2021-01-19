impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() < nums2.len() {
            return Self::intersect(nums2, nums1);
        }
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for x in nums1 {
            let count = map.entry(x).or_insert(0);
            *count += 1;
        }
        let mut output: Vec<i32> = Vec::new();
        for x in nums2 {
            if let Some(val) = map.get_mut(&x) {
                if *val > 0 {
                    output.push(x);
                    *val -= 1;
                }
            }
        }
        output
    }
}
