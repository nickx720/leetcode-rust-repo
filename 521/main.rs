impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            return -1;
        }
        let output = std::cmp::max(a.len(), b.len());
        output as i32
    }
}
