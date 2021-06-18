impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        // let create a char vector
        // count down from n,get the min between each value of n and k, with 26
        // add the value to char array with a decrementing the value to stop overflow to z
        // print array to string

        let mut outpit:Vec<char> = Vec::new();
        let mut k = k;
        for i in (0..n).rev(){
            let val = std::cmp::min(26,k-i);
            outpit.push(('a' as u8+(val-1) as u8) as char);
            k=k-val;
        }
        outpit.into_iter().rev().into_iter().collect()

    }
}
