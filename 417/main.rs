//https://dev.to/seanpgallivan/solution-pacific-atlantic-water-flow-4lb3
// https://leetcode.com/problems/pacific-atlantic-water-flow/discuss/1127040/Rust-DFS-solution
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if heights.is_empty(){
            return Vec::new();
        }
        let (m,n) = (heights.len(),heights[0].len());
        let mut p_reachable = vec![vec![false;n];m];
        let mut a_reachable = vec![vec![false;n];m];
        for i in 0..m {
            Self::dfs(&mut p_reachable, &heights, (i,0));
            Self::dfs(&mut a_reachable, &heights, (i,n-1));
        }
        for j in 0..n{
            Self::dfs(&mut p_reachable, &heights, (0,j));
            Self::dfs(&mut a_reachable, &heights, (m-1,j));
        }
        let mut answers :Vec<Vec<i32>> = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if p_reachable[i][j] && a_reachable[i][j] {
                    answers.push([i as i32,j as i32].to_vec());
                }
            }
        }
        answers

    }
    fn dfs(reachable: &mut Vec<Vec<bool>>,matrix: &Vec<Vec<i32>>,(i,j): (usize,usize)) {
        reachable[i][j] = true;
        for &(di,dj) in &[(-1,0),(0,-1),(1,0),(0,1)] {
            let ni = di + i as i32;
            let nj = dj + j as i32;
            if (0..matrix.len() as i32).contains(&ni) && (0..matrix[0].len() as i32).contains(&nj) && !reachable[ni as usize][nj as usize] && matrix[ni as usize][nj as usize] >= matrix[i][j] {
                Self::dfs(reachable,&matrix,(ni as usize, nj as usize));
            }

        }
    }

}
