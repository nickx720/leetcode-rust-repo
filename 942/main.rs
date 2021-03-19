impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let n = s.len();
        let mut lo = 0;
        let mut hi = n as i32;
        let s = s.chars().collect::<Vec<char>>();
        let mut ans = vec![0; n + 1];
        for i in 0..n {
            if s[i] == 'I' {
                ans[i] = lo;
                lo += 1;
            } else {
                ans[i] = hi;
                hi -= 1;
            }
        }
        ans[n] = lo;
        ans
    }
}
