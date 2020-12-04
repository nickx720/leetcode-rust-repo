impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses;
        courses.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut heap: std::collections::BinaryHeap<i32> = std::collections::BinaryHeap::new();
        let mut time = 0;
        for course in courses.into_iter() {
            if time + course[0] <= course[1] {
                time += course[0];
                heap.push(course[0])
            } else {
                if let Some(&previous_time) = heap.peek() {
                    if previous_time > course[0] {
                        time -= previous_time - course[0];
                        heap.pop();
                        heap.push(course[0]);
                    }
                }
            }
        }
        heap.len() as i32
    }
}
