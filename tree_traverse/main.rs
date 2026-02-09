mod traversal;
mod tree;

use std::cell::RefCell;
use std::rc::Rc;
use traversal::TreeTraverser;
use tree::TreeNode;

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
    })));

    let result = TreeTraverser::preorder_traversal(root);
    println!("{:?}", result); // Should print [1, 2, 3]
}
