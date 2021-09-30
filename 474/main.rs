impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let (m,n) = (m as usize, n as usize);
    let mut dp:Vec<Vec<i32>> = vec![vec![0;101];101];
    for item in strs{
        let (mut zero, mut one) = (0,0);
        for ch in item.chars(){
            if ch == '0'{
                zero+=1;
            }else {
                one+=1;
            }
        }
        for i in (zero..=m).rev(){
            for j in (one..=n).rev(){
                dp[i][j] = std::cmp::max(dp[i][j],dp[i-zero][j-one]+1);
            }
        }
    }
    dp[m][n]

    }
}
/// https://dev.to/seanpgallivan/solution-ones-and-zeroes-2emf
