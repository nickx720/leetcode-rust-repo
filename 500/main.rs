impl Solution {
    // Create a set of rows each represeting one row of the keyboard, iterate through words and
    // filter each by passing the word to the keyboard row to check if each character is contained
    // in the row
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let keyboard_rows = ["qwertyuiop","asdfghjkl","zxcvbnm"];       
        words.into_iter().filter(|x|{
            keyboard_rows.iter().fold(false,|b,&row|{
                b || x.to_lowercase().chars().all(|x| row.contains(x))
            })
        }).collect()
    }
}
