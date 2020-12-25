impl Solution {
    pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let mut sum: Vec<i32> = vec![0; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + a[i];
        }
        let mut count: Vec<i32> = vec![0; k as usize];
        for x in sum {
            count[((x % k + k) % k) as usize] += 1;
        }
        let mut ans = 0;
        for v in count {
            ans += v * (v - 1) / 2;
        }
        ans
    }
}
