impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let digit_count = |n:i32| -> [usize;10]{
            let mut n = n;
            let mut d = [0;10];
            while n > 0 {
                d[(n%10) as usize] +=1;
                n /=10;
            }
            d
        };
        let count = digit_count(n);
        (0..31).any(|item| digit_count(1 << item) == count)

    }
}
