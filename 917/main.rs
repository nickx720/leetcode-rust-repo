impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        use std::collections::VecDeque;
        let mut stack:VecDeque<char> = VecDeque::new();
        for item in s.chars(){
            if item.is_alphabetic(){
                stack.push_back(item);
            }
        }
        let mut output = String::new();
        for item in s.chars(){
            if item.is_alphabetic(){
                output.push(stack.pop_back().unwrap());
            } else{
                output.push(item);
            }
        }
        output

    }
}
