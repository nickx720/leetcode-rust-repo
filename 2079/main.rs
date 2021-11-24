impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let (mut output, mut cap) = (0, capacity);
        for i in 0..plants.len() {
            if cap - plants[i] >= 0 {
                output += 1;
                cap -= plants[i];
            } else {
                output += (2 * i as i32) + 1;
                cap = capacity;
                cap -= plants[i];
            }
        }
        output
    }
}
