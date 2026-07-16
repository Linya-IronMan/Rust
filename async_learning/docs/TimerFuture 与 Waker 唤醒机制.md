---
tags:
  - Rust
  - 异步原理
  - TimerFuture
  - Waker
---

# TimerFuture 与 Waker 唤醒机制

在第一步中我们了解到：Future 在被 `poll` 时，如果数据未就绪，它必须返回 `Poll::Pending`。
但是，**执行器（Executor）不能没完没了地不停 poll 一个 Pending 的 Future**（那会退化成死循环盲等，导致 CPU 占用 100%）。执行器必须把这个 Future 挂起，等待数据好了以后，由 Future 主动发出“我好了，快来重新 poll 我”的通知。

这套“挂起 ➔ 唤醒”的核心纽带，就是 **`Waker`**。本章以自定义的 `TimerFuture` 为蓝本，剖析这套唤醒机制的物理流转。

---

## 1. TimerFuture 的物理数据结构

我们来看我们在项目 [mini_executor/src/main.rs](file:///Users/linya/Code/Self/Rust/async_learning/mini_executor/src/main.rs#L14-L87) 中定义的底层模型：

```rust
struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}
```
### 为什么需要 `Arc<Mutex<...>>` 共享状态？
- 因为 `TimerFuture` 会派生一个后台工作线程（模拟非阻塞网络或定时中断）。%% 为什么会派生出后台工作线程, 这只是一个普通的结构体, 因为有 waker 属性么? %%
- 状态在两个不同的物理线程间共享：
  1. **执行器线程**：轮询 `TimerFuture`，读取 `completed`，或者在 `waker` 中存入当前执行上下文的唤醒信号。
  2. **后台工作线程**：等待 2 秒时间到后，将 `completed` 标为 `true`，并取出 `waker` 执行 `wake()`。

---

## 2. 轮询函数 `poll` 的内部奥秘

下面是 `Future` 特征实现中最核心的代码：

```rust
impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            // 状态转移：直接 Ready 退出
            Poll::Ready(())
        } else {
            // ⚠️ 关键操作：克隆当前上下文的 Waker！
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
   }
}
```

### 深入解析每一行物理意图：
- `cx: &mut Context<'_>`：代表当前轮询的上下文，它包裹着当前任务的 `Waker` 唤醒句柄。
- `cx.waker().clone()`：我们将这个 `Waker` 进行深度拷贝。这是因为 `cx` 生命期极短，只在当前 `poll` 调用栈内有效，我们必须把它保存到能跨越线程共享的 `SharedState` 中。
- **多次 Poll 的覆盖问题**：
  如果一个 Future 历经多次 `poll`（例如它被转移到了另一个线程的执行器上运行），每一次 `poll` 传进来的 `Waker` 可能是不同的。因此，每次调用 `poll` 时，只要未就绪，我们都必须用最新的 `cx.waker()` **无条件覆盖（Overwrite）**之前保存的旧 `waker`，确保唤醒信号最终送达正确的执行器。

---

## 3. 后台线程的唤醒逻辑

在定时器的后台睡眠线程中：

```rust
thread::spawn(move || {
    thread::sleep(duration); // 模拟底层非阻塞 I/O 硬件等待
    let mut shared_state = thread_shared_state.lock().unwrap();
    shared_state.completed = true;
    
    // 唤醒：拉响铃铛
    if let Some(waker) = shared_state.waker.take() {
        waker.wake();
    }
});
```
- 当 `sleep(2)` 结束，后台线程修改 `completed = true`。
- 然后，它提取出之前由执行器存入的 `waker`。
- 调用 `waker.wake()`！
* **物理结果**：这个 `wake()` 就像是拉响了铃铛。它将在底层触发一系列关联反应，最终把包裹该定时器的任务指针重新发入就绪管道。在下一步中，我们将拆解这个“重排队”的详细过程。

---

## 🔗 下一步导航
- 下一步：[[Task 载体与 ArcWake 重排队]]
- 上一步：[[并发模型与 Future 本质]]
- 返回 MOC：[[🧭 Rust 异步底层原理]]
