impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut c: Vec<i32> = vec![0; 60];
        let mut res = 0;
        for item in time.iter() {
            res += c[((540 - item) % 60) as usize];
            c[(item % 60) as usize] += 1;
        }
        res
    }
}
