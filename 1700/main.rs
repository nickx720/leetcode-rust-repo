impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        // Using retain to visit each elements exactly once in the same order
    // Check each element, see if it is the same as reversed last element of sandwich
    // if true remove from sandwiche, retain if not the same as last element of sandwich 
    let mut students = students;
    let mut sandwiches = sandwiches;
    sandwiches.reverse();
    loop {
        let start_len = students.len();
        students.retain(|prefer| {
            let last_elem = sandwiches.last().unwrap();
            let will_take = prefer == last_elem;
            if will_take {
                sandwiches.pop();
            }
            !will_take
        });
        let end_len = students.len();
        if start_len == end_len {
            break end_len as i32;
        }
    }
    }
}