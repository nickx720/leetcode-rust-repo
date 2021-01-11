impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let n = arr.len();
    if n < (m * k) as usize {
        return false;
    }
    let end = n - (m * k) as usize;
    let m = m as usize;
    for i in 0..=end {
        let mut res = true;
        for j in 1..k as usize {
            if arr[i..m + i] != arr[i + m * j..i + m * (j + 1)] {
                res = false;
               break;
            }
        }
        if res {
            return true;
        }
    }
    false
    }
}