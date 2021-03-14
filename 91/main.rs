impl Solution {
    fn num_decodings(s: String) -> i32 {
        let mut dp: Vec<i32> = vec![0; s.len() as usize + 1];
        let chars_as_string: Vec<char> = s.chars().collect();
        dp[0] = 1;
        dp[1] = if chars_as_string[0] == '0' { 0 } else { 1 };
        for index in 2..=s.len() {
            let one_digit: i32 = *&s[index - 1..index].parse().unwrap();
            let two_digit: i32 = *&s[index - 2..index].parse().unwrap();
            if one_digit >= 1 {
                dp[index] += dp[index - 1];
            }
            if two_digit >= 10 && two_digit <= 26 {
                dp[index] += dp[index - 2];
            }
        }
        dp[s.len()]
    }
}
