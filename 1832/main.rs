impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let char_range = (97..123).collect::<Vec<u8>>();
        let mut sentence = sentence.chars().map(|x| x as u8).collect::<Vec<u8>>();
        sentence.sort();
        sentence.dedup();
        sentence == char_range
    }
}
