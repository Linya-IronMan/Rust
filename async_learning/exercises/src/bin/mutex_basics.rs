use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

/// @fn main
/// @brief Mutex 互斥锁与有毒锁（Poisoning）处理实战练习。
/// @details 本练习分为两个任务：
///          任务一：通过 Arc<Mutex<i32>> 在 10 个线程中安全地并发累加计数器。
///          任务二：学习当持锁线程 Panic 后，如何在主线程挽救被毒化的锁并提取数据。
fn main() {
    // =========================================================================
    // 1. 任务一：并发累加计数器
    // =========================================================================
    println!("--- 任务一：并发累加计数器开始 ---");
    let counter = Arc::new(Mutex::new(0));
    let mut thread_handles = vec![];

    for _ in 0..10 {
        let counter_clone = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                // =============================================================
                // 🚨 TODO 任务一：获取 Mutex 锁的所有权并安全递增内部数值。
                // 提示：调用 lock() 抢锁，并通过 unwrap() 处理可能的 PoisonError。
                //       修改数据后，确保锁的守护者（MutexGuard）能及时 Drop 释放锁。
                // =============================================================
                // 🚨 请补全下方代码，替换掉 unimplemented!()
                let mut data_guard: MutexGuard<'_, i32> = counter_clone.lock().unwrap();
                *data_guard += 1;
            }
        });
        thread_handles.push(handle);
    }

    // 等待所有累加线程结束
    for handle in thread_handles {
        handle.join().unwrap();
    }

    // 验证结果是否为 10 * 1000 = 10000
    let final_value = *counter.lock().unwrap();
    println!("【任务一】累加结束，最终数值为: {}", final_value);
    assert_eq!(final_value, 10000);
    println!("🎉 任务一通过！\n");

    // =========================================================================
    // 2. 任务二：毒化锁 (Poisoning) 数据自救
    // =========================================================================
    println!("--- 任务二：毒化锁数据自救开始 ---");
    // 初始化一个存有核心机密数据的锁
    let db_lock = Arc::new(Mutex::new(String::from("Sensitive Data")));

    let db_lock_clone = db_lock.clone();
    let handle = thread::spawn(move || {
        // 子线程拿到锁
        let mut guard = db_lock_clone.lock().unwrap();
        // 修改写了一半的数据
        guard.clear();
        guard.push_str("Dirty data written by a crashing thread");

        // 🚨 线程突然发生 panic 崩溃！此时由于锁被持有，锁会被打上 Poisoned 标记
        panic!("线程意外崩溃！锁已被污染！");
    });

    // 捕获线程崩溃，防止主线程随之崩溃退出
    let _ = handle.join();

    // =========================================================================
    // 🚨 TODO 任务二：此时主线程去 lock() 会报错 Err(PoisonError)。
    // 请补全下方代码，利用 match 匹配锁的返回结果，并在 Err 中使用 `.into_inner()`
    // 强制强救出那个有毒的 MutexGuard，读取出内部的脏数据，并断言其内容符合预期。
    // =========================================================================
    // 🚨 请补全下方匹配逻辑
    let retrieved_data = match db_lock.lock() {
        Ok(guard) => guard.to_string(),
        Err(poison_error) => {
            // 提示：poison_error 里面包裹着原 Guard 的存根。
            // 请使用 .into_inner() 拯救出内部的 MutexGuard
            // 🚨 请解除下方注释并填空
            let salvaged_guard = poison_error.into_inner();
            salvaged_guard.to_string()
            // unimplemented!("请在此处捕捉 PoisonError 并自救还原内部数据")
        }
    };

    println!("【任务二】自救恢复出来的数据内容为: \"{}\"", retrieved_data);
    assert_eq!(retrieved_data, "Dirty data written by a crashing thread");
    println!("🎉 任务二自救成功，锁毒化练习完美通过！");
}

/* =============================================================================
📖 参考答案说明（请在思考后再查看）：
1. 任务一：
   ```rust
   let mut data_guard = counter_clone.lock().unwrap();
   *data_guard += 1;
   ```
   每一轮循环中，`data_guard` 作为一个局部变量（MutexGuard），在当前大括号循环体 `{}` 结束时
   会自动被 Drop（释放锁），从而允许其他线程在下一轮抢锁。
2. 任务二：
   ```rust
   Err(poison_error) => {
       let salvaged_guard = poison_error.into_inner();
       salvaged_guard.to_string()
   }
   ```
   当持锁线程 Panic 时，Rust 强迫我们在 Err 分支处理 PoisonError。
   通过 `into_inner()`，我们依然能够获得底层数据的可变引用（MutexGuard），从而挽救脏数据进行容灾恢复。
============================================================================= */
