impl Solution {
    pub fn count_prime_set_bits(l: i32, r: i32) -> i32 {
        let mut ans = 0;
        let set_bits:[i32;8] = [2,3,5,7,11,13,17,19];
        for item in l..=r {
            let item_string = format!("{:b}",item).split("").filter(|&x| x == "1".to_string().as_str()).count() as i32;
            if set_bits.contains(&item_string) {
                ans+=1;
            }
        }
        ans

    }
}
