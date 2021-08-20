impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut arr = arr;
        arr.sort();
        let percentage = (5 * arr.len()) / 100;
        let arr = &arr[percentage..arr.len()-percentage];
        arr.iter().fold(0, |acc,elem| acc + elem) as f64 / arr.len() as f64

    }
}
