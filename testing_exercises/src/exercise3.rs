// 题目 3：Result 返回值与 should_panic 的冲突
//
// 目标：
// 1. 理解测试函数返回 `Result<(), E>` 的用法，并使用 `?` 运算符优雅地传播错误。
// 2. 掌握为什么在返回 `Result` 的测试上绝对不能标注 `#[should_panic]`，并修复其导致的编译/逻辑问题。
//
// 运行验证命令：
// cargo test --bin testing_exercises exercise3

use std::num::ParseIntError;

pub fn parse_percentage(s: &str) -> Result<u32, String> {
    let val: u32 = s.parse().map_err(|e: ParseIntError| e.to_string())?;
    if val > 100 {
        return Err(format!("Percentage cannot exceed 100: {}", val));
    }
    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 正确使用 Result 返回值的测试
    // 如果返回 Ok(()) 测试通过，如果返回 Err 测试失败，我们可以用 `?` 连缀
    #[test]
    fn test_parse_valid() -> Result<(), String> {
        let val = parse_percentage("85")?;
        assert_eq!(val, 85);
        Ok(())
    }

    // TODO: 观察下面的测试。我们的目标是测试输入非法字符串时能够被正确拒绝。
    // 但是这里既声明了返回 `Result`，又标注了 `#[should_panic]`，这在编译时是冲突的，或者在逻辑上无法按预期通过。
    // 笔记提示：“Err 并不会触发系统的 panic 恐慌，而是被测试框架作为返回值优雅捕获，因此即使返回了 Err 也无法触发 should_panic 的通过判定。”
    //
    // 请修复这个测试，使其在得到 Err 时判断测试成功通过，且**不使用** `#[should_panic]`。
    #[test]
    #[should_panic]
    fn test_parse_invalid() -> Result<(), String> {
        let _ = parse_percentage("invalid_number")?;
        Ok(())
    }
}
