impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut s = s.bytes().map(|x| {x - 96}).map(|x| x.to_string()).collect::<Vec<String>>().join("");
        let mut sum = 0;
        for _ in 0..k{
            sum = s.chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>().iter().sum();
            s = sum.to_string();
        }
        sum as i32

    }
}
