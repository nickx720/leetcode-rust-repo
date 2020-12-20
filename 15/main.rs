impl Solution {
    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut array: Vec<i32> = nums;
        array.sort();
        let mut output: Vec<Vec<i32>> = Vec::new();
        for i in 0..array.len() - 2 {
            if i == 0 || (i > 0 && array[i] != array[i - 1]) {
                let mut low = i + 1;
                let mut high = array.len() - 1;
                let sum = 0 - array[i];
                while low < high {
                    if array[low] + array[high] == sum {
                        output.push(vec![array[i], array[low], array[high]]);
                        while low < high && array[low] == array[low + 1] {
                            low += 1
                        }
                        while low < high && array[high] == array[high - 1] {
                            high -= 1
                        }
                        low += 1;
                        high -= 1;
                    } else if array[low] + array[high] > sum {
                        high -= 1;
                    } else {
                        low += 1;
                    }
                }
            }
        }
        output
    }
}
