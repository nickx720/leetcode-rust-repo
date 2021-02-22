impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let word_1 = word1.split("").collect::<Vec<&str>>();
        let word_2 = word2.trim().split("").collect::<Vec<&str>>();
        let output: Vec<(&&str, &&str)> = word_1.iter().zip(word_2.iter()).collect();
        let remaining = if word1.len() > word2.len() {
            &word1[output.len() - 1..]
        } else if word2.len() > word1.len() {
            &word2[output.len() - 1..]
        } else {
            ""
        };
        let mut result = String::new();
        for item in output {
            result += item.0;
            result += item.1;
        }
        result += remaining;
        result
    }
}
