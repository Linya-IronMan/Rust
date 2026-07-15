// 题目 5 的辅助共享模块（错放的位置）
//
// TODO: 笔记中指出：
// “在 `tests/` 目录下，如果有很多辅助测试用例的公共初始化函数，如果直接写成 `tests/common.rs`，
// Cargo 会误以为这也是一个集成的测试文件而尝试为其编译并触发没有 main 入口或无测试用例的警告。”
//
// 为了避免这种误判，您需要：
// 1. 将本文件移动到旧版模块物理路径下：`tests/common/mod.rs`
// 2. 然后在 `tests/exercise5.rs` 中通过 `mod common;` 引用。

pub fn setup_test_context() -> &'static str {
    println!("初始化集成测试模拟环境（如创建临时数据库、加载配置桩等）...");
    "MOCK_DB_CONNECTION_STRING"
}
