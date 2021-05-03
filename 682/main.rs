impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        use std::collections::VecDeque;
        let mut baseball_stack: VecDeque<i32> = VecDeque::new();
        for item in ops {
            match item.as_str() {
                "C" => {
                    let _ = baseball_stack.pop_back();
                }
                "D" => {
                    if let Some(&previous_score) = baseball_stack.back() {
                        baseball_stack.push_back(previous_score * 2);
                    }
                }
                "+" => {
                    let values = (
                        baseball_stack.get(baseball_stack.len() - 1),
                        baseball_stack.get(baseball_stack.len() - 2),
                    );
                    if let (Some(&first), Some(&second)) = (values.0, values.1) {
                        baseball_stack.push_back(first + second);
                    }
                }
                value => {
                    if let Ok(value) = value.parse::<i32>() {
                        baseball_stack.push_back(value);
                    };
                }
            }
        }
        baseball_stack
            .into_iter()
            .collect::<Vec<i32>>()
            .iter()
            .sum()
    }
}
