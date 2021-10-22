/// https://dev.to/seanpgallivan/solution-remove-all-adjacent-duplicates-in-string-ii-431f
impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut s = s.chars().collect::<Vec<char>>();
        let mut st : Vec<i32> = Vec::new();
        st.push(0);
        let (mut i, mut j) = (1,1);
        while j < s.len(){
            s[i as usize] = s[j];
            if i==0 || s[i as usize] != s[(i-1) as usize]{
                st.push(i);
            } else if i - st.last().unwrap() + 1 == k {
                i = st.pop().unwrap() -1;
            }
            i+=1;
            j+=1;
        }
        s[0..(i as usize)].iter().map(|x| x.to_string()).collect::<Vec<String>>().join("")

    }
}
