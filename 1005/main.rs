impl Solution {
    pub fn largest_sum_after_k_negations(a: Vec<i32>, k: i32) -> i32 {
        let mut a = a;
        let mut k = k;
        a.sort();
        let mut i = 0;
        while i < a.len() && k > 0 && a[i] < 0 {
            a[i] = -a[i];
            i += 1;
            k -= 1;
        }
        let mut min = a[0];
        let mut sum = 0;
        for num in a {
            sum += num;
            min = std::cmp::min(num, min);
        }
        if k % 2 == 0 {
            sum
        } else {
            sum - 2 * min
        }
    }
}
