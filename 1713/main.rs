impl Solution {
    // https://dev.to/seanpgallivan/solution-minimum-operations-to-make-a-subsequence-48b2
    pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut imap: HashMap<i32, usize> = HashMap::new();
        for (index, item) in target.iter().enumerate() {
            imap.insert(*item, index);
        }
        let mut items_array: Vec<usize> = Vec::new();
        for item in arr {
            if let Some(&value) = imap.get(&item) {
                items_array.push(value);
            }
        }
        let mut lis: Vec<usize> = Vec::new();
        for item in items_array {
            let index = Self::find(item, &lis);
            if index == lis.len() {
                lis.push(item);
            } else {
                lis[index] = item;
            }
        }
        (target.len() - lis.len()) as i32
    }
    fn find(target: usize, arr: &Vec<usize>) -> usize {
        let mut left = 0;
        let mut right = arr.len();
        while left < right {
            let mid = (left + right) / 2;
            if arr[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}
