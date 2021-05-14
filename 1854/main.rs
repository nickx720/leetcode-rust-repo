impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut log: Vec<i32> = vec![0; 2051];
    for item in logs {
        let (start, end) = (item[0] as usize, item[1] as usize);
        log[start] += 1;
        log[end] -= 1;
    }
    let mut max_year = 1950;
    for i in 1950 as usize..2051 as usize {
        log[i] += log[i - 1];

        if log[i] > log[max_year as usize] {
            max_year = i as i32;
        }
    }
    max_year
    }
}