impl Solution {
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n1 = nums1.len();
        let n2 = nums2.len();
        let mut nums1_product: std::collections::HashMap<i64, i32> =
            std::collections::HashMap::new();
        let mut nums2_product: std::collections::HashMap<i64, i32> =
            std::collections::HashMap::new();
        for i in 0..n1 {
            for j in i + 1..n1 {
                let count = nums1_product
                    .entry(nums1[i] as i64 * nums1[j] as i64)
                    .or_insert(0);
                *count += 1;
            }
        }
        for i in 0..n2 {
            for j in i + 1..n2 {
                let count = nums2_product
                    .entry(nums2[i] as i64 * nums2[j] as i64)
                    .or_insert(0);
                *count += 1;
            }
        }
        for num in nums1 {
            let num = num as i64;
            let target = num.pow(2);
            if let Some(value) = nums2_product.get(&target) {
                ans += value;
            }
        }
        for num in nums2 {
            let num = num as i64;
            let target = num.pow(2);
            if let Some(value) = nums1_product.get(&target) {
                ans += value;
            }
        }
        ans
    }
}
