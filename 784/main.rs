impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let len = s.len();
        let mut ans:Vec<String> = Vec::new();
        let mut s = s.to_lowercase().chars().map(|x| x.to_string()).collect::<Vec<String>>();
        Self::recursive_search(&mut s, &mut ans, 0, len);
        ans
    }

    fn recursive_search(s:&mut Vec<String>,ans:&mut Vec<String>, i:usize,len:usize) {
        use std::mem;
        if i < len {
            Self::recursive_search(s,ans,i+1,len);
            let character = s[i].chars().collect::<Vec<char>>();
            if character[0].is_alphabetic(){
                let element = s[i].to_uppercase();
                mem::replace(&mut s[i],element);
                Self::recursive_search(s, ans, i+1, len);
                s[i]=s[i].to_lowercase();
            }
        } else {
            ans.push(s.join("").to_string());
        }
    }
}
