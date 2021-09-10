impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        use std::collections::VecDeque;
        let mut stack:VecDeque<i32> = VecDeque::new();
        for item in logs{
            if item == "../"{
                stack.pop_back();
                continue;
            } else if item == "./"{
                continue;
            } else {
                stack.push_back(0);
            }
        }
        stack.len() as i32

    }
}
