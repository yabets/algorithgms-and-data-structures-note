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

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack = Vec::new();
        let mut cur = root;

        while cur.is_some() || !stack.is_empty() {
            // println!("{:?}", cur);
            while let Some(rc) = cur {
                stack.push(rc.clone());
                cur = {
                    let node = rc.borrow();
                    node.left.clone()
                };
            }
            cur = stack.pop();
            if let Some(ref rc) = cur {
                let (val, right) = {
                    let node = rc.borrow();
                    (node.val, node.right.clone())
                };
                res.push(val);
                cur = right;  
            };
        }

        res
    }
}
