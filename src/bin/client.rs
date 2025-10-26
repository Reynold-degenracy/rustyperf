use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::time::{self, Duration};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. 连接到服务器
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("已连接到服务器");

    let data = [0u8; 1024 * 64]; // 64KB 的数据块
    let test_duration = Duration::from_secs(10);
    let start_time = time::Instant::now();
    let mut total_bytes_sent = 0;

    // 2. 在指定时间内持续发送数据
    while time::Instant::now() - start_time < test_duration {
        stream.write_all(&data).await?;
        total_bytes_sent += data.len();
    }

    // 3. 计算并打印结果
    let elapsed = start_time.elapsed();
    let throughput_mbps = (total_bytes_sent as f64 * 8.0) / (elapsed.as_secs_f64() * 1_000_000.0);

    println!("测试完成！");
    println!("持续时间: {:.2?}", elapsed);
    println!("总共发送: {:.2} MB", total_bytes_sent as f64 / 1_000_000.0);
    println!("吞吐率: {:.2} Mbps", throughput_mbps);

    Ok(())
}
