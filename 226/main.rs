impl Solution {
    fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root.as_ref() {
            let mut root = root.borrow_mut();
            invert_tree(root.left.as_ref().map(Rc::clone));
            invert_tree(root.right.as_ref().map(Rc::clone));
            let temp = root.left.clone();
            root.left = root.right.clone();
            root.right = temp;
        }
        root
    }
}
