impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let rows = mat.len();
        let column = mat[0].len();
        let (r,c) = (r as usize, c as usize);
        if rows * column != r * c{
            return mat;
        }
        let mut output:Vec<Vec<i32>> = vec![vec![0;c];r];
        let (mut row_index,mut col_index) = (0,0);
        for i in 0..mat.len(){
            for j in 0..mat[0].len(){
                output[row_index][col_index] = mat[i][j];
                col_index+=1;
                if col_index == c {
                    col_index=0;
                    row_index+=1;
                }
            }
        }
        output

    }
}
