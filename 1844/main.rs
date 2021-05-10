impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut output = String::new();
        for i in (0..s.len()).step_by(2) {
            if i + 2 <= s.len() {
                output += &Self::shift(&s[i..i + 2]);
            } else {
                output += &s[i..];
            }
        }
        output
    }

    fn shift(s: &str) -> String {
        let chars_arr = s.chars().collect::<Vec<char>>();
        let (first, second) = (chars_arr[0].to_string(), chars_arr[1].to_digit(10).unwrap());
        let second = first.bytes().next().unwrap() + second as u8;
        let output = first.to_string() + (second as char).to_string().as_str();
        output
    }
}
