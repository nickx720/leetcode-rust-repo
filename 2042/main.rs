impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let s = s.split(" ").collect::<Vec<&str>>().iter().map(|x| x.parse::<u32>().ok()).filter(|x| x.is_some()).map(|x| {
            match x {
                Some(value) => value,
                None=> unreachable!()
            }
        }).collect::<Vec<u32>>();
        for index in 1..s.len(){
            if s[index-1] >= s[index]{
                return false;
            }
        }
        true

    }
}
