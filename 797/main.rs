impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut output = vec![];
        let mut path_taken = vec![];
        Self::dfs(&graph, &mut output, &mut path_taken, 0);
        output
    }
    fn dfs(graph: &Vec<Vec<i32>>, output: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, curr: i32) {
        path.push(curr);

        if curr + 1 == graph.len() as i32 {
            output.push(path.to_owned());
        } else {
            for &each_item in graph[curr as usize].iter() {
                Self::dfs(graph, output, path, each_item);
            }
        }
        path.pop();
    }
}
