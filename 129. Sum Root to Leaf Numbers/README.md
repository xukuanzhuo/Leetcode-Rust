
### Sum Root to Leaf Numbers :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/sum-root-to-leaf-numbers](https://leetcode-cn.com/problems/sum-root-to-leaf-numbers)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.5 MB
- 提交日期/Datetime: 2019-03-10 00:17

```rust
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ret = 0;
        Solution::sum_numbers_(&root, 0, &mut ret);
        ret
    }
    fn sum_numbers_(root: &Option<Rc<RefCell<TreeNode>>>, num: i32, sum: &mut i32) {
        match root {
            Some(node) => {
                let node = node.borrow();
                let num = num * 10 + node.val;
                if node.left.is_none() && node.right.is_none() {
                    *sum += num;
                }
                Solution::sum_numbers_(&node.left, num, sum);
                Solution::sum_numbers_(&node.right, num, sum);
            },
            None => {}
        }

    }
}

```
