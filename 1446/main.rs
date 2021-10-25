impl Solution {
    pub fn max_power(s: String) -> i32 {
        let (mut output,mut count) = (1,1);
        let s = s.chars().collect::<Vec<char>>();
        for i in 1..s.len(){
            if s[i-1] == s[i]{
                count+=1;
                output = std::cmp::max(output,count)
            } else {
                count = 1;
            }
        }
        output

    }
}
