impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut count: Vec<i32> = vec![0; 101];
        for &i in heights.iter() {
            count[i as usize] += 1;
        }
        let mut res = 0;
        let mut h = 0;
        for i in 0..heights.len() {
            while count[h] == 0 {
                h += 1;
            }
            if h as i32 != heights[i] {
                res += 1;
            }
            count[h] -= 1;
        }
        res
    }
}
