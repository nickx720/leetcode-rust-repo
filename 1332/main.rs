impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let new_string = s.clone().chars().rev().collect::<String>();
        if s == new_string {
            return 1;
        }
        2
    }
}
