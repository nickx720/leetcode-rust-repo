/// https://dev.to/seanpgallivan/solution-number-of-submatrices-that-sum-to-target-3521
impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        use std::collections::HashMap;
        let (xlen, ylen,mut ans, mut matrix) = (matrix[0].len(),matrix.len(),0,matrix);
        let mut res: HashMap<i32,i32> = HashMap::new();
        for i in 0..ylen{
            for j in 1..xlen{
                matrix[i][j] += matrix[i][j-1];
            }
        }
        for j in 0..xlen{
            for k in j..xlen{
                res.clear();
                res.insert(0, 1);
                let mut csum = 0;
                for i in 0..ylen{
                    let value = if j != 0{
                        matrix[i][j-1]
                    } else {
                        0
                    };
                    csum += matrix[i][k] - value;
                    let key_value = res.get(&(csum-target)).unwrap_or(&0);
                    ans+=key_value;
                    res.insert(csum, *res.get(&csum).unwrap_or(&0)+1);
                }
            }
        }
        ans

    }
}
