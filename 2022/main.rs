impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let (m,n,mut original) = (m as usize, n as usize,original);
        if original.len() != m * n {
            return vec![]
        }
        let mut output:Vec<Vec<i32>> = vec![vec![0;n];m];
        for i in 0..m{
            for j in 0..n{
                output[i][j] = original.remove(0);
            }
        }
        output

    }
}
