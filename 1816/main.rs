impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let s = s.trim().split(" ").collect::<Vec<&str>>();
        s[0..k as usize].join(" ")
    }
}
