impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashSet;
        let mut min_set: HashSet<i32> = HashSet::new();
        let mut output: Vec<i32> = Vec::new();
        for row in 0..matrix.len() {
            let mut min = std::i32::MAX;
            for col in 0..matrix[0].len() {
                if matrix[row][col] < min {
                    min = matrix[row][col];
                }
            }
            min_set.insert(min);
        }
        for col in 0..matrix[0].len() {
            let mut max = 0;
            for row in 0..matrix.len() {
                if matrix[row][col] > max {
                    max = matrix[row][col];
                }
            }
            if min_set.contains(&max) {
                output.push(max);
            }
        }
        output
    }
}
