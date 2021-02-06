impl Solution {
    fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_depth = max_depth(node.clone().borrow().left.clone());
            let right_depth = max_depth(node.clone().borrow().right.clone());
            let max_depth_val = std::cmp::max(left_depth, right_depth) + 1;
            max_depth_val
        } else {
            0
        }
    }
}
