impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut matrix = vec![vec![0; m as usize]; n as usize];
        for item in indices.iter() {
            let (first, second) = (item[0] as usize, item[1] as usize);

            for i in 0..m as usize {
                matrix[first][i] += 1;
            }
            for i in 0..n as usize {
                matrix[i][second] += 1;
            }
        }
        let mut count = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] % 2 != 0 {
                    count += 1;
                }
            }
        }
        count
    }
}
