// 题目 5：集成测试架构与 Binary Crate 局限性
//
// 目标：
// 1. 验证对于纯 Binary Crate（只有 src/main.rs）无法编写集成测试的现象，理解为什么必须拆分出 `src/lib.rs`。
// 2. 物理重构项目：新建 `src/lib.rs` 并将计算逻辑 `calculator_add` 迁移过去，以便集成测试能通过 `use testing_exercises::...` 导入。
// 3. 将错放的共享测试模块 `tests/common.rs` 移入 `tests/common/mod.rs` 并在此处引入和调用。
//
// 运行验证命令：
// cargo test --test exercise5

// TODO 1: 运行上面的验证命令，您会遇到编译报错，无法解析或链接 `testing_exercises`。
// 请在 `testing_exercises/` 根目录下新建 `src/lib.rs`，将原先在 `src/main.rs` 中的
// `pub fn calculator_add` 函数迁移至 `src/lib.rs` 中，并确保 `src/lib.rs` 同样注册了题目 1、2、3。
use testing_exercises::calculator_add;

// TODO 2: 此时如果直接在 `tests/` 目录下用 `mod common;` 引用 `tests/common.rs`，
// Cargo 会在每次 `cargo test` 时发出警告或将其编译为独立的测试 Crate（提示缺少测试用例等）。
// 请在物理上创建 `tests/common/` 目录，将 `tests/common.rs` 重命名移动到 `tests/common/mod.rs`。
// 移动完毕后，在此处取消下面的注释，以便通过旧版模块物理路径引用共享模块。
// mod common;

#[test]
fn test_integration_calculator() {
    // TODO 3: 当把 common.rs 移动到 tests/common/mod.rs 后，取消下面两行的注释，
    // 调用共享的初始化环境函数，并断言其返回值。
    // let db_conn = common::setup_test_context();
    // assert_eq!(db_conn, "MOCK_DB_CONNECTION_STRING");

    // 运行被测库的公开逻辑
    let result = calculator_add(100, 200);
    assert_eq!(result, 300);
}
