impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let odd_positions = ['a', 'c', 'e', 'g'];
        let coordinates = coordinates.chars().collect::<Vec<char>>();
        let (char_position, box_position) = (coordinates[0], coordinates[1] as i32);
        if (odd_positions.contains(&char_position) && box_position % 2 != 0)
            || (!odd_positions.contains(&char_position) && box_position % 2 == 0)
        {
            false
        } else {
            true
        }
    }
}
