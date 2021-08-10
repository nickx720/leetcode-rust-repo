impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        use std::collections::HashSet;
        let k = k as usize;
        let mut need = 1 << k;
        let mut got:HashSet<&str> = HashSet::new();
        for i in k..=s.len(){
            let local_string = &s[i-k..i];
            if !got.contains(local_string){
                got.insert(local_string);
                need-=1;
                if need == 0 {
                    return true;
                }
            }
        }
        false
    }
}
