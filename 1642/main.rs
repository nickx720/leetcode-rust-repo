/// https://dev.to/seanpgallivan/solution-furthest-building-you-can-reach-1m24
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        use std::collections::BinaryHeap;
        let (mut bricks, mut ladders) = (bricks,ladders);
        let mut pq: BinaryHeap<i32> = BinaryHeap::new();
        let length = heights.len()-1;
        for i in 0..length{
            let diff = heights[i+1] - heights[i];
            if diff > 0 {
                if ladders > 0 {
                    pq.push(-diff);
                    ladders-=1;
                } else if pq.len() > 0 && diff > -*pq.peek().unwrap(){
                    pq.push(-diff);
                    bricks -= -pq.pop().unwrap();
                } else {
                    bricks -=diff;
                }
                if bricks < 0 {
                    return i as i32;
                }
            }
        }
        length as i32

    }
}
