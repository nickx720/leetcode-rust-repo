impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        use std::collections::VecDeque;
        let s: Vec<char> = s.chars().collect();
        let mut stack: VecDeque<(char, usize)> = VecDeque::new();
        let mut output: Vec<String> = Vec::new();
        for (index, &item) in s.iter().enumerate() {
            let mut start = None;
            if item == '(' {
                stack.push_back((item, index));
            } else if item == ')' {
                start = stack.pop_back();
            }
            if stack.is_empty() && start.is_some() {
                let start = start.unwrap();
                output.push(s[start.1 + 1..index].iter().cloned().collect::<String>());
            }
        }
        output.join("")
    }
}
