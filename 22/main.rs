impl Solution {
    fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        backtrack(&mut output, "".to_string(), 0, 0, n);
        output
    }
}
