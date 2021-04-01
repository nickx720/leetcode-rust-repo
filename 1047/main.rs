impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        use std::collections::VecDeque;
        let mut stack: VecDeque<char> = VecDeque::new();
        for item in s.chars().into_iter() {
            if stack.len() == 0 {
                stack.push_back(item);
                continue;
            }
            if let Some(&front) = stack.back() {
                if front == item {
                    stack.pop_back();
                } else {
                    stack.push_back(item);
                }
            }
        }
        stack.iter().collect()
    }
}
