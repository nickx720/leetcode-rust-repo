/// https://dev.to/seanpgallivan/solution-palindromic-substrings-54jf
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let (len,mut ans, mut i) = (s.len() as i32,0,0);
        while i < len {
            let mut j = i-1;
            let mut k = i;
            while k < len -1 && s[k as usize] == s[(k+1) as usize] {
                k+=1;
            }
            ans += (k-j) * (k-j+1)/2;
            i = k+1;
            k+=1;
            while j >= 0 && k < len && s[k as usize] == s[j as usize] {
                j-=1;
                k+=1;
                ans+=1;
            }
        }
        ans

    }
}
