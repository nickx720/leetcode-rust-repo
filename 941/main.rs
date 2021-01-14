impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
         let mut i = 0;
    while i < arr.len()-1 && arr[i] < arr[i + 1] {
        i += 1;
    }
    if i == 0 || i == arr.len() - 1 {
        return false;
    }
    while i < arr.len()-1 && arr[i] > arr[i + 1] {
        i += 1;
    }
    let output = if i == arr.len()-1 { true } else { false };
    output
    }
}