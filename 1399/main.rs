impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        use std::collections::HashMap;
        let mut output:HashMap<i32,i32> = HashMap::new();
        for i in 1..=n{
            *output.entry(Self::sum_of_digits(i)).or_insert(0)+=1;
        }
        let max_val = output.values().max().unwrap();
        output.values().filter(|&x| x == max_val).count() as i32

    }
    pub   fn sum_of_digits(n:i32)-> i32 {
        let mut n = n;
        let mut sum = 0;
        while n != 0 {
            sum = sum + n%10;
            n = n/10;
        }
        sum
    }
}
