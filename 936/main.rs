// https://leetcode.com/problems/stamping-the-sequence/discuss/1135900/Rust-C%2B%2B%3A-Undo-The-Stamps-%2B-Explanation
// https://dev.to/seanpgallivan/solution-stamping-the-sequence-4fc7
impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let (stamp,mut target) = (stamp.as_bytes(),target.into_bytes());
        let mut res:Vec<i32> = Vec::new();

        while target.iter().any(|&c| c != b'?') {
            let previous_len = res.len();

            for i in 0..target.len()-stamp.len()+1{
                let sub = &mut target[i..i+stamp.len()];

                if sub.iter().any(|&c| c != b'?') && sub.iter().zip(stamp.iter()).all(|(&wc,&sc)| wc == sc || wc == b'?') {
                    sub.iter_mut().for_each(|c| *c = b'?');
                    res.push(i as _);
                }
            }
            if previous_len == res.len(){
                return Vec::new();
            }
        }
        res.reverse();
        res

    }
}
