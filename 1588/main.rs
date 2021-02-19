impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        if arr.len() == 1 {
            return arr.iter().sum();
        }
        let mut output = 0;
        let mut len = arr.len();
        while len > 0 {
            if len % 2 != 0 && len == arr.len() {
                output += arr[..].iter().sum::<i32>();
                len -= 2;
            } else if len % 2 == 0 {
                len -= 1;
            } else {
                let mut start = 0;
                while start + len <= arr.len() {
                    output += arr[start..start + len].iter().sum::<i32>();
                    start += 1;
                }
                if len == 1 {
                    break;
                }
                len -= 2;
            }
        }
        output
    }
}
