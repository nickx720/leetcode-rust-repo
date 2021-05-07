impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::max;
        let mut ans = 0;
        for i in 0..grid.len() {
            let mut best_row = 0;
            let mut best_col = 0;
            for j in 0..grid.len() {
                if grid[i][j] > 0 {
                    ans += 1;
                }
                best_row = max(best_row, grid[i][j]);
                best_col = max(best_col, grid[j][i]);
            }
            ans += best_row + best_col;
        }
        ans
    }
}
