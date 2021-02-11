impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let s: Vec<&str> = s.trim().split("").filter(|&c| c != "").collect();
        let mut combined: Vec<(i32, &str)> = indices.into_iter().zip(s.into_iter()).collect();
        combined.sort_by(|(a, _), (b, _)| a.cmp(&b));
        let output: String = combined.into_iter().map(|(_, a)| a).collect();
        output
    }
}
