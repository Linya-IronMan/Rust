use std::sync::Arc;
use std::thread;

/// @struct DatabaseConnection
/// @brief 模拟一个高昂的数据库物理连接句柄。
#[derive(Debug)]
struct DatabaseConnection {
    /// 模拟数据库物理连接的唯一 URL
    url: String,
}

/// @fn main
/// @brief 练习主入口函数。
/// @details 您的任务是：使用 Arc 智能指针包装 DatabaseConnection，
///          使得 3 个子线程能够同时并发访问这一个数据库连接而不会触发借用检查报错。
///          请找到 TODO 留空处，补全代码以通过断言。
fn main() {
    // 1. 初始化高昂的共享连接
    let connection = DatabaseConnection {
        url: String::from("postgres://localhost:5432/production"),
    };

    // =========================================================================
    // 🚨 TODO 任务一：用 Arc 包装 connection，使得所有权可以被安全共享计数
    // 提示：使用 Arc::new(...)
    // =========================================================================
    let shared_conn = Arc::new(connection); // 👈 这一行已经帮您写好作为示例

    let mut thread_handles = vec![];

    for i in 0..3 {
        // =====================================================================
        // 🚨 TODO 任务二：在派生线程前，克隆 shared_conn 的所有权指针
        // 提示：使用 Arc::clone(&...) 或者 shared_conn.clone()
        // =====================================================================
        // 🚨 请解除下方注释并补全代码，替换掉 unimplemented!()
        // let conn_clone: Arc<DatabaseConnection> = ......;
        let conn_clone: Arc<DatabaseConnection> = shared_conn.clone();

        let handle = thread::spawn(move || {
            // 在子线程中打印当前连接，验证其有效性
            println!("【子线程 {}】正在通过连接读取数据: {}", i, conn_clone.url);

            // 验证：通过计算指针地址，断言它们是否指向堆内存上的同一块空间
            // 提示：shared_conn 是在主线程，我们无法在这里直接调用它（因为 move 了），
            // 但我们可以确保 3 个线程内打印出来的 url 地址完全一致。
            assert_eq!(conn_clone.url, "postgres://localhost:5432/production");
        });
        thread_handles.push(handle);
    }

    // 等待所有线程执行完毕
    for handle in thread_handles {
        handle.join().unwrap();
    }

    // =========================================================================
    // 🚨 TODO 任务三：在主线程，验证 Arc 的强计数器是否依然为 1（因为子线程都死光了）
    // 提示：调用 Arc::strong_count(&...) 获取强引用计数
    // =========================================================================
    let final_count = Arc::strong_count(&shared_conn); // 👈 这一行已经帮您写好，请确保代码通过断言

    println!(
        "【主线程】子线程执行完毕，当前 Arc 强计数器为: {}",
        final_count
    );
    assert_eq!(final_count, 1);
    println!("🎉 恭喜！Arc 练习完美通过！");
}

/* =============================================================================
📖 参考答案说明（请在思考后再查看）：
1. 任务一：使用 `let shared_conn = Arc::new(connection);` 将所有权移入 Arc 堆盒中。
2. 任务二：在 spawn 的 `move ||` 闭包前，执行 `let conn_clone = Arc::clone(&shared_conn);`
   （或者 `shared_conn.clone()`）。这样 move 进去的就是被 clone 后的快捷方式指针，
   而原 `shared_conn` 指针依然保留在主线程中以备后续使用。
3. 任务三：子线程全部 join 结束后，它们的 Arc 指针生命期结束并被自动 Drop（强计数减3），
   主线程只剩唯一的 `shared_conn`，故 `Arc::strong_count(&shared_conn)` 必然返回 1。
============================================================================= */
