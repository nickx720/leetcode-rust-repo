impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        use std::collections::HashSet;
        if s.len() < 3 {
            return 0;
        }
        let mut count = 0;
        for i in 0..s.len()-2{
            let mut set_of_string:HashSet<char> = HashSet::new();
            &s[i..i+3].chars().for_each(|x| {
                set_of_string.insert(x);

            }
            );
            if set_of_string.len() == 3 {
                count+=1;
            }
        }
        count 
    }
}
