impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut output = a.into_iter().map(|x| x.pow(2)).collect::<Vec<i32>>();
        output.sort();
        output
    }
}
