impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let s = s.to_lowercase();
        let (first, second) = (&s[0..s.len() / 2], &s[s.len() / 2..]);
        let output = if Self::get_count(first) == Self::get_count(second) {
            true
        } else {
            false
        };
        output
    }

    fn get_count(s: &str) -> i32 {
        let mut count = 0;
        for item in s.chars() {
            match item {
                'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
                _ => continue,
            }
        }
        count
    }
}
