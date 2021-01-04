impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut directions = (0, 1);
        let mut start = [0, 0];
        for _ in 0..4 {
            for x in instructions.chars() {
                match x {
                    'G' => {
                        let (left, right) = directions;
                        start[0] += left;
                        start[1] += right;
                    }
                    'L' => {
                        let (left, right) = directions;
                        directions = (-right, left);
                    }
                    'R' => {
                        let (left, right) = directions;
                        directions = (right, -left);
                    }
                    _ => panic!("Illegal operation"),
                }
            }
        }
        start == [0, 0]
    }
}
