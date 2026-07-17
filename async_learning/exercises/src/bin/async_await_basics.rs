use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

/// 一个极简的定时器 Future，用于在不阻塞当前线程的前提下实现异步等待。
struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// 共享状态，用于在后台线程与 Future 之间同步完成标记与 Waker。
struct SharedState {
    /// 任务是否已完成
    completed: bool,
    /// 用于通知执行器重新 poll 的 Waker
    waker: Option<std::task::Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    /// 轮询 Future 状态。
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // 将当前执行上下文中的 waker 存入共享状态中，
            // 当后台线程睡眠结束后，可以通过此 waker 唤醒执行器重新 poll
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    /// 创建一个新的 TimerFuture，指定其在后台线程中等待的时长。
    ///
    /// # 参数
    /// * `duration` - 后台线程等待时间
    fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        // 启动后台线程来模拟非阻塞的异步计时器
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            // waker.take() 有什么用?
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
}

/// 阶段 1: 演示 Future 的惰性求值特性 (Lazy Evaluation)。
///
/// 观察控制台输出，理解为什么未被 `.await` 或未被执行器驱动的异步函数不会真正执行。
fn stage_1_lazy_future() {
    println!("\n=== 阶段 1: Future 的惰性求值 ===");

    // 调用一个普通的同步函数，控制台会立刻打印日志
    sync_hello();

    // ------------------------------------------------------------------
    // TODO 练习 1: 调用下面的异步函数 `async_hello()`，但不加 `.await`
    // 观察控制台是否会输出 "[async] Hello from async fn!"？
    // 提示：你可以声明一个变量来接收它，如 `let future = ...;`
    // ------------------------------------------------------------------
    let future = async_hello();
    println!(
        "  -> 调用 async_hello() 后返回了一个 Future，其大小: {} 字节",
        std::mem::size_of_val(&future)
    );

    // ------------------------------------------------------------------
    // TODO 练习 2: 用 `futures::executor::block_on` 驱动上述返回的 Future 运行
    // 提示：block_on 会阻塞当前线程直到 Future 运行完毕并返回结果
    // ------------------------------------------------------------------
    println!("  -> 现在使用 block_on 驱动 Future 运行：");
    futures::executor::block_on(future);
}

/// 阶段 2: 演示异步任务的串行 (Serial) 挂起与等待。
///
/// 学习如何在异步块中，通过 `.await` 操作符按顺序挂起并等待两个独立的异步任务。
fn stage_2_serial_await() {
    println!("\n=== 阶段 2: 串行 .await 挂起等待 ===");

    // 我们使用 block_on 在同步环境中运行这个异步块
    futures::executor::block_on(async {
        println!("  -> 开始执行串行任务...");
        let start = std::time::Instant::now();

        // ------------------------------------------------------------------
        // TODO 练习 3: 分别按顺序 `.await` 两个任务：`delay_print(1, "A")` 和 `delay_print(2, "B")`
        // 观察总耗时是否是这两个任务的耗时总和（1秒 + 2秒 = 3秒）？
        // ------------------------------------------------------------------
        delay_print(1, "A").await;
        delay_print(3, "B").await;

        println!("  -> 串行任务全部完成，总共耗时: {:?}", start.elapsed());
    });
}

/// 阶段 3: 演示使用 `futures::join!` 并发运行多个 Future。
///
/// 相比于串行 await，并发运行多个 Future 可以在非阻塞的情况下提高整体效率。
fn stage_3_concurrent_join() {
    println!("\n=== 阶段 3: 使用 futures::join! 并发执行 ===");

    futures::executor::block_on(async {
        println!("  -> 开始并发执行任务 A 和 B...");
        let start = std::time::Instant::now();

        // ------------------------------------------------------------------
        // TODO 练习 4: 使用 `futures::join!` 宏并发执行 `delay_print(1, "A")` 和 `delay_print(2, "B")`
        // 观察控制台输出并对比总耗时。是否应该接近单个最长任务的耗时（约 2 秒）？
        // 提示：
        // 1. 先声明两个 Future（注意不要立即对其使用 .await 导致退化为串行）：
        //    let fut_a = delay_print(1, "A");
        //    let fut_b = delay_print(2, "B");
        // 2. 然后通过 futures::join!(fut_a, fut_b); 同时运行它们
        // ------------------------------------------------------------------
        let fut_a = delay_print(1, "A");
        let fut_b = delay_print(2, "B");

        futures::join!(fut_a, fut_b);

        println!("  -> 并发任务全部完成，总共耗时: {:?}", start.elapsed());
    });
}

/// 一个普通的同步函数，调用即执行。
fn sync_hello() {
    println!("  [sync] Hello from sync fn!");
}

/// 一个简单的异步函数。调用它不会立即执行，而是返回一个实现 `Future` 的状态机。
async fn async_hello() {
    println!("  [async] Hello from async fn!");
}

/// 模拟一个具有延迟的异步打印任务。
///
/// # 参数
/// * `secs` - 延迟秒数
/// * `label` - 任务标识标签
async fn delay_print(secs: u64, label: &str) {
    println!("  [Task {}] 启动 (计划耗时 {} 秒)...", label, secs);
    // 使用非阻塞的 TimerFuture 进行等待
    TimerFuture::new(Duration::from_secs(secs)).await;
    println!("  [Task {}] 完成！", label);
}

/// 练习程序主入口。
fn main() {
    println!("==========================================");
    println!("  开始 Rust 原生 async/await 基础练习");
    println!("==========================================");

    stage_1_lazy_future();
    stage_2_serial_await();
    stage_3_concurrent_join();

    println!("\n==========================================");
    println!("  恭喜！async/await 基础练习执行完毕");
    println!("==========================================");
}
