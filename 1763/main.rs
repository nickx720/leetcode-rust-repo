impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        use std::collections::HashSet;
        if s.len() < 2 {
            return "".to_string();
        }
        let mut set: HashSet<String> = HashSet::new();
        s.chars().for_each(|x|{  set.insert(x.to_string());});
        for i in 0..s.len() {
            if set.contains(&s[i..i+1].to_string().to_lowercase()) && set.contains(&s[i..i+1].to_string().to_uppercase()) {
                continue;
            }
            let string_1 = Self::longest_nice_substring(s[0..i].to_string());
            let string_2 = Self::longest_nice_substring(s[i+1..].to_string());
            if string_1.len() >= string_2.len(){
                return string_1;
            }else {
                return string_2;}
        }
        s

    }
}
