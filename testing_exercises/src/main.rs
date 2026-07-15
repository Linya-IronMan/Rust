// 声明单元测试模块，以供 Cargo 编译并执行其中的测试
mod exercise1;
mod exercise2;
// mod exercise3;

// TODO (针对题目 5)：
// 集成测试（如 `tests/exercise5.rs`）是作为外部客户端调用的，它们无法直接 `use` 一个二进制项目的内部函数。
// 换句话说，像下面这个 `calculator_add` 函数，如果只留在 `src/main.rs`，外部测试是绝对无法引用的。
//
// 您的任务：
// 1. 在 `src/` 目录下新建 `src/lib.rs`。
// 2. 将 `calculator_add` 剪切到 `src/lib.rs` 中，并保持其为 `pub`。
// 3. 同时把 `mod exercise1;`、`mod exercise2;`、`mod exercise3;` 声明也移到 `src/lib.rs` 中。
// 4. 在 `src/main.rs` 中通过 `testing_exercises::calculator_add` 来调用，以保持二进制项目的功能一致。
pub fn calculator_add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("欢迎来到 Rust 测试专题练习！");
    println!("请仔细阅读 README.md 并根据指引完成各个练习文件中的 TODO 任务。");
    println!("例如，下面调用了一个本该作为库函数导出的计算器相加逻辑：");
    println!("100 + 200 = {}", calculator_add(100, 200));
}
