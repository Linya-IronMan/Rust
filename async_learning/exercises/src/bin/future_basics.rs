use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

/// @struct StepFuture
/// @brief 模拟一个需要历经 3 次 Poll 才能 Ready 的步进式 Future。
struct StepFuture {
    /// 当前已经被轮询的次数，保存在受锁保护的共享空间中，方便在 poll 线程与唤醒线程间共享
    shared_state: Arc<Mutex<SharedState>>,
}

/// @struct SharedState
/// @brief StepFuture 内部的状态存根。
struct SharedState {
    /// 记录当前被 poll 的计数
    poll_count: usize,
    /// 记录用于唤醒的 waker 信号
    waker: Option<Waker>,
}

impl Future for StepFuture {
    type Output = usize;

    /// @fn poll
    /// @brief 状态机的核心驱动逻辑。
    /// @param self 处于 Pin 状态的可变自身引用，防止内存移动
    /// @param cx 随身 Context 工具包，提供 Waker 句柄
    /// @return Poll<Self::Output>
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.shared_state.lock().unwrap();

        if state.poll_count >= 3 {
            // =================================================================
            // 🚨 TODO 任务一：一旦 poll_count 达到 3，表明数据就绪，返回 Ready。
            // 提示：返回 Poll::Ready(...) 并带上最终的计数
            // =================================================================
            // 🚨 请补全下方代码，替换掉 unimplemented!()
            // unimplemented!("请完成任务一：就绪时返回 Ready(计数)")
            Poll::Ready(state.poll_count)
        } else {
            // 计数累加 1
            state.poll_count += 1;
            println!(
                "【StepFuture】第 {} 次被 poll，尚未就绪...",
                state.poll_count
            );

            // =================================================================
            // 🚨 TODO 任务二：克隆 Context 中的 waker，存入 shared_state。
            //                 然后派生一个后台工作线程在 10 毫秒后拉响该 waker，
            //                 最后返回 Pending 状态。
            // 提示：1. 拷贝 waker: cx.waker().clone() 并存入 state.waker
            //       2. 启动后台线程 thread::spawn，在 sleep 10ms 后，
            //          抢占 Mutex 锁并调用 waker.wake() 触发重新轮询。
            //       3. 返回 Poll::Pending
            // =================================================================
            // 🚨 请补全下方代码，替换掉 unimplemented!()
            // unimplemented!("请完成任务二：克隆 waker、派生后台线程并在 sleep 后执行唤醒，最后返回 Pending")
            state.waker = Some(cx.waker().clone());

            let state_clone = self.shared_state.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(1000));
                let mut state = state_clone.lock().unwrap();
                if let Some(waker) = state.waker.take() {
                    waker.wake(); // 🚨 重排队触发！
                }
            });

            Poll::Pending
        }
    }
}

/// @fn main
/// @brief 亲手实现 Future 状态流转与 Context/Waker 绑定。
fn main() {
    let shared_state = Arc::new(Mutex::new(SharedState {
        poll_count: 0,
        waker: None,
    }));
    let future = StepFuture {
        shared_state: shared_state.clone(),
    };

    println!("--- 开始运行自定义 StepFuture 轮询测试 ---");
    // 我们使用 futures 库的 block_on 作为一个微型执行器来驱动当前 Future
    let final_result = futures::executor::block_on(future);

    println!("【主线程】Future 运行完毕！最终返回值: {}", final_result);
    assert_eq!(final_result, 3);
    assert_eq!(shared_state.lock().unwrap().poll_count, 3);
    println!("🎉 恭喜！Future & Context 底层唤醒练习完美通过！");
}

/* =============================================================================
📖 参考答案说明（请在思考后再查看）：
1. 任务一：
   直接返回 `Poll::Ready(state.poll_count)` 结束 Future 的状态机。
2. 任务二：
   ```rust
   // 1. 克隆 Waker 并覆盖保存
   state.waker = Some(cx.waker().clone());

   // 2. 后台唤醒线程派生
   let state_clone = self.shared_state.clone();
   thread::spawn(move || {
       thread::sleep(Duration::from_millis(10));
       let mut state = state_clone.lock().unwrap();
       if let Some(waker) = state.waker.take() {
           waker.wake(); // 🚨 重排队触发！
       }
   });

   // 3. 返回挂起状态
   Poll::Pending
   ```
============================================================================= */
