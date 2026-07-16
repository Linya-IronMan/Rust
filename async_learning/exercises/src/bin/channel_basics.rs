use crossbeam_channel::{Receiver, Sender};
use std::thread;
use std::time::Duration;

/// @fn main
/// @brief 练习使用 crossbeam-channel 搭建多生产者多消费者任务传输管道。
/// @details 本练习的任务是：
///          1. 创建一个 MPMC 通道。
///          2. 派生 2 个并发 Worker 线程，不断从通道中拉取任务进行处理。
///          3. 主线程向通道注入 5 个任务，并正确关闭通道以防止子线程死锁阻塞不退出。
fn main() {
    // =========================================================================
    // 🚨 TODO 任务一：使用 crossbeam_channel 创建一个无边界任务传输通道。
    // 提示：调用 crossbeam_channel::unbounded() 函数，解构成 (tx, rx)
    // =========================================================================
    // 🚨 请解除下方注释并补全代码，替换掉 unimplemented!()
    // let (tx, rx): (Sender<String>, Receiver<String>) = ......;
    let (tx, rx): (Sender<String>, Receiver<String>) = crossbeam_channel::unbounded();

    let mut worker_handles = vec![];

    // 启动 2 个消费者工作线程
    for worker_id in 0..2 {
        // 克隆接收端以便多个消费者线程并发抢单
        let rx_clone = rx.clone();

        let handle = thread::spawn(move || {
            println!("【工作线程 {}】启动，正在等待任务...", worker_id);
            // =================================================================
            // 🚨 TODO 任务二：让工作线程进入循环，阻塞拉取任务，直至通道关闭。
            // 提示：使用 while let Ok(task) = rx_clone.recv() 循环消费。
            //       当通道没有任务且发送端全部释放时，recv() 会返回 Err 自动退出循环。
            // =================================================================
            // 🚨 请补全下方代码，替换掉 unimplemented!()
            // unimplemented!("请在此处实现基于 while let 的阻塞接收循环，打印接收到的任务，并模拟处理延时（如sleep 10ms）");
            while let Ok(task) = rx_clone.recv() {
                println!("【工作线程 {}】接收到任务: {}", worker_id, task);
                thread::sleep(Duration::from_millis(10)); // 模拟处理延时
            }
        });
        worker_handles.push(handle);
    }

    // 丢弃本地 rx 变量的引用，确保只有子线程持有 Receiver 的快捷方式副本。
    // 这一步有利于通道生命周期的精准管理。
    drop(rx);

    // 主线程（生产者）开始往传送带里注入 5 个大任务
    for task_id in 1..=5 {
        let task_name = format!("Task #{}", task_id);
        println!("【主线程】向管道塞入任务: {}", task_name);

        // 往通道发送任务
        tx.send(task_name).expect("无法发送任务，通道已关闭");
        thread::sleep(Duration::from_millis(5)); // 模拟点单间隔
    }

    // =========================================================================
    // 🚨 TODO 任务三：主线程任务分发完毕后，必须关闭通道，使子线程退出 recv()。
    // 提示：如何宣告发送端生命期结束？
    //       在 Rust 中只需主动 drop 掉主线程唯一的发送端 tx！
    // =========================================================================
    // 🚨 请补全下方代码，替换掉 unimplemented!()
    // unimplemented!("请在此处 drop 掉 tx 变量");
    drop(tx);

    // 等待所有 Worker 线程处理完积压任务后安全退出
    for handle in worker_handles {
        handle.join().unwrap();
    }

    println!("🎉 恭喜！Crossbeam Channel 传送带练习完美通过！");
}

/* =============================================================================
📖 参考答案说明（请在思考后再查看）：
1. 任务一：
   `let (tx, rx) = crossbeam_channel::unbounded();`
2. 任务二：
   ```rust
   while let Ok(task) = rx_clone.recv() {
       println!("【工作线程 {}】开始处理任务: {}", worker_id, task);
       thread::sleep(Duration::from_millis(10)); // 模拟做菜用时
       println!("【工作线程 {}】完成任务: {}", worker_id, task);
   }
   ```
3. 任务三：
   `drop(tx);`
   主线程调用 `drop(tx)` 后，整个通道中将不存在任何活跃的 Sender。
   此时当就绪队列被消费空后，子线程调用 `rx_clone.recv()` 就会立即返回 `Err(RecvError)`，
   从而安全跳出 `while let Ok` 事件循环，防止子线程永久阻塞死锁。
============================================================================= */
