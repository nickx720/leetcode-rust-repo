
impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap_or(&0);
        let min = *nums.iter().min().unwrap_or(&0);
        let mut output = Vec::new();
        for item in 1..=min{
            if max % item == 0 && min % item == 0 {
                output.push(item);
            }
        }
        output.last().unwrap().to_owned()
    }
}
