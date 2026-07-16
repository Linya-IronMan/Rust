---
tags:
  - Rust
  - 异步原理
  - MOC
---

# 🧭 Rust 异步底层原理

本 MOC（知识地图）是 Rust 异步编程底层运作机制的导航中心。它将手写微型异步调度器（Mini-Executor）的核心演进过程拆解为 5 大系统性步骤，并提供针对源码的深度对应解析。

---

## 🗺️ 异步原理演进地图

点击进入对应的原子细节卡片，按步骤进行递进式研读：

1. **第一步**：[[并发模型与 Future 本质]] —— 剖析并发模型演进物理本质，理解“惰性拉模型”状态机。
2. **第二步**：[[TimerFuture 与 Waker 唤醒机制]] —— 揭开 Future 在 Pending 时保存 Waker、在就绪时唤醒的核心谜底。
3. **第三步**：[[Task 载体与 ArcWake 重排队]] —— 探讨如何通过 `ArcWake` 特征将唤醒通知重定向回就绪通道。
4. **第四步**：[[Executor 事件循环与任务调度]] —— 解析事件循环 `while let Ok` 以及 `poll` 对状态机的物理驱动。
5. **第五步**：[[Mini-Executor 物理图景大串联]] —— 结合 Mermaid 时序图与全景故事，完整复盘 2 秒睡眠任务的 CPU 流转画卷。

---

## 🛠️ 本地配套实战源码
- 核心手写原理项目：[mini_executor/src/main.rs](file:///Users/linya/Code/Self/Rust/async_learning/mini_executor/src/main.rs)
- 工业级 Tokio 实战：[tokio_chat/src/main.rs](file:///Users/linya/Code/Self/Rust/async_learning/tokio_chat/src/main.rs)
- 导读运行手册：[README.md](file:///Users/linya/Code/Self/Rust/async_learning/README.md)

---

## 🔗 相关联的笔记 (Obsidian)
- 异步主路线规划：[[异步编程]]
- 复杂异常处理机制：[[Rust 复杂与异步错误恢复实践]]
