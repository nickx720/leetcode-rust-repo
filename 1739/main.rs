impl Solution {
    pub fn minimum_boxes(n: i32) -> i32 {
        let (mut total, mut bottom, mut height) = (1, 1, 1);
        while total < n {
            height += 1;
            bottom += height;
            total += bottom;
        }
        while total >= n {
            bottom -= 1;
            total -= height;
            height -= 1;
        }
        bottom + 1
    }
}
