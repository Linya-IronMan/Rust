# Rust 测试专题练习（仿 Rustlings 风格）

本练习专门用于验证和巩固您对 Rust 测试机制（包括单元测试、断言约束、边界测试、异常处理、测试运行参数、集成测试架构及局限性）的理解。

由于工程中故意设计了多处**编译错误**和**逻辑漏洞**，直接运行 `cargo test` 可能会因为早期编译错误而中断。建议您**按顺序（从题目 1 到题目 5）**逐一修复代码并运行测试。

---

## 🧭 做题指南与步骤

### 题目 1：单元测试基础与断言限制
* **物理位置**：[`src/exercise1.rs`](file:///Users/linya/Code/Self/Rust/testing_exercises/src/exercise1.rs)
* **背景**：我们想在测试中比较两个自定义的矩形尺寸 `BoxDimension` 是否相等。
* **现象**：当前代码尝试使用 `assert_eq!` 进行比较，但因为 `BoxDimension` 缺失必要的 Trait 而编译报错；且测试函数缺失 `#[test]` 属性，导致测试无法被 Test Runner 收集。
* **您的任务**：
  1. 修复 `BoxDimension` 的 Trait 派生，使其满足 `assert_eq!` 的要求。
  2. 给测试函数标注正确的属性宏，使其成为一个可执行的测试。
* **验证命令**：`cargo test --bin testing_exercises exercise1`

### 题目 2：边界异常与自定义错误消息
* **物理位置**：[`src/exercise2.rs`](file:///Users/linya/Code/Self/Rust/testing_exercises/src/exercise2.rs)
* **背景**：有一个限制年龄范围的函数 `validate_age`。如果年龄越界，应该触发恐慌（Panic）。
* **现象**：
  - `too_old` 测试期望捕获特定的错误消息，但是属性宏 `#[should_panic]` 的 `expected` 过滤文本配置错误，导致测试失败。
  - `assert_message_evaluation` 测试想验证断言通过时自定义格式化参数不会求值（避免不必要的副作用），目前断言条件不正确。
* **您的任务**：
  1. 修改 `too_old` 里的 `expected` 参数以匹配真实的 panic 消息。
  2. 修复 `assert_message_evaluation` 中的测试断言，确保只有测试失败时才会求值自定义错误文本。
* **验证命令**：`cargo test --bin testing_exercises exercise2`

### 题目 3：Result 返回值与 should_panic 的冲突
* **物理位置**：[`src/exercise3.rs`](file:///Users/linya/Code/Self/Rust/testing_exercises/src/exercise3.rs)
* **背景**：测试一个解析百分比的函数 `parse_percentage`。当输入错误格式时应该返回 `Err`。
* **现象**：测试函数试图同时使用返回 `Result` 机制（用 `?` 运算符）和 `#[should_panic]` 属性宏，但这在 Rust 中会导致严重的编译冲突。
* **您的任务**：理解为什么返回 `Result` 的测试在发生错误时不会触发系统的 Panic。修复该测试，使其在返回 `Err` 时正确被测试框架判定为失败，而不使用 `#[should_panic]`。
* **验证命令**：`cargo test --bin testing_exercises exercise3`

### 题目 4：并发测试的状态冲突与过滤参数
* **物理位置**：[`tests/exercise4.rs`](file:///Users/linya/Code/Self/Rust/testing_exercises/tests/exercise4.rs)
* **背景**：有两个测试会向同一个外部文件写入数据并读取。如果默认以多线程并发跑，两个测试会互相干扰，产生间歇性失败。另外还有一个耗时的测试被 `#[ignore]` 标记。
* **现象**：直接运行该文件测试可能会因为冲突而失败，且 `#[ignore]` 的测试默认没有运行。
* **您的任务**：
  1. 理解多线程共享状态的危害。尝试使用控制执行文件的参数，让该文件的测试**以单线程串行模式**稳定执行并通过。
  2. 使用命令行参数单独拉起被忽略的测试（且该测试内部有待修复的 bug，运行后需要将其修复）。
* **验证命令**：
  - 串行运行测试：`cargo test --test exercise4 -- --test-threads=1`
  - 单独运行被忽略的测试：`cargo test --test exercise4 -- --ignored`

### 题目 5：集成测试架构与 Binary Crate 局限性
* **物理位置**：[`tests/exercise5.rs`](file:///Users/linya/Code/Self/Rust/testing_exercises/tests/exercise5.rs) 和 [`tests/common.rs`](file:///Users/linya/Code/Self/Rust/testing_exercises/tests/common.rs)
* **背景**：此工程目前是一个纯粹的 Binary Crate。现在我们想在外部 `tests/` 目录为其编写集成测试，并且共享一些初始化辅助逻辑。
* **现象**：
  - 编译时报错：集成测试 `tests/exercise5.rs` 无法通过 `use testing_exercises::...` 导入二进制工程的函数。
  - Cargo 给出警告：错放位置的 `tests/common.rs` 被当成了独立的集成测试 Crate，提示缺少 main 并且在每次 `cargo test` 时都会被多余编译。
* **您的任务**：
  1. 将该工程重构为 `src/lib.rs` + `src/main.rs` 的混合架构（核心逻辑移至 `src/lib.rs` 中，`src/main.rs` 仅作为胶水调用），使外部集成测试能顺利导入。
  2. 将 `tests/common.rs` 迁移至规范的老式子模块目录中（`tests/common/mod.rs`），并在集成测试中通过 `mod common;` 正确引入它。
* **验证命令**：`cargo test --test exercise5`

---

祝您顺利通过所有测试！每道题目的代码中都含有详细的注释和待解决提示，请仔细阅读。
