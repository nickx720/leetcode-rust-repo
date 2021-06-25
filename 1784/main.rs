impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        s.split("0").collect::<Vec<&str>>().into_iter().filter(|&x| x != "").collect::<Vec<&str>>().len() == 1  
    }
}
