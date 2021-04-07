impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let output = match arr.iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b)) {
            Some((output, _)) => output as i32,
            None => unreachable!(),
        };
        output
    }
}
