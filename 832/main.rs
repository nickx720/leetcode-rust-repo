impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let a = a
            .into_iter()
            .map(|x| x.into_iter().rev().collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>();
        let a = a
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|y| if y == 0 { 1 } else { 0 })
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        a
    }
}
