/// # 判断回文数
/// 给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。
///
/// 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
///
/// 例如，121 是回文，而 123 不是。
/// 示例 1：
///
/// 输入：x = 121
/// 输出：true
/// 示例 2：
///
/// 输入：x = -121
/// 输出：false
/// 解释：从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
/// 示例 3：
///
/// 输入：x = 10
/// 输出：false
/// 解释：从右向左读, 为 01 。因此它不是一个回文数。
///
///
/// 提示：
///
/// -231 <= x <= 231 - 1
///
///
/// 进阶：你能不将整数转为字符串来解决这个问题吗？
fn is_palindrome(num: i32) -> bool {
    let str = num.to_string();
    let mut pre = 0;
    let mut last = str.len() - 1;

    while pre < last {
        if str.as_bytes()[pre] != str.as_bytes()[last] {
            return false;
        }

        pre = pre + 1;
        last = last - 1;
    }

    true
}

#[cfg(test)]
mod palindromic_tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
    }
}
