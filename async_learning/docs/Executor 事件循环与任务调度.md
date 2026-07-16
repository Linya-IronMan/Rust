---
tags:
  - Rust
  - 异步原理
  - Executor
  - 事件循环
---

# Executor 事件循环与任务调度

我们已经手写了 Future 和 Waker 桥梁。现在，我们需要一个引擎来推动这一切运转。这个引擎就是 **`Executor`**。
`Executor` 的唯一职责是维持一个高效率的**事件循环（Event Loop）**，轮询并消耗就绪的任务队列。

---

## 1. Executor 的物理数据结构

在我们的源码 [mini_executor/src/main.rs](file:///Users/linya/Code/Self/Rust/async_learning/mini_executor/src/main.rs#L129-L215) 中，`Executor` 和 `Spawner` 的定义非常精简：

```rust
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

struct Spawner {
    task_sender: Sender<Arc<Task>>,
}
```
- **单就绪队列模型**：`ready_queue` 使用了 Crossbeam 提供的多生产者多消费者通道（mpmc）。
- **`Spawner` 职责**：负责提交全新（尚未首次 poll）的任务。它将 Future 包装为 `Task` 并发送给通道。
- **`Executor` 职责**：接收通道输出，对其进行 poll。

---

## 2. 阻塞式 Event Loop 机制

很多开发者误以为异步事件循环就是一个 `while true` 的死循环。其实，真正的 Executor 绝对不允许让 CPU 盲等空转。

我们来看 `Executor::run` 的主循环实现：

```rust
pub fn run(&self) {
    // 1. ready_queue.recv() 会阻塞当前线程
    while let Ok(task) = self.ready_queue.recv() {
        let mut future_slot = task.future.lock().unwrap();
        if let Some(mut future) = future_slot.take() {
            // 2. 构造 Context 并包装 Waker
            let waker = waker_ref(&task);
            let mut context = Context::from_waker(&waker);
            
            // 3. 对 Future 发起 poll 轮询
            if future.as_mut().poll(&mut context).is_pending() {
                // 4. Pending 状态保留
                *future_slot = Some(future);
            }
        }
    }
}
```

### 事件循环物理流转图解：
1. **CPU 友好的阻塞机制**：
   - `self.ready_queue.recv()` 是一个阻塞式通道调用。
   - 当就绪通道中没有任何任务时，操作系统会将当前的 Executor 线程挂起，转去让其他进程运行，**CPU 占用率归 0**。
   - 只有当 `Waker` 重新投递任务进入通道后，Executor 线程才会被操作系统唤醒，继续从 `recv` 的下一行执行。
2. **Context 重建与 Poll**：
   - 提取出 `Task` 后，利用 `waker_ref(&task)` 将 Task 本身转换为标准 `Waker`。
   - 通过 `Context::from_waker(&waker)` 组装当前执行上下文。
   - 调用 `future.as_mut().poll(&mut context)`。
3. **状态捕获与内存释放**：
   - **捕获 Pending**：如果 `poll` 返回 `Poll::Pending`，说明还没完成。我们将 `future` 重新放回 `future_slot` 里保存，等待它的 Waker 唤醒。
   - **捕获 Ready**：如果返回 `Poll::Ready(())`，由于我们在 `future_slot.take()` 时已经将 Future 移出了 Slot，我们只需**不再将它塞回**。此时，该 Future 状态机所占用的全部栈/堆内存会立刻由于超出作用域而被销毁释放（Drop）。

---

## 🔗 下一步导航
- 下一步：[[Mini-Executor 物理图景大串联]]
- 上一步：[[Task 载体与 ArcWake 重排队]]
- 返回 MOC：[[🧭 Rust 异步底层原理]]
