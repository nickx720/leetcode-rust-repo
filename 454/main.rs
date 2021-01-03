impl Solution {
    fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        let mut a_and_b: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for i in 0..a.len() {
            for j in 0..b.len() {
                let sum: i32 = a[i] + b[j];
                if a_and_b.contains_key(&sum) {
                    let count_update: i32 = *a_and_b.get(&sum).unwrap() + 1;
                    a_and_b.insert(sum, count_update);
                } else {
                    a_and_b.entry(sum).or_insert(1);
                }
            }
        }

        for i in 0..c.len() {
            for j in 0..d.len() {
                let sum: i32 = c[i] + d[j];
                if a_and_b.contains_key(&-sum) {
                    count += *a_and_b.get(&-sum).unwrap();
                }
            }
        }
        count
    }
}
