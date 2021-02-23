// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        l: i32,
        r: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        //         https://dev.to/seanpgallivan/solution-trim-a-binary-search-tree-8ea
        if let Some(node_ref) = root {
            let mut node = node_ref.borrow_mut();
            if node.val > r {
                return Self::trim_bst(node.left.clone(), l, r);
            } else if node.val < l {
                return Self::trim_bst(node.right.clone(), l, r);
            };
            node.left = Self::trim_bst(node.left.clone(), l, r);
            node.right = Self::trim_bst(node.right.clone(), l, r);
            Some(node_ref.clone())
        } else {
            None
        }
    }
}
