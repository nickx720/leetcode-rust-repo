impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        if n % 2 != 0 {
            result.push(0);
        }
        for i in 0..n / 2 {
            result.push(n - i);
            result.push(-n + i);
        }
        result
    }
}
