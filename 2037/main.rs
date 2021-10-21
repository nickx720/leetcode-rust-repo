impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let (mut seats, mut students) = (seats,students);
        seats.sort();
        students.sort();

        let mut output=0;
        for (&seat,&student) in seats.iter().zip(students.iter()){
            output+= (seat - student).abs()
        }
        output

    }
}
