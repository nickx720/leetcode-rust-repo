impl Solution {
    fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph = std::iter::repeat(Vec::<i32>::new())
            .take(num_courses as usize)
            .collect::<Vec<Vec<i32>>>();
        let mut indegrees = vec![0; num_courses as usize];
        for edge in &prerequisites {
            let (u, v) = (edge[0], edge[1]);
            graph[v as usize].push(u);
            indegrees[u as usize] += 1;
        }
        let mut stack: Vec<i32> = Vec::new();
        for (node, &degree) in indegrees.iter().enumerate() {
            if degree == 0 {
                stack.push(node as i32);
            }
        }
        // let mut order: Vec<i32> = Vec::new();
        while !stack.is_empty() {
            let v = stack.pop().unwrap();
            // order.push(v);
            for &u in &graph[v as usize] {
                indegrees[u as usize] -= 1;
                if indegrees[u as usize] == 0 {
                    stack.push(u);
                }
            }
        }
        if indegrees.iter().all(|&e| e == 0) {
            // order
            true
        } else {
            // vec![]
            false
        }
    }
}
