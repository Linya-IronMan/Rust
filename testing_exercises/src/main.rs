// 声明单元测试模块，以供 Cargo 编译并执行其中的测试
mod exercise1;
mod exercise2;
mod exercise3;

use testing_exercises::calculator_add;

fn main() {
    println!("欢迎来到 Rust 测试专题练习！");
    println!("请仔细阅读 README.md 并根据指引完成各个练习文件中的 TODO 任务。");
    println!("例如，下面调用了一个本该作为库函数导出的计算器相加逻辑：");
    println!("100 + 200 = {}", calculator_add(100, 200));
}
