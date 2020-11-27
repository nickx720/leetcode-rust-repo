impl Solution {
    pub fn three_sum_multi(a: Vec<i32>, target: i32) -> i32 {
        let modulo = 10u32.pow(9) as i32 + 7;
        let mut ans = 0;
        let mut a = a;
        a.sort();
        for i in 0..a.len() {
            let t = target - a[i];
            let mut j = i + 1;
            let mut k = a.len() - 1;
            while j < k {
                if a[j] + a[k] < t {
                    j += 1;
                } else if a[j] + a[k] > t {
                    k -= 1;
                } else if a[j] != a[k] {
                    let mut left = 1;
                    let mut right = 1;
                    while j + 1 < k && a[j] == a[j + 1] {
                        left += 1;
                        j += 1;
                    }
                    while k - 1 > j && a[k] == a[k - 1] {
                        right += 1;
                        k -= 1;
                    }
                    ans += left * right;
                    ans %= modulo;
                    j += 1;
                    k -= 1;
                } else {
                    ans += ((k - j + 1) * (k - j)) as i32 / 2;
                    ans %= modulo;
                    break;
                }
            }
        }
        ans
    }
}
