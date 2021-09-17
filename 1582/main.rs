impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let row_len = mat.len();
        let col_len = mat[0].len();
        let mut row = vec![0;row_len];
        let mut col = vec![0;col_len];
        for i in 0..row_len{
            for j in 0..col_len{
                if mat[i][j] == 1 {
                    row[i]+=1;
                    col[j]+=1;
                }
            }
        }
        let mut count = 0;
        for i in 0..row_len{
            for j in 0..col_len{
                if mat[i][j] == 1 && row[i] == 1 && col[j] == 1 {
                    count+=1;
                }
            }
        }
        count
    }
}
