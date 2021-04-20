impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let len = s.len();
        let mut ans: Vec<i32> = vec![0; len];
        let mut prev = -(len as i32);
        let s = s.chars().collect::<Vec<char>>();
        for i in 0..len as i32 {
            if s[i as usize] == c {
                prev = i;
            }
            ans[i as usize] = i - prev;
        }
        for i in (0..len as i32).rev() {
            if s[i as usize] == c {
                prev = i;
            }
            ans[i as usize] = std::cmp::min(ans[i as usize], (i - prev).abs())
        }
        ans
    }
}
