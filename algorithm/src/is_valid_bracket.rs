use std::collections::HashMap;

/// # 有效括号检查
///
/// 判断字符串中的括号是否有效。有效字符串需满足：
/// - 左括号必须用相同类型的右括号闭合
/// - 左括号必须以正确的顺序闭合
/// - 每个右括号都有一个对应的相同类型的左括号
///
/// # 参数
/// - `value`: 包含括号字符 '()[]{}\' 的字符串
///
/// # 返回值
/// - 如果字符串括号有效，返回 `true`
/// - 如果字符串括号无效，返回 `false`
///
/// # 示例
/// ```
/// use algorithm::is_valid_bracket::is_valid_bracket;
///
/// // 示例 1
/// assert_eq!(is_valid_bracket("()"), true);
///
/// // 示例 2
/// assert_eq!(is_valid_bracket("()[]{}"), true);
///
/// // 示例 3
/// assert_eq!(is_valid_bracket("(]"), false);
///
/// // 示例 4
/// assert_eq!(is_valid_bracket("([])"), true);
///
/// // 示例 5
/// assert_eq!(is_valid_bracket("([)]"), false);
/// ```
///
/// # 约束条件
/// - 1 <= s.length <= 104
/// - s 仅由括号 '()[]{}' 组成
pub fn is_valid_bracket(value: &str) -> bool {
    let dic = HashMap::from([('}', '{'), (')', '('), (']', '[')]);
    let mut stack = Vec::new();

    for c in value.chars() {
        if stack.is_empty() {
            stack.push(c)
        } else {
            let last = stack.last().unwrap();
            match dic.get(&c) {
                Some(cc) if cc.eq(&last) => {
                    stack.pop();
                    ()
                }
                _ => {
                    stack.push(c);
                    ()
                }
            }
        }
        // println!("{:?}", stack)
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use crate::is_valid_bracket::is_valid_bracket;

    #[test]
    fn test_valid_brackets() {
        assert_eq!(is_valid_bracket("["), false);
        assert_eq!(is_valid_bracket("()[]{}"), true);
        assert_eq!(is_valid_bracket("(]"), false);
        assert_eq!(is_valid_bracket("([])"), true);
        assert_eq!(is_valid_bracket("([)]"), false);
    }
}
