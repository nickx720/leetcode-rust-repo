impl Solution {
    fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        backtrack(&mut output, "".to_string(), 0, 0, n);
        output
    }
    fn backtrack(list: &mut Vec<String>, current_string: String, open: i32, close: i32, max: i32) {
        if (current_string.len() as i32) == (max * 2) {
            list.push(current_string);
            return;
        }
        if open < max {
            backtrack(list, current_string.clone() + "(", open + 1, close, max)
        }
        if close < open {
            backtrack(list, current_string.clone() + ")", open, close + 1, max)
        }
    }
}
