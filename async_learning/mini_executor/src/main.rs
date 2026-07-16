use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};
use crossbeam_channel::{Receiver, Sender};
use futures::task::{waker_ref, ArcWake};

/// ============================================================================
/// 1. 异步定时器 Future (TimerFuture)
/// ============================================================================

/// @struct TimerFuture
/// @brief 模拟非阻塞 I/O 的自定义定时器 Future。
/// @details 当 Future 被 poll 时，它会检查共享状态中的已完成标志。
///          如果未完成，它会派生一个后台 OS 线程在指定延时后通过 Waker 唤醒执行器。
struct TimerFuture {
    /// 共享的状态信息，受 Mutex 保护以便在主线程（Executor 轮询）和后台睡眠线程间安全传递
    shared_state: Arc<Mutex<SharedState>>,
}

/// @struct SharedState
/// @brief 定时器 Future 与后台工作线程共享的物理状态。
struct SharedState {
    /// 标识定时睡眠任务是否已经彻底完成
    completed: bool,
    /// 存储用于唤醒当前 Future 的 waker。
    /// 当后台线程完成 sleep 后，将通过此 waker 触发重新 poll
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    /// @fn poll
    /// @brief 驱动 Future 状态向前流转的核心方法。
    /// @param self 处于 Pin 锚定状态下的可变借用，防止自引用指针由于内存移动而失效
    /// @param cx 异步执行上下文，承载了当前任务的 Waker 指针
    /// @return Poll<Self::Output> 返回 Ready(()) 表示任务完成，返回 Pending 表示任务挂起
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            // 状态转移：从 Pending 走向 Ready，标志当前 Future 生命周期终结
            Poll::Ready(())
        } else {
            // 状态转移：尚无就绪数据，需要将当前 Context 中的 waker 深度克隆保存下来。
            // 后续底层数据（或睡眠时间）就绪后，将调用此 waker 唤醒任务
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    /// @fn new
    /// @brief 构造并初始化一个新的 TimerFuture。
    /// @param duration 延迟睡眠的时间长度
    /// @return TimerFuture
    /// @details 此方法会主动生成一个后台线程。该线程会在指定 Duration 后，
    ///          将 completed 状态修改为 true，并触发 waker 的 wake() 通知。
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            // 模拟底层非阻塞 I/O（如网卡就绪或定时中断）的等待过程
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            
            // 唤醒机制触发：当 sleep 结束，主动提取出之前保存在状态中的 waker 并唤醒。
            // 这将触发 Executor 将包裹此 Future 的 Task 重新丢入执行队列
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
}

/// ============================================================================
/// 2. 任务载体定义 (Task)
/// ============================================================================

/// @struct Task
/// @brief 异步任务的物理载体，连接 Future 与运行时的纽带。
/// @details 它负责将任意实现了 Future 的匿名类型进行擦除（Box），
///          并提供能够在线程间安全共享的同步控制。
struct Task {
    /// 待执行的 Future。因为 Future 在 poll 时会被转换为状态机，
    /// 里面可能存在自引用字段，因此必须使用 Pin 锁死其在堆上的物理地址
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>,
    /// 任务发送通道，用于在 wake() 被触发时，将自身重新送回 Executor 的就绪队列
    task_sender: Sender<Arc<Task>>,
}

/// 为 Task 实现 ArcWake，使其能够与标准库的 Waker 机制融合。
/// 这是从“自制 Waker”走向“标准 Waker”的关键特征。
impl ArcWake for Task {
    /// @fn wake_by_ref
    /// @brief 引用唤醒机制。当底层资源就绪调用 waker.wake() 时，实际会重定向到此方法。
    /// @param arc_self 代表当前 Task 自身的 Arc 智能指针
    /// @details 物理操作是克隆当前 Task 的 Arc 指针，并重新发送回任务就绪通道。
    ///          这实现了协作式多任务在数据就绪后的“重新排队”逻辑。
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        // 将自身发送回 Executor，通知其需要重新对其进行 poll 轮询
        arc_self.task_sender.send(cloned).expect("任务就绪队列已满或已关闭");
    }
}

/// ============================================================================
/// 3. 执行器与任务分发器 (Executor & Spawner)
/// ============================================================================

/// @struct Executor
/// @brief 协作式运行时的执行引擎，负责维护事件循环。
struct Executor {
    /// 接收就绪任务的通道接收端
    ready_queue: Receiver<Arc<Task>>,
}

/// @struct Spawner
/// @brief 任务派生器，用于向运行时提交全新的异步任务。
#[derive(Clone)]
struct Spawner {
    /// 发送新任务到就绪队列的通道发送端
    task_sender: Sender<Arc<Task>>,
}

impl Spawner {
    /// @fn spawn
    /// @brief 提交一个新的 Future 到运行时中执行。
    /// @param future 实现了 Future 特征的异步任务
    /// @details 物理操作是将 Future 包装为 Task 结构，并通过 Channel 发送出去。
    ///          该 Future 会在 Executor 的下一轮循环中被首次 poll。
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = Box::pin(future);
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("无法提交新任务，Channel 已满或关闭");
    }
}

impl Executor {
    /// @fn run
    /// @brief 启动异步运行时的阻塞主事件循环（Event Loop）。
    /// @details 循环不断从 ready_queue 管道中拉取（pop）就绪的 Task。
    ///          通过 waker_ref 获取 Task 对应的 Waker 引用并构建 Context，
    ///          最后调用 future 的 poll 推进状态。
    pub fn run(&self) {
        // 事件循环核心：只要通道内有就绪任务，就持续运行。
        // 当 ready_queue 中没有任务时，recv() 会阻塞当前线程，从而实现 CPU 友好型的零占用等待
        while let Ok(task) = self.ready_queue.recv() {
            // 物理取出 Future，如果任务已经 Ready 被拿走，则跳过
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 基于 waker_ref 特征，将实现了 ArcWake 的 Task 引用零成本转换为标准 Waker 引用
                let waker = waker_ref(&task);
                let mut context = Context::from_waker(&waker);
                
                // 推进状态：在此处发起对底层 Future 的 poll 轮询
                if future.as_mut().poll(&mut context).is_pending() {
                    // 如果 poll 返回 Pending，说明任务尚需等待底层 I/O。
                    // 重新把 future 放回 task 中保存，等待下一次 waker.wake() 触发重新排队
                    *future_slot = Some(future);
                }
            }
        }
    }
}

/// @fn new_executor_and_spawner
/// @brief 工厂方法，用于配套初始化 Executor 与 Spawner。
/// @return (Executor, Spawner)
fn new_executor_and_spawner() -> (Executor, Spawner) {
    // 限制队列容量（如 10000），使用 crossbeam 的多生产者多消费者 Channel 作为任务传输带
    let (task_sender, ready_queue) = crossbeam_channel::unbounded();
    (Executor { ready_queue }, Spawner { task_sender })
}

/// ============================================================================
/// 4. 运行时跑通主函数
/// ============================================================================

/// @fn main
/// @brief 主入口函数，展示异步多任务协作式调度的核心流程。
fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // 派生任务 1：模拟一个需要等待 2 秒的非阻塞延时任务
    spawner.spawn(async {
        println!("【任务 1】启动，开始等待 2 秒...");
        TimerFuture::new(Duration::from_secs(2)).await;
        println!("【任务 1】完成！已经收到 TimerFuture 的唤醒通知。");
    });

    // 派生任务 2：模拟另一个需要等待 1 秒的非阻塞延时任务
    let spawner_clone = spawner.clone();
    spawner.spawn(async move {
        println!("【任务 2】启动，开始等待 1 秒...");
        TimerFuture::new(Duration::from_secs(1)).await;
        println!("【任务 2】完成！开始派生任务 3...");
        
        // 验证动态派生：异步块内可以自由嵌套 spawn 新的任务到当前运行时中
        spawner_clone.spawn(async {
            println!("【任务 3】动态启动，等待 0.5 秒...");
            TimerFuture::new(Duration::from_millis(500)).await;
            println!("【任务 3】完成！");
        });
    });

    println!("【运行时】准备就绪，开始执行 Event Loop...");
    // 阻断主线程，转交给 Executor 接管，驱动所有 Future 运转
    // 当所有 Spawner 释放（且任务处理完通道断开）时，Executor 循环才会自然结束
    drop(spawner); // 丢弃主线程的 spawner，以便当所有 spawn 任务执行完毕后通道能正确关闭退出
    executor.run();
    println!("【运行时】所有就绪队列任务已清空，Event Loop 安全退出。");
}
