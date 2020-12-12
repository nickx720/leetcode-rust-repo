impl Solution {
    fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut headZ = head;
        let mut output: Vec<i32> = Vec::new();
        // consume each value of the node and push it into the output array
        while let Some(value) = headZ {
            output.push(value.val);
            headZ = value.next;
        }
        let mut ans: Vec<i32> = vec![0; output.len()];
        let mut stack = vec![];
        for val in 0..output.len() {
            while stack.len() > 0 && output[*stack.last().unwrap()] < output[val] {
                ans[stack.pop().unwrap()] = output[val];
            }
            stack.push(val);
        }
        ans
    }
}
