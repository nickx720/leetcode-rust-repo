impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::new();
        output.push(first);
        for i in 0..encoded.len() {
            output.push(output[i] ^ encoded[i]);
        }
        output
    }
}
