impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let mut dict: Vec<i32> = vec![0; 26];
        let module = 10u32.pow(9) as i32 + 7;
        let mut total = 0;
        let s = s.chars().collect::<Vec<char>>();
        for i in 0..=s.len() {
            total = 0;
            for j in 0..26 {
                total = (total + dict[j]) % module;
            }
            if i < s.len() {
                let index = s[i] as usize - 'a' as usize;
                dict[index] = (1 + total) % module;
            }
        }
        total
    }
}
