impl Solution {
    // Works on Rust 1.50
    fn sort_string(s: String) -> String {
        let mut output = String::new();
        let mut frequency: Vec<u8> = vec![0; 26];
        let calculate_position = |item: char| -> usize { item as usize - 'a' as usize };
        for item in s.chars() {
            frequency[calculate_position(item)] += 1;
        }
        let mut count = s.len();
        let lookup: Vec<char> = ('a'..='z').collect();
        while count > 0 {
            for i in 0..frequency.len() {
                if frequency[i] != 0 {
                    output = format!("{}{}", output, String::from(lookup[i]));
                    frequency[i] -= 1;
                    count -= 1;
                }
            }
        }
        output
    }
}
// Accepted by leetcode
impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut arr = vec![0; 26];
        for c in s.chars() {
            arr[c as usize - 'a' as usize] += 1;
        }
        let mut res = String::from("");
        let mut cter = s.len();
        let mut pivot = 0;

        while cter > 0 {
            for i in 0..26 {
                let j = if pivot == 0 {i as usize} else {(25 - i) as usize};
                if arr[j] > 0 {
                    arr[j] -= 1;
                    res.push((j as u8 + 97) as char);
                    cter -= 1;
                }
            }
            pivot = 1 - pivot;
        }
        res
    
    }
}