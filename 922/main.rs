impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut output: Vec<i32> = vec![0; a.len()];
        let mut even_count = 0;
        let mut odd_count = 1;
        for i in 0..a.len() {
            if a[i] % 2 == 0 {
                output[even_count] = a[i];
                even_count += 2;
            } else {
                output[odd_count] = a[i];
                odd_count += 2;
            }
        }
        output
    }
}
