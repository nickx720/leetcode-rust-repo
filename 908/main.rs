impl Solution {
    pub fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
        use std::cmp::{max,min};
        let (mut max_value, mut min_value) = (a[0],a[0]);
        for &val in a.iter().skip(1) {
            min_value = min(min_value, val);
            max_value = max(val, max_value);
        }
        if min_value + k >= max_value - k {
            0
        }else {
            (max_value-k) - (min_value + k)
        }

    }
}
