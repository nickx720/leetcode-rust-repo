impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (col_len,row_len) = (obstacle_grid.len(),obstacle_grid[0].len());
        if obstacle_grid[0][0] == 1{
            return 0;
        }

        let mut output:Vec<Vec<i32>> = vec![vec![0;row_len];col_len];
        output[0][0] = 1;

        for i in 0..col_len{
            for j in 0..row_len{
                if obstacle_grid[i][j] == 1 || (i == 0 && j == 0){
                    continue;
                }
                let previous_col = if i == 0 {
                    0
                } else {
                    output[i-1][j]
                };
                let previous_row = if j == 0 {
                    0
                } else {
                    output[i][j-1]
                };
                output[i][j] = previous_row + previous_col;
            }
        }
        output[col_len-1][row_len-1]

    }
}
