impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut count = 0;
        let mut truck_size = truck_size;
        let mut box_types = box_types
            .iter()
            .map(|x| (x[0], x[1]))
            .collect::<Vec<(i32, i32)>>();
        box_types.sort_by(|(_, a), (_, b)| b.cmp(a));
        for (size, item) in box_types {
            truck_size -= size;
            if truck_size > 0 {
                count += size * item;
            } else {
                count += (truck_size + size) * item;
                break;
            }
        }
        count
    }
}
