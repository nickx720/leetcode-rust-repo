use std::collections::HashSet;
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut list: Vec<i32> = Vec::new();
        if graph.is_empty() {
            return list;
        }
        let mut visiting: HashSet<i32> = HashSet::new();
        let mut visited: HashSet<i32> = HashSet::new();
        for i in 0..graph.len() as i32 {
            if !Self::has_cycle(i, &graph, &mut visiting, &mut visited) {
                list.push(i);
            }
        }
        list
    }
    fn has_cycle(
        index: i32,
        graph: &Vec<Vec<i32>>,
        visiting: &mut HashSet<i32>,
        visited: &mut HashSet<i32>,
    ) -> bool {
        if visiting.contains(&index) {
            return true;
        }
        if visited.contains(&index) {
            return false;
        }
        visiting.insert(index);
        for &nei in &graph[index as usize] {
            if Self::has_cycle(nei, graph, visiting, visited) {
                return true;
            }
        }
        visiting.remove(&index);
        visited.insert(index);
        false
    }
}
