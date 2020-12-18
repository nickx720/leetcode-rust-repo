impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        if n % 4 != 0 {
            return true;
        } else {
            return false;
        }
    }
}
