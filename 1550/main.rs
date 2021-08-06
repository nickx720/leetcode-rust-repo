impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        for item in 0..arr.len() {
            if item+2 < arr.len() {
                if arr[item] % 2 != 0 && arr[item+1] %2 != 0 && arr[item+2] %2 !=0 {
                    return true;
                }
            } 
        }
        false

    }
}
