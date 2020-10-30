impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let steps = steps as usize;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; steps + 1]; steps + 1];
        for i in 1..=steps {
            let mut j: usize = 0;
            loop {
                if j <= i && j < arr_len as usize {
                    dp[i][j] = 1;
                } else {
                    break;
                }
                j += 1;
            }
        }
        let modu: i32 = (10u32.pow(9) + 7) as i32;
        for i in 2..=steps {
            let mut j: usize = 0;
            loop {
                if j <= i && j < arr_len as usize {
                    let left = if j == 0 { 0 } else { dp[i - 1][j - 1] };
                    let res = (left + dp[i - 1][j]) % modu;
                    let right = if j >= i || j >= arr_len as usize {
                        0
                    } else {
                        dp[i - 1][j + 1]
                    };
                    dp[i][j] = (res + right) % modu;
                } else {
                    break;
                }
                j += 1;
            }
        }
        dp[steps][0]
    }
}
