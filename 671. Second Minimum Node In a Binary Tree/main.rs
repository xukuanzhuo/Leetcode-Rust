use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;
impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match &root {
            Some(node) => Solution::find_second_minimum_value_(&root, node.borrow().val),
            None => -1
        }
    }

    fn find_second_minimum_value_(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                if node.val > val {
                    return node.val;
                }
                let l = Solution::find_second_minimum_value_(&node.left, val);
                let r = Solution::find_second_minimum_value_(&node.right, val);
                if l < 0 {
                    return r;
                }
                if r < 0 {
                    return l;
                }
                cmp::min(l, r)
            },
            None => {
                -1
            }
        }
    }
}