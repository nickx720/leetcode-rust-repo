impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        for (index,word) in sentence.split_whitespace().enumerate(){
            if word.starts_with(&search_word){
                return (index as i32) + 1
            }
        }
        -1

    }
}
