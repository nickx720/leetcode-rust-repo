impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort();
        let (first, last) = (arr[0], arr[1]);
        let diff = (last - first).abs();
        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] != diff {
                return false;
            }
        }
        true
    }
}
