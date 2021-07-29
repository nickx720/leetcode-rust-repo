impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        use std::collections::VecDeque;
        let mut stack:VecDeque<i32> = VecDeque::new();
        stack.push_back(0);
        for character in s.chars(){
            if character == '(' {
                stack.push_back(0);
            } else {
                let (last,prev) = (stack.pop_back(),stack.pop_back());
                if let (Some(last_val),Some(prev_val)) = (last,prev){
                    stack.push_back(prev_val + std::cmp::max(2 * last_val,1));
                }
            }
        }
        let output = if let Some(out) = stack.pop_back(){ 
            out
        }else {
            0
        };
        output
    }
}
