use crate::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeTraverser;

impl TreeTraverser {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack = Vec::new();

        stack.push(root);

        while let Some(cur) = stack.pop() {
            if let Some(rc) = cur {
                let (left, val, right) = {
                    let n = rc.borrow();
                    (n.left.clone(), n.val, n.right.clone())
                };
                stack.push(right);
                stack.push(left);
                res.push(val);
            }
        }
        res
    }
}
