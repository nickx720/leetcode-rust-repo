impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let total_col = matrix.len();
        let total_row = matrix[0].len();
        for i in (0..total_col).rev(){
            for j in 0..total_row {
                if i ==0 || j==0 {
                    continue;
                }
                if matrix[i][j] != matrix[i-1][j-1] {
                    return false;
                }
            }
        }
        true 
    }
}
