use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // 1. 创建一个 TCP 监听器并绑定到地址
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("服务器正在监听 127.0.0.1:8080");

    loop {
        // 2. 接受新的连接
        let (socket, _) = listener.accept().await?;
        
        // 3. 为每个连接生成一个新任务，防止阻塞主循环
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buffer = [0; 1024]; // 创建一个缓冲区
    loop {
        // 4. 从套接字读取数据
        // `read` 返回 0 表示连接已关闭
        let n = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("读取数据失败: {}", e);
                return;
            }
        };

        // (此处可以添加测量逻辑，如计算接收速率)
        
        // 5. 将数据写回（用于往返测试）或仅作丢弃
        // if let Err(e) = socket.write_all(&buffer[0..n]).await {
        //     eprintln!("写入数据失败: {}", e);
        //     return;
        // }
        
    }
}
