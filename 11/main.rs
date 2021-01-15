impl Solution {
    if height.len() < 1 {
    fn max_area(height: Vec<i32>) -> i32 {
        0
    } else {
        let mut max: i32 = 0;
        let mut left: usize = 0;
        let mut right: usize = height.len() - 1;
        while left < right {
            let min_of_distance: i32 = std::cmp::min(height[left], height[right]);
            let distance = right - left;
            max = std::cmp::max(max, min_of_distance * (distance as i32));
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max
    }
}

}