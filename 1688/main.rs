impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut count = 0;
        let mut n = n;
        while n > 0 {
            if n == 1 {
                n = 0;
            }
            if n % 2 == 0 {
                n = n / 2;
                count += n;
            } else {
                count += (n - 1) / 2;
                n = (n - 1) / 2 + 1;
            }
        }
        count
    }
}
