use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::time::{Instant, Duration};

pub async fn handle_connection(mut socket: TcpStream) -> io::Result<u64> {
    let mut buffer = [0; 1024];
    let mut total_bytes_received: u64 = 0;
    let mut interval_bytes: u64 = 0;
    
    // 用于计算速率的时间点
    let mut last_report_time = Instant::now();
    let report_interval = Duration::from_secs(1); // 每秒显示一次速率
    
    loop {
        // 从接口读取数据
        let n = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => {
                println!("接收结束，连接关闭");
                // 显示最终统计
                let total_time = last_report_time.elapsed();
                if total_time.as_secs() > 0 {
                    let avg_speed = total_bytes_received as f64 / total_time.as_secs_f64();
                    println!("总接收: {} 字节, 平均速率: {:.2} MB/s", 
                        total_bytes_received, 
                        avg_speed / 1_048_576.0);
                }
                return Ok(total_bytes_received);
            },
            Ok(n) => n,
            Err(e) => {
                eprintln!("读取数据失败: {}", e);
                return Ok(total_bytes_received);
            }
        };
        
        // 累计字节数
        total_bytes_received += n as u64;
        interval_bytes += n as u64;
        
        // 检查是否到达报告间隔
        let elapsed = last_report_time.elapsed();
        if elapsed >= report_interval {
            // 计算当前速率 (字节/秒)
            let speed_bps = interval_bytes as f64 * 8.0 / elapsed.as_secs_f64();
            
            // 格式化显示
            let speed_str = format_speed(speed_bps);
            println!(
                "接收速率: {} | 总接收: {:.2} MB",
                speed_str,
                total_bytes_received as f64 / 1_048_576.0
            );
            
            // 重置间隔计数器
            interval_bytes = 0;
            last_report_time = Instant::now();
        }
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
            let throughput_bps = (bytes_since_last_report) as f64 * 8.0;
            println!("发送速率： {}", format_speed(throughput_bps));

            // 重置计数
            bytes_since_last_report = 0;
            last_report_time = Instant::now();
        }
    }

    // 3. 计算并打印结果
    let elapsed = start_time.elapsed();
    let throughput_bps = (total_bytes_sent as f64 * 8.0) / (elapsed.as_secs_f64());

    println!("测试完成！");
    println!("持续时间: {:.2?}", elapsed);
    println!("总共发送: {:.2} MB", total_bytes_sent as f64 / 1_000_000.0);
    println!("吞吐率: {} ", format_speed(throughput_bps));
    Ok(0)
}

fn format_speed(bits_per_sec: f64) -> String {
    if bits_per_sec >= 1e9 as f64 {
        format!("{:.2} Gbps", bits_per_sec / 1e9 as f64)
    } else if bits_per_sec >= 1e6 as f64 {
        format!("{:.2} Mbps", bits_per_sec / 1e6 as f64)
    } else if bits_per_sec >= 1e3 as f64 {
        format!("{:.2} Kbps", bits_per_sec / 1e3 as f64)
    } else {
        format!("{:.2} bps", bits_per_sec)
    }
}