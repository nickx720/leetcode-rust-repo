impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut d = dictionary;
        d.sort_unstable_by(|a,b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));
        for item in d{
            let mut current_item = item.chars().peekable();
            for character in s.chars(){
                if let Some(&current_character_item) = current_item.peek(){
                    if current_character_item == character{
                        current_item.next();
                    }
                } else {
                    break;
                }
            }
            if current_item.peek().is_none(){
                return item;
            }
        }
        String::new()

    }
}
