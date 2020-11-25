impl Solution {
    pub fn num_enclaves(a: Vec<Vec<i32>>) -> i32 {
        let n = a.len();
        let m = a[0].len();
        let mut a = a;
        for i in 0..n {
            for j in 0..m {
                if a[i][j] == 1 {
                    if i == 0 || i == n - 1 || j == 0 || j == m - 1 {
                        Self::dfs(&mut a, i as isize, j as isize);
                    }
                }
            }
        }
        let mut total = 0;
        for i in 0..n {
            for j in 0..m {
                if a[i][j] == 1 {
                    total += 1;
                }
            }
        }
        total
    }
    fn dfs(a: &mut Vec<Vec<i32>>, i: isize, j: isize) {
        let n = a.len() as isize;
        let m = a[0].len() as isize;
        if i < 0 || i >= n || j < 0 || j >= m || a[i as usize][j as usize] == 0 {
            return;
        }
        a[i as usize][j as usize] = 0;
        Self::dfs(a, i - 1, j);
        Self::dfs(a, i + 1, j);
        Self::dfs(a, i, j - 1);
        Self::dfs(a, i, j + 1);
        return;
    }
}
