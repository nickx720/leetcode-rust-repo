impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        use std::cmp::min;
        let (mut start, mut count) = (0,0);

        for item in word.bytes(){
            let current = (item -97) as i32;
            let diff = start - current;
            let diff_anti_clockwise = start - 26 - current;
            let diff_clockwise = start + 26 - current;
            count+= min(diff.abs(), min(diff_clockwise.abs(), diff_anti_clockwise.abs()));
            start = current;
        }
        let length = word.len() as i32;
        count + length

    }
}
