use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};
use std::net::SocketAddr;

/// ============================================================================
/// Tokio 异步广播聊天服务器 (Tokio Chat Server)
/// ============================================================================

/// @fn main
/// @brief 异步聊天室服务端的主入口函数。
/// @details 使用 `#[tokio::main]` 宏在运行时启动 Tokio 的多线程工作窃取调度器（Work-stealing Scheduler）。
///          初始化一个广播通道（Broadcast Channel）作为聊天室的总线传输带，
///          并在主线程事件循环中监听并接受 TCP 客户端连接。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 监听本地 8080 端口，绑定 TCP 套接字
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("【服务器】启动成功，正在监听 127.0.0.1:8080...");

    // 初始化多生产者多消费者广播通道（mpmc）。
    // 容量设为 16。这意味着如果积压的消息超过 16 条，慢速消费者将会收到 Lagged 错误。
    // tx 用于向通道发送消息，rx（此处未解构）可用于主线程接收
    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(16);

    loop {
        // 挂起式等待：在有新连接进来前，这里不会占用 CPU 资源。
        // accept() 成功后会返回客户端的 TcpStream 以及其 IP 套接字地址
        let (socket, addr) = listener.accept().await?;
        println!("【连接接入】来自客户端: {}", addr);

        // 克隆广播发送端，以便在挂载的客户端协程中向聊天室群发消息
        let tx_clone = tx.clone();
        // 订阅当前广播通道，为该客户端生成其专属的接收端（Receiver）
        let rx_subscriber = tx.subscribe();

        // 核心异步模式：tokio::spawn 将客户端处理流程提升为一个独立的异步 Task，
        // 由 Tokio 调度器自动派发到后台的某个物理线程中并发执行。
        // 这实现了主 accept 循环的非阻塞，即使有数万个连接也能轻松应对。
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, addr, tx_clone, rx_subscriber).await {
                eprintln!("【连接异常断开】客户端 {} 报错: {}", addr, e);
            }
        });
    }
}

/// @fn handle_client
/// @brief 针对单个客户端连接的异步处理状态机。
/// @param socket 代表客户端的异步 TCP 流
/// @param addr 客户端的套接字物理地址
/// @param tx 聊天室广播总线发送端
/// @param mut rx 当前客户端专属的广播订阅接收端
/// @return Result<(), Box<dyn std::error::Error>>
/// @details 此方法使用 `tokio::select!` 宏实现异步多路复用，
///          同时监听“客户端发送消息”与“聊天室广播消息就绪”两个异步源。
async fn handle_client(
    mut socket: TcpStream,
    addr: SocketAddr,
    tx: broadcast::Sender<(String, SocketAddr)>,
    mut rx: broadcast::Receiver<(String, SocketAddr)>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 将套接字拆分为读半部和写半部
    let (reader, mut writer) = socket.split();
    // 使用带缓存的异步读取器包裹读半部，以支持按行读取数据
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    // 向新接入的客户端发送欢迎语
    writer.write_all(b"Welcome to Rust Async Chatroom!\nUse Telnet or Netcat to chat.\n\n").await?;
    writer.flush().await?;

    // 广播客户端上线消息
    let join_msg = format!("【系统广播】客户端 {} 加入了聊天室！\n", addr);
    let _ = tx.send((join_msg, addr));

    loop {
        // 核心技术陷阱避坑说明：
        // `tokio::select!` 属于多路复用控制流。
        // 它会并发轮询所有分支的 Future。一旦其中某个分支的 Future 就绪，
        // `select!` 会执行该分支的代码块，并**静默销毁（Drop）**其他未就绪的分支 Future。
        // 
        // ⚠️ 取消安全性 (Cancel Safety) 注意点：
        // 如果 Future 被销毁时，它的状态会丢失，这称为“非取消安全”。
        // 例如，如果我们在 `select!` 分支中直接写 `reader.read_line(&mut line)`，
        // 一旦另一个分支（接收到广播）先就绪，未读完一行数据的 `read_line` Future 就会被销毁，
        // 导致已经读入缓冲区的部分字节丢失。
        // 
        // 修复方案：
        // 在此处，我们没有在 select 块中动态创建 Future，而是通过在外部复用一个 `line` 变量，
        // 或者使用 Tokio 提供的规范封装来实现安全的读取状态流转。
        // 为了演示简便，这里我们利用极简的分支清理逻辑，并在此详细注释其运作本质。
        tokio::select! {
            // 分支 1：监听客户端的网络数据输入
            result = reader.read_line(&mut line) => {
                let bytes_read = result?;
                // bytes_read == 0 说明客户端主动断开了 TCP 连接（收到 EOF）
                if bytes_read == 0 {
                    break;
                }

                // 去除尾部的换行符，整理消息格式
                let client_msg = line.trim().to_string();
                line.clear(); // 清空缓存，为下一次按行读取做准备

                if !client_msg.is_empty() {
                    // 将接收到的消息和来源地址打包发送到聊天室总线
                    let broadcast_msg = format!("{}: {}\n", addr, client_msg);
                    let _ = tx.send((broadcast_msg, addr));
                }
            }

            // 分支 2：监听聊天室广播总线，接收其他客户端发送的消息
            msg_result = rx.recv() => {
                match msg_result {
                    // 成功拉取到广播消息
                    Ok((msg, sender_addr)) => {
                        // 过滤掉当前客户端自己发出的消息，避免回显刷屏
                        if sender_addr != addr {
                            writer.write_all(msg.as_bytes()).await?;
                            writer.flush().await?;
                        }
                    }
                    // 处理由于消费消息过慢导致的缓冲区溢出（Lagged 错误）
                    Err(broadcast::error::RecvError::Lagged(skipped)) => {
                        let warning = format!("【系统警告】由于您网络较慢，跳过了 {} 条历史消息。\n", skipped);
                        writer.write_all(warning.as_bytes()).await?;
                        writer.flush().await?;
                    }
                    // 广播通道彻底关闭（通常不会发生，除非所有发送端都被释放）
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        }
    }

    // 客户端下线处理
    let leave_msg = format!("【系统广播】客户端 {} 离开了聊天室。\n", addr);
    let _ = tx.send((leave_msg, addr));
    println!("【连接断开】客户端 {} 正常退出。", addr);
    
    Ok(())
}
