impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut new_mat = mat
            .iter()
            .map(|x| x.iter().sum())
            .enumerate()
            .collect::<Vec<(usize, i32)>>();
        new_mat.sort_by(|(_, a), (_, b)| a.cmp(&b));
        let final_mat = new_mat.iter().map(|x| x.0 as i32).collect::<Vec<i32>>();
        final_mat[0..k as usize].to_vec()
    }
}
