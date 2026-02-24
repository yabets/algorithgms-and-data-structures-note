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
    // ascii tree:
    //     1
    //      \
    //       2
    //      /
    //     3

    let result = TreeTraverser::preorder_traversal(root.clone());
    println!("Preorder traversal: {:?}", result); // Should print [1, 2, 3]
    let result = TreeTraverser::inorder_traversal(root.clone());
    println!("Inorder traversal: {:?}", result); // Should print [1, 3, 2]
}
