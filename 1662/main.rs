impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let word1 = word1.join("");
        let word2 = word2.join("");
        word1 == word2
    }
}
