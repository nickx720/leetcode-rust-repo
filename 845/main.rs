impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let length = arr.len();
        let (mut base, mut ans) = (0, 0);
        while base < length {
            let mut end = base;
            // check if end is both within array and increasing
            if end + 1 < length && arr[end] < arr[end + 1] {
                // increment the value of end pointer as long as it keeps steadily increasing
                while end + 1 < length && arr[end] < arr[end + 1] {
                    end += 1;
                }
                // Found a value which is peak because following value is less,
                if end + 1 < length && arr[end] > arr[end + 1] {
                    // as long as following value is less than the current peak, increment end
                    while end + 1 < length && arr[end] > arr[end + 1] {
                        end += 1;
                    }
                    // calculate answer by getting max value of current ans and end-base
                    ans = std::cmp::max(ans, end - base + 1);
                }
            }
            // increment the base
            base = std::cmp::max(base + 1, end);
        }
        ans as i32
    }
}
