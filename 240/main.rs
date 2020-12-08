impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() {
            return false;
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let mut cur_row = 0 as isize;
        let mut cur_col = (n - 1) as isize;
        while cur_row < m as isize && cur_col >= 0 {
            if matrix[cur_row as usize][cur_col as usize] == target {
                return true;
            }
            if matrix[cur_row as usize][cur_col as usize] > target {
                cur_col -= 1;
            } else {
                cur_row += 1;
            }
        }
        false
    }
}
