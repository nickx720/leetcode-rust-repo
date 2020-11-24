impl Solution {
    pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
        Self::at_most(&a, k) - Self::at_most(&a, k - 1)
    }
    fn at_most(a: &Vec<i32>, k: i32) -> i32 {
        let mut k = k;
        let mut count: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mut max_arr = 0;
        let mut start_window = 0;
        for end_window in 0..a.len() {
            let right_window = a[end_window];
            count.entry(right_window).or_insert(0);
            if let Some(val) = count.get_mut(&right_window) {
                if *val == 0 {
                    k -= 1;
                }
                *val += 1;
            }
            while k < 0 {
                let left_window = a[start_window];
                if let Some(val) = count.get_mut(&left_window) {
                    *val -= 1;
                    if *val == 0 {
                        k += 1;
                    }
                }
                start_window += 1;
            }
            max_arr += end_window - start_window + 1;
        }
        max_arr as i32
    }
}
