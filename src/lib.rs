use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;

pub async fn handle_connection(mut socket: TcpStream) {
    let mut buffer = [0; 1024]; // 创建一个缓冲区
    // let mut total_bytes_received: usize = 0;
    loop {
        // 4. 从接口读取数据
        // `read` 返回 0 表示连接已关闭
        let _n = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("读取数据失败: {}", e);
                return;
            }
        };

        // (此处可以添加测量逻辑，如计算接收速率)
        // total_bytes_received += n;
        // let start_time = time::Instant::now();
        // 5. 将数据写回（用于往返测试）或仅作丢弃
        // if let Err(e) = socket.write_all(&buffer[0..n]).await {
        //     eprintln!("写入数据失败: {}", e);
        //     return;
        // }
        
    }
}

