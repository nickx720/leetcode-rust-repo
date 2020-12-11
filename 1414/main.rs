impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut ve: Vec<i32> = vec![1; 1];
        let mut f = 1;
        let mut s = 1;
        loop {
            if f + s > k {
                break;
            }
            ve.push(f + s);
            f = s;
            s = *ve.last().unwrap();
        }
        let mut k = k;
        let mut ans = 0;
        for i in (0..=ve.len() - 1).rev() {
            if ve[i] <= k {
                ans += 1;
                k -= ve[i]
            }
            if k == 0 {
                return ans;
            }
        }
        ans
    }
}
