// 题目 4：并发测试下的状态冲突与过滤运行
//
// 目标：
// 1. 理解默认下 Rust 测试的多线程并行机制对共享状态（如文件、全局变量）的危害。
// 2. 学会通过控制底层 runner 参数限制为单线程串行模式，确保共享状态下的测试稳定性。
// 3. 掌握 `#[ignore]` 属性以及如何单独拉起被忽略的测试。
//
// 运行与验证说明：
// - 直接并行运行测试可能会因为文件并发写入冲突导致测试间歇性失败：
//   cargo test --test exercise4
//
// - TODO 1：请尝试在命令行输入以下命令，以单线程串行运行，确保 test_writer_a 与 test_writer_b 不发生冲突：
//   cargo test --test exercise4 -- --test-threads=1
//
// - TODO 2：使用下面命令单独运行被 ignore 的测试，并修复其代码中的断言 BUG 使得测试通过：
//   cargo test --test exercise4 -- --ignored

use std::fs;
use std::thread;
use std::time::Duration;

const TEMP_FILE: &str = "temp_shared_state.txt";

fn write_and_verify(data: &str) {
    // 写入共享的物理文件
    fs::write(TEMP_FILE, data).expect("无法写入测试文件");

    // 故意加上小幅睡眠延迟，以显著提高并发运行下 test_writer_a 和 test_writer_b 发生冲突的概率
    thread::sleep(Duration::from_millis(50));

    let content = fs::read_to_string(TEMP_FILE).expect("无法读取测试文件");

    // 断言读取出来的数据必须等于刚刚写入的数据
    assert_eq!(content, data, "测试冲突！读取到的数据与写入不一致。");

    // 清除临时文件
    let _ = fs::remove_file(TEMP_FILE);
}

#[test]
fn test_writer_a() {
    write_and_verify("STATE_DATA_AAAAAA");
}

#[test]
fn test_writer_b() {
    write_and_verify("STATE_DATA_BBBBBB");
}

// 模拟极其耗时或暂时关闭的测试，标注了 #[ignore]
#[test]
#[ignore]
fn test_expensive_simulation() {
    println!("这行输出只有在使用了 --show-output 或者特定运行时才能在通过时看到");

    let x = 10;
    let y = 20;
    let sum = x + y;

    // TODO: 这是一个故意的错误，请在通过命令行参数拉起此测试后将其修改正确，使测试通过。
    assert_eq!(sum, 30, "求和计算不正确！");
}
