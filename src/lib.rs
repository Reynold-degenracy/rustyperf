use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::time::{Instant, Duration};

pub async fn handle_connection(mut socket: TcpStream) -> io::Result<u64> {
    let mut buffer = [0; 1024]; // 创建一个缓冲区
    // let mut total_bytes_received: usize = 0;
    loop {
        // 从接口读取数据
        // `read` 返回 0 表示连接已关闭
        let _n = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => return Ok(0),
            Ok(n) => n,
            Err(e) => {
                eprintln!("读取数据失败: {}", e);
                return Ok(0);
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

pub async fn make_connection(mut stream: TcpStream, time: u64) -> io::Result<u64> {
    let data = [0u8; 1024 * 64]; // 64KiB 的数据块
    let start_time = Instant::now();
    let test_duration: Duration = Duration::from_secs(time as u64);
    let mut total_bytes_sent: u64 = 0;
    let mut last_report_time = Instant::now();
    let mut bytes_since_last_report: u64 = 0;
    println!("开始{}秒测试...", time);
    while Instant::now() - start_time < test_duration {
        stream.write_all(&data).await?;
        let bytes_sent = data.len() as u64;
        total_bytes_sent += bytes_sent;
        bytes_since_last_report += bytes_sent;
        if last_report_time.elapsed() >= Duration::from_secs(1) {
            let throughput_mbps = (bytes_since_last_report * 8) as f64 / 1_000_000.0;
            println!("当前速度 {:.2} Mbps", throughput_mbps);

            // 重置计数
            bytes_since_last_report = 0;
            last_report_time = Instant::now();
        }
    }

    // 3. 计算并打印结果
    let elapsed = start_time.elapsed();
    let throughput_mbps = (total_bytes_sent as f64 * 8.0) / (elapsed.as_secs_f64() * 1_000_000.0);

    println!("测试完成！");
    println!("持续时间: {:.2?}", elapsed);
    println!("总共发送: {:.2} MB", total_bytes_sent as f64 / 1_000_000.0);
    println!("吞吐率: {:.2} Mbps", throughput_mbps);
    Ok(0)
}