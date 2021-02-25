impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        if mat.len() == 1 {
            return mat[0][0];
        }
        let len = mat.len() - 1;
        let mut count = 0;
        for (index, item) in mat.iter().enumerate() {
            if index == 0 || index == len {
                count += item[0] + item[len];
                continue;
            }
            if mat.len() % 2 != 0 && index == mat.len() / 2 {
                count += item[index];
                continue;
            }
            count += item[index] + item[len - index];
        }
        count
    }
}
