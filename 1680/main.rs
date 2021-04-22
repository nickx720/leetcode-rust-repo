impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut ans = 1u64;
        let mut len = 0b100u64;
        let modulo = 10_u64.pow(9) + 7;
        for i in 2..=n {
            let i = i as u64;
            if i == len {
                len <<= 1;
            };
            ans = (ans * len + i) % modulo;
        }
        ans as i32
    }
}
