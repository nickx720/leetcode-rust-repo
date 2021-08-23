impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        let mut dp:Vec<bool> = vec![false;n as usize + 1];
        Self::helper(n,&mut dp)

    }
    fn helper(n:i32,dp: &mut Vec<bool>)-> bool {
        if n == 1 {
            return false;
        }
        if dp[n as usize] != false {
            return dp[n as usize];
        }else {
            for i in 1..n {
                if n % i == 0 {
                    if Self::helper(n-1,dp) == false {
                        dp[n as usize] = true;
                        return dp[n as usize];
                    }
                }
                dp[n as usize] = false;
                return dp[n as usize];
            }
        }
        unreachable!()
    }
}
