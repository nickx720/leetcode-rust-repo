impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: std::collections::VecDeque<i32> = std::collections::VecDeque::new();

        for token in tokens {
            match &token[..] {
                "+" => {
                    if let Some(value_one) = stack.pop_back() {
                        if let Some(value_two) = stack.pop_back() {
                            stack.push_back(value_one + value_two);
                        }
                    }
                }
                "-" => {
                    if let Some(value_one) = stack.pop_back() {
                        if let Some(value_two) = stack.pop_back() {
                            stack.push_back(value_two - value_one);
                        }
                    }
                }
                "*" => {
                    if let Some(value_one) = stack.pop_back() {
                        if let Some(value_two) = stack.pop_back() {
                            stack.push_back(value_one * value_two);
                        }
                    }
                }
                "/" => {
                    if let Some(value_one) = stack.pop_back() {
                        if let Some(value_two) = stack.pop_back() {
                            stack.push_back(value_two / value_one);
                        }
                    }
                }
                token => {
                    if let Ok(value) = token.parse::<i32>() {
                        stack.push_back(value);
                    }
                }
            }
        }
        if let Some(output) = stack.pop_back() {
            output
        } else {
            0
        }
    }
}
