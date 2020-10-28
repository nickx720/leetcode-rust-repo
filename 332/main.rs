use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph: HashMap<&str, BinaryHeap<Reverse<&str>>> = HashMap::new();
        for item in tickets.iter() {
            graph
                .entry(&item[0])
                .or_insert_with(BinaryHeap::new)
                .push(Reverse(&item[1]));
        }
        let mut answers: Vec<String> = Vec::with_capacity(tickets.len() + 1);
        let mut stack = vec!["JFK"];
        while let Some(value) = stack.last() {
            if let Some(val) = graph.get_mut(value) {
                if !val.is_empty() {
                    if let Some(v) = val.pop() {
                        stack.push(v.0);
                    }
                    continue;
                }
            }
            if let Some(item) = stack.pop() {
                answers.push(item.to_string());
            }
        }
        answers.reverse();
        answers
    }
}
