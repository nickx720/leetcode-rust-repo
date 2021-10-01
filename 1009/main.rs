impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let mut output= String::new();
        let string_rep = format!("{:b}",n);
        for ch in string_rep.chars(){
            if ch == '1'
            {
                output.push_str("0")
            } else {
                output.push_str("1");
            }
        }
        i32::from_str_radix(&output, 2).unwrap()

    }
}
