impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        const VOWELS: [char;10] = ['a','A','e','E','i','I','o','O','u', 'U'];
        let sentence = sentence.split(" ");
        let mut output = Vec::new();
        for (index,word) in sentence.enumerate() {
            let mut char_words = word.trim().chars().collect::<Vec<char>>();
            if !VOWELS.contains(&char_words.get(0).unwrap()) { 
                let removed_item = char_words.remove(0);
                char_words.push(removed_item);
            }
            output.push([char_words,vec!['m','a'],vec!['a'].repeat(index+1)].concat().iter().cloned().collect::<String>()); 
        }
        output.join(" ")

    }
}
