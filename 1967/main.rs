impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns.iter().filter(|each_word| word.contains(*each_word)).count() as i32

    }
}
