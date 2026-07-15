// 题目 2：边界异常与自定义错误消息
//
// 目标：
// 1. 掌握 `#[should_panic]` 以及细粒度 expected 匹配的配置方法。
// 2. 掌握断言宏的自定义错误消息语法，并理解自定义格式化消息的“延迟/仅在失败时求值”的特性。
//
// 运行验证命令：
// cargo test --bin testing_exercises exercise2

use std::sync::atomic::{AtomicU32, Ordering};

pub fn validate_age(age: i32) {
    if age < 0 {
        panic!("Age cannot be negative: {}", age);
    } else if age > 150 {
        panic!("Age {} is too old for a human!", age);
    }
}

// 模拟一个带副作用的错误信息获取函数
static SideEffectCounter: AtomicU32 = AtomicU32::new(0);

fn get_error_message(detail: &str) -> String {
    // 每次被调用时，计数器加 1
    SideEffectCounter.fetch_add(1, Ordering::SeqCst);
    format!("Error detail: {}", detail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_negative_age() {
        validate_age(-5);
    }

    // TODO: 观察下面的 `expected` 字符串，它与真实 panic 抛出的消息不一致，导致测试失败。
    // 请修改 `expected` 值，使其匹配 `validate_age(200)` 时真实 panic 消息的子串。
    #[test]
    #[should_panic(expected = "Age 200 is too old for a human!")]
    fn test_too_old_age() {
        validate_age(200);
    }

    #[test]
    fn test_assert_message_evaluation() {
        // 重置计数器
        SideEffectCounter.store(0, Ordering::SeqCst);

        let value = 42;

        // TODO: 笔记提示：自定义消息仅在断言失败时才会被格式化并求值。
        // 目前下面这句断言会失败。如果我们想让测试通过，并且让 `get_error_message` 根本不被执行（即副作用计数器为 0）：
        // 1. 请修改断言的布尔表达式，使其评估为 `true`。
        // 2. 理解为什么此时 `SideEffectCounter` 依然是 0。
        assert!(
            value == 42, // TODO: 修复这个布尔表达式
            "{}",
            get_error_message("Value is not equal to expected number")
        );

        // 验证 `get_error_message` 是否从未被求值
        assert_eq!(
            SideEffectCounter.load(Ordering::SeqCst),
            0,
            "警告！get_error_message 被意外求值了，说明断言判断可能失败或者被错误触发。"
        );
    }
}
