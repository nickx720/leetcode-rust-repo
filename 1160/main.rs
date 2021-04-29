impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
         let calculate_char = |c: char| (c as u8 - 'a' as u8) as usize;
    let mut count = 0;
    for word in words {
        let mut char_freq: Vec<usize> = vec![0; 26];
        for character in chars.chars() {
            char_freq[calculate_char(character)] += 1;
        }
        count += word.len() as i32;
        for word_char in word.chars() {
            if char_freq[calculate_char(word_char)] > 0 {
                char_freq[calculate_char(word_char)] -= 1;
            } else {
                count -= word.len() as i32;
                break;
            }
        }
    }
    count
    }
}