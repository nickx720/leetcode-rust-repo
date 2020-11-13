impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut output: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut words = words;
        words.sort_unstable_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
        for (i, w) in words.iter().enumerate() {
            for j in i + 1..words.len() {
                if words[j].contains(w) {
                    output.insert(w.to_string());
                }
            }
        }
        output.into_iter().collect::<Vec<String>>()
    }
}
