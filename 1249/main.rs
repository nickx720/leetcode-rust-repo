impl Solution {
    // explaination https://dev.to/seanpgallivan/solution-minimum-remove-to-make-valid-parentheses-2bdj
    pub fn min_remove_to_make_valid(s: String) -> String {
        use std::collections::VecDeque;
        let mut s = s.trim().chars().collect::<Vec<char>>();
        let mut stack:VecDeque<usize> = VecDeque::new();
        for i in 0..s.len(){
            if s[i] == '(' {
                stack.push_front(i);
            } else if s[i] == ')'{
                if stack.len() > 0 {
                    stack.pop_front();
                } else {
                    s[i] = ' ';
                }
            }
        }
        for i in stack{
            s.remove(i);
        }
        s.iter().cloned().filter(|&x| x != ' ').collect()

    }
}
