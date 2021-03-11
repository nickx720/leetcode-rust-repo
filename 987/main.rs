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
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::BTreeMap;
        type NodeVal = Option<Rc<RefCell<TreeNode>>>;
        type BinaryX = BTreeMap<isize, BTreeMap<isize, Vec<i32>>>;
        fn recurse_me(x_y_map: &mut BinaryX, node: NodeVal, (x, y): (isize, isize)) {
            if let Some(node) = node {
                let node = node.borrow();
                let x_y_container = x_y_map.entry(x).or_default().entry(y).or_default();
                x_y_container.push(node.val);
                recurse_me(x_y_map, node.left.clone(), (x - 1, y + 1));
                recurse_me(x_y_map, node.right.clone(), (x + 1, y + 1));
            }
        }
        let mut x_y_cont = BinaryX::new();
        recurse_me(&mut x_y_cont, root.clone(), (0, 0));
        x_y_cont
            .values_mut()
            .for_each(|x| x.values_mut().for_each(|v| v.sort_unstable()));
        x_y_cont
            .values()
            .map(|x| x.values().flatten().cloned().collect())
            .collect()
    }
}
