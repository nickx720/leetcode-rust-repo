impl Solution {
    pub fn sum_base(n: i32, k: i32) -> i32 {
        let mut output: Vec<i32> = Vec::new();
        let mut n = n;
        loop {
            if n / k == 0 {
                output.push(n);
                break;
            }
            let rem = n % k;
            n = n / k;
            output.push(rem);
        }
        output.reverse();
        output.into_iter().sum()
    }
}
