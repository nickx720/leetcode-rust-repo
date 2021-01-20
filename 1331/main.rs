impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        if arr.len() == 0 {
            return Vec::new();
        }
        let mut output: Vec<(usize, i32)> = arr
            .iter()
            .enumerate()
            .map(|(id, key)| (id, *key))
            .collect::<Vec<(usize, i32)>>();
        output.sort_by(|(_, a), (_, b)| a.cmp(&b));
        let mut total = 1;
        let mut result: Vec<i32> = vec![0; arr.len()];
        let mut prev: Option<(usize, i32)> = None;
        for (id, key) in output {
            if prev != None && key == prev.unwrap().1 {
                result[id] = result[prev.unwrap().0];
                continue;
            }
            result[id] = total as i32;
            total += 1;
            prev = Some((id, key));
        }
        result
    }
}
