impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes = boxes
            .chars()
            .enumerate()
            .map(|(ind, val)| {
                let val = val.to_digit(10).unwrap();
                (ind, val)
            })
            .collect::<Vec<(usize, u32)>>();
        let collection = boxes
            .clone()
            .into_iter()
            .filter(|(_, x)| *x != 0)
            .collect::<Vec<(usize, u32)>>();
        let mut output: Vec<i32> = Vec::new();
        for (index, _) in boxes {
            let mut value = 0;
            for (ind, _) in collection.iter() {
                value += (*ind as i32 - index as i32).abs();
            }
            output.push(value);
        }
        output
    }
}
