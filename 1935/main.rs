impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut count = 0;
        let mut text = text.split(" ").collect::<Vec<&str>>();
        for word in text {
            for chars in broken_letters.chars(){
                if word.contains(chars){
                    count-=1;
                    break;
                }
            }
            count+=1;
        }
        count

    }
}
