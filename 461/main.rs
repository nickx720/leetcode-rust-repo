impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        use std::collections::HashMap;
        let x = format!("{:b}", x).chars().collect::<Vec<_>>();
        let y = format!("{:b}", y).chars().collect::<Vec<_>>();
        let mut output: HashMap<usize, i32> = HashMap::new();
        for (index, &item) in x.iter().rev().enumerate() {
            if item == '1' {
                let mut count = output.entry(index).or_insert(0);
                *count += 1;
            };
        }
        for (index, &item) in y.iter().rev().enumerate() {
            if item == '1' {
                if output.contains_key(&index) {
                    output.remove(&index);
                } else {
                    let mut count = output.entry(index).or_insert(0);
                    *count += 1;
                }
            }
        }
        output.len() as i32
    }
}
