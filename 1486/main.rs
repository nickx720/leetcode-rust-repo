impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut output: Vec<i32> = Vec::new();
        for i in 0..n {
            let value = start + (2 * i);
            output.push(value);
        }
        let output = output.iter().fold(0, |acc, x| acc ^ x);
        output
    }
}
