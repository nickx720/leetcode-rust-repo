impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::new();
        for i in 1..arr.len() {
            let sliced_arr = &arr[i..];
            let max_item = *sliced_arr.iter().max().unwrap();
            output.push(max_item);
        }
        output.push(-1);
        output
    }
}
