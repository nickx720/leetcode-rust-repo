impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let (mut ans, mut prev, mut cur) = (0,0,1);
        for i in 1..s.len(){
            if s[i-1] != s[i]{
                ans+= std::cmp::min(prev,cur);
                prev = cur;
                cur =1;
            } else {
                cur+=1;
            }
        }
        ans + std::cmp::min(prev,cur)

    }
}
