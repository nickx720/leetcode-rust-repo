impl Solution {
    pub fn max_depth(s: String) -> i32 {
        use std::collections::VecDeque;
        let mut count = 0;
        let mut stack: VecDeque<char> = VecDeque::new();
        for v in s.chars() {
            if v == '(' {
                stack.push_back(v);
            } else if v == ')' {
                stack.pop_back();
                count = std::cmp::max(count, stack.len() as i32 + 1);
            }
        }
        count
    }
}
