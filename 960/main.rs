impl Solution {
    pub fn min_deletion_size(a: Vec<String>) -> i32 {
        let len = a[0].len();
        let mut dp: Vec<i32> = vec![1; len];
        let mut res = 1;
        for i in 0..len {
            for j in 0..i {
                let mut flag: bool = false;
                for item in a.iter() {
                    let item = item.chars().collect::<Vec<char>>();
                    if item[i] >= item[j] {
                        flag = true;
                        continue;
                    }
                    flag = false;
                    break;
                }
                if flag {
                    dp[i] = std::cmp::max(dp[i], 1 + dp[j]);
                    res = std::cmp::max(res, dp[i]);
                }
            }
        }
        let len = len as i32;
        len - res
    }
}
