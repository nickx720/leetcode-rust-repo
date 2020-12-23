impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut set = std::collections::HashSet::new();
        for word in word_list {
            set.insert(word);
        }
        if !set.contains(&end_word) {
            return 0;
        }
        let mut queue: std::collections::VecDeque<String> = std::collections::VecDeque::new();
        queue.push_back(begin_word);
        let mut level = 1;
        let letters = (b'a'..=b'z').map(char::from).collect::<Vec<_>>();
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                if let Some(current_word) = queue.pop_front() {
                    for j in 0..current_word.len() {
                        for k in &letters {
                            let new_word = (&current_word[..j]).to_string()
                                + &k.to_string()
                                + &current_word[j + 1..];
                            if new_word == current_word {
                                continue;
                            }
                            if new_word == end_word {
                                return level + 1;
                            }
                            if set.contains(&new_word) {
                                queue.push_back(new_word.clone());
                                set.remove(&new_word);
                            }
                        }
                    }
                }
            }
            level += 1;
        }
        0
    }
}
