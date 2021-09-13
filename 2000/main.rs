impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let words = word.clone().chars().collect::<Vec<char>>();
        let index = words.iter().position(|&x| x == ch);
        if let Some(index) = index {
            let (first, second) = words.split_at(index+1);
            let mut first = first.to_vec().into_iter().rev().collect::<Vec<char>>();
            first.into_iter().chain(second.to_vec().into_iter()).collect::<Vec<char>>().into_iter().collect()
        } else {
            word
        }

    }
}
