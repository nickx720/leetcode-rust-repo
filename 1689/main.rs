impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let n = n
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>()
            .into_iter()
            .max();
        if let Some(n) = n {
            n
        } else {
            0
        }
    }
}
