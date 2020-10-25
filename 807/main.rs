impl Solution{
fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let mut output = 0;
    let n = grid.len();
    let mut max_row_value: Vec<i32> = vec![0; n];
    let mut max_column_value: Vec<i32> = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            max_row_value[i] = std::cmp::max(max_row_value[i], grid[i][j]);
            max_column_value[j] = std::cmp::max(max_column_value[j], grid[i][j]);
        }
    }
    for i in 0..n {
        for j in 0..n {
            output += std::cmp::min(max_row_value[i], max_column_value[j]) - grid[i][j]
        }
    }
    output
}


}