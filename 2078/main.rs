impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let mut remove_dup = colors.clone();
        let mut output = 0;
        remove_dup.sort();
        remove_dup.dedup();
        for item in remove_dup {
            let furthest_distance = colors.iter().rposition(|&x| x != item).unwrap();
            let distance = colors.iter().position(|&x| x == item).unwrap();
            output = output.max((distance as i32 - furthest_distance as i32).abs());
        }
        output
    }
}
