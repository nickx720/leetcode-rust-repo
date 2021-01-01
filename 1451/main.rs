impl Solution {
    pub fn arrange_words(text: String) -> String {
        let mut text = text.split(" ").collect::<Vec<&str>>();
        text.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut text = text
            .into_iter()
            .map(|x| x.clone().to_lowercase())
            .collect::<Vec<String>>();
        let mut first_word = text.remove(0).chars().collect::<Vec<char>>();
        first_word[0] = first_word[0].to_uppercase().nth(0).unwrap();
        let first_word = first_word.into_iter().collect::<String>();
        text.insert(0, first_word);
        text.join(" ")
    }
}
