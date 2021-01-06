impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        if k > 0 {
            return Self::case_one(code, k);
        }
        Self::case_two(code, k)
    }
    fn case_one(code: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut sum_of_k: i32 = code[0..k].iter().sum();
        let mut output: Vec<i32> = Vec::new();
        for i in 0..code.len() {
            sum_of_k -= code[i];
            sum_of_k += code[(i + k) % code.len()];
            output.push(sum_of_k);
        }
        output
    }
    fn case_two(code: Vec<i32>, k: i32) -> Vec<i32> {
        let mut code = code;
        code.reverse();
        code = Self::case_one(code, -k);
        code.reverse();
        code
    }
}
