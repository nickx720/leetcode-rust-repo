impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        let char_to_index = |c: char| (c as u8 - 'a' as u8) as usize;
        let index_to_string = |c: usize| char::from(c as u8 + 'a' as u8).to_string();
        let mut min_freq: Vec<u8> = vec![std::u8::MAX; 26];
        for each_str in a {
            let mut char_freq = vec![0; 26];
            for item in each_str.chars() {
                let index = char_to_index(item);
                char_freq[index] += 1;
            }
            for index in 0..min_freq.len() {
                min_freq[index] = std::cmp::min(min_freq[index], char_freq[index]);
            }
        }
        for index in 0..min_freq.len() {
            while min_freq[index] > 0 {
                output.push(index_to_string(index));
                min_freq[index] -= 1;
            }
        }
        output
    }
}
