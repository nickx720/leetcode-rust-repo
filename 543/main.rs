impl Solution {
    fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_some() {
            let (_max_depth, max_diameter) = max_depth_diameter(&root);
            max_diameter
        } else {
            0
        }
    }

    fn max_depth_diameter(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = root {
            let node = node.borrow();

            let (left_depth, left_diameter) = max_depth_diameter(&node.left);
            let (right_depth, right_diameter) = max_depth_diameter(&node.right);

            let max_depth = 1 + std::cmp::max(left_depth, right_depth);
            let max_diameter = [left_diameter, right_diameter, left_depth + right_depth]
                .iter()
                .max()
                .unwrap()
                .clone();

            (max_depth, max_diameter)
        } else {
            (0, 0)
        }
    }
}
