
### String Compression :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/string-compression](https://leetcode-cn.com/problems/string-compression)
- 执行时间/Runtime: 8 ms 
- 内存消耗/Mem Usage: 2.5 MB
- 提交日期/Datetime: 2019-03-06 16:03

```rust
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut repeat_num = 1;
        let mut i = 1;
        while i < chars.len() {
            if chars[i - 1] == chars[i] {
                repeat_num += 1;
            } else {
                if repeat_num >= 2 {
                    let repeat_num_char = repeat_num.to_string();
                    chars.splice((i - repeat_num + 1)..i, repeat_num_char.chars());
                    i = i - repeat_num + 1 + repeat_num_char.len();
                }
                repeat_num = 1;
            }
            i += 1;
        }

        if repeat_num >= 2 {
            chars.splice((chars.len() - repeat_num + 1)..chars.len(), repeat_num.to_string().chars());
        }

        chars.len() as i32
    }
}

```
