impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let n = n as usize;
        let mut output = vec![0; n + 3];
        output[1] = 1;
        output[2] = 1;
        for i in 3..=n {
            output[i] = output[i - 3] + output[i - 2] + output[i - 1];
        }
        output[n]
    }
}
