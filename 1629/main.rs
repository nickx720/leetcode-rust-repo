impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let (mut max_time, mut max_key) = (0, ' ');
        release_times
            .iter()
            .zip(keys_pressed.chars())
            .fold(0, |prev, (&curr,character)|{
                let delay = curr - prev;
                if delay > max_time || delay == max_time && character > max_key{
                    max_key = character;
                    max_time = delay;
                }
                curr
            });
        max_key

    }
}
