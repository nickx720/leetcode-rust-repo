impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let (mut ret, mut power, mut n) = (0,31,x);
        while n != 0 {
            ret += (n & 1) << power;
            n = n >> 1;
            power -=1;
        }
        ret

    }
}
