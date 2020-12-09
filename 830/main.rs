impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut i = 0;
        let n = s.len();
        let s = s.chars().collect::<Vec<char>>();
        for j in 0..n {
            if j == n - 1 || s[j] != s[j + 1] {
                if j - i + 1 >= 3 {
                    result.push(vec![i as i32, j as i32]);
                }
                i = j + 1;
            }
        }
        result
    }
}
