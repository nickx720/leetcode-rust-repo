impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut count = 0;
        for word in words {
            let mut all_chars = false;
            for chars in word.chars() {
                if allowed.contains(chars) {
                    all_chars = true;
                } else {
                    all_chars = false;
                    break;
                }
            }
            if all_chars {
                count += 1;
            }
        }
        count
    }
}
