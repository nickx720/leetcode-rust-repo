impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        use std::collections::HashMap;
        let mut map_word1: HashMap<char, i32> = HashMap::new();
        let mut map_word2: HashMap<char, i32> = HashMap::new();
        word1
            .chars()
            .for_each(|x| *map_word1.entry(x).or_insert(0) += 1);
        word2
            .chars()
            .for_each(|x| *map_word2.entry(x).or_insert(0) += 1);
        let mut map_word: HashMap<char, i32> = HashMap::new();
        for (key, value) in map_word1.iter() {
            map_word.insert(*key, *value);
        }
        for (&key, &value) in map_word2.iter() {
            if map_word.contains_key(&key) {
                let new_value = *map_word.get(&key).unwrap();
                map_word.insert(key, (value - new_value).abs());
            } else {
                map_word.insert(key, value);
            }
        }
        !map_word.values().any(|&x| x > 3)
    }
}
