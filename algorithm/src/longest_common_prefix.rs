/// # 查找字符串数组中的最长公共前缀
///
/// 该函数遍历字符串数组，寻找所有字符串共有的最长前缀。
/// 如果不存在公共前缀或数组为空，返回空字符串。
///
/// # 参数
/// - `strs`: 包含多个字符串的向量，用于查找公共前缀
///
/// # 返回值
/// - 返回一个字符串，表示所有输入字符串的最长公共前缀
///
/// # 示例
/// ```
/// use algorithm::longest_common_prefix;
///
/// // 示例 1
/// let result1 = longest_common_prefix(vec![
///     String::from("flower"),
///     String::from("flow"),
///     String::from("flight")
/// ]);
/// assert_eq!(result1, "fl");
///
/// // 示例 2
/// let result2 = longest_common_prefix(vec![
///     String::from("dog"),
///     String::from("racecar"),
///     String::from("car")
/// ]);
/// assert_eq!(result2, "");
/// ```
///
/// # 约束条件
/// - 1 <= strs.length <= 200
/// - 0 <= strs[i].length <= 200
/// - strs[i] 如果非空，则仅由小写英文字母组成
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::from("");
    }
    let mut common = String::from("");
    // TODO: 闭包
    let min_len = strs.iter().map(|str| str.len()).min().unwrap();
    let mut i = 0;

    while i < min_len {
        // TODO: unwrap 处理
        if let Some(cur_char) = strs[0].chars().nth(i) {
            let matched = strs
                .iter()
                .enumerate()
                .all(|(_index, str)| str.chars().nth(i).unwrap().eq(&cur_char));

            if !matched {
                break;
            }
            common.push(cur_char)
        } else {
            break;
        }
        i += 1;
    }

    common
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            String::from("fl")
        );
    }
}
