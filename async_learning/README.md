# Rust 异步编程实战学习工作区

本目录是一个为 Rust 异步编程设计的本地实战代码工作区。包含了从“底层微型调度器”到“上层高并发 Tokio 实战”的完整实践。

---

## 📂 项目结构

- **`mini_executor/`**：手写微型异步运行时（零依赖，仅引入核心辅助库）。
  - 实现自定义的 `TimerFuture`，模拟睡眠非阻塞唤醒状态。
  - 实现包裹 Future 且支持 `ArcWake` 规范的 `Task` 任务载体。
  - 实现 `Executor` 主事件循环（Event Loop）与异步任务派发器 `Spawner`。
- **`tokio_chat/`**：基于 Tokio 异步运行时的工业级高并发广播聊天室。
  - 运用 `tokio::spawn` 动态挂载并发任务。
  - 运用 `tokio::sync::broadcast` 通道总线在多个客户端连接间传递消息。
  - 运用 `tokio::select!` 多路复用分流网络读取与广播分发，并附带**取消安全性（Cancel Safety）**的代码规避指导。

---

## 🚀 快速启动运行

在当前 `async_learning/` 目录下（或项目根目录下使用 `-p` 指定包名），在终端中运行以下命令：

### 1. 运行底层微型调度器
```bash
cargo run -p mini_executor
```
* **预期输出**：
  你将看到两个异步任务交替等待不同时长，并动态派生出“任务 3”，最后所有任务处理完毕，调度器安全退出的完整逻辑。

### 2. 运行高并发 Tokio 聊天服务器
```bash
cargo run -p tokio_chat
```
* **如何测试聊天室**：
  服务器启动后会监听本机 `8080` 端口。你可以在两个不同的终端中，使用 `telnet` 或 `nc`（netcat）连接它，模拟多用户聊天：
  ```bash
  # 终端 1
  nc 127.0.0.1 8080
  
  # 终端 2
  nc 127.0.0.1 8080
  ```
  在任意终端中输入文字并敲击回车，即可在另一端实时收到广播消息！

---

## 🔍 代码研读重点指引

### 1. `mini_executor` 底层原理解析
- **为什么需要 `Pin`**：
  在 `mini_executor/src/main.rs` 的 `Task::future` 字段中，我们使用了 `Pin<Box<dyn Future...>>`。请思考如果 Future 内部在 `.await` 前后保存了对局部变量的引用（自引用结构体），一旦被 `move`，其引用的物理内存地址失效会带来什么后果？`Pin` 是如何将其物理地址钉死在堆上的？
- **`Waker` 的真实传递机制**：
  看看 `TimerFuture` 里的后台工作线程，它在延时结束后，是如何拿到之前在 `poll` 阶段由 Context 存入的 `waker` 并触发 `wake()` 的？追踪 `Task::wake_by_ref` 是如何把当前任务指针塞回 Crossbeam 通道的。

### 2. `tokio_chat` 生产级设计技巧
- **理解 `tokio::select!` 与“取消安全性”**：
  在 `tokio_chat/src/main.rs` 中，我们使用 `select!` 来并发监听客户端发送的消息以及来自群聊的广播。思考：当广播消息先被读取并写入 Socket 时，另一个监听客户端 Socket 读的 `read_line` Future 是怎么被 `select!` 自动销毁的？为什么如果状态不保存在 Future 外部，就会发生丢数据的情况？
- **多播通道的应用**：
  思考为什么我们在聊天室广播里使用的是 `broadcast`（多生产者多消费者）而不是常规的 `mpsc`（多生产者单消费者）。

---

## 🔗 相关联的笔记 (Obsidian)
- 异步主路线规划：[[异步编程]]
- 复杂异常处理机制：[[Rust 复杂与异步错误恢复实践]]
