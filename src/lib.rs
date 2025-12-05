use tokio::net::{TcpStream, UdpSocket};
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use tokio::time::{Instant, Duration};
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize, PartialEq)]
#[command(
    name = "RustyPerf",
    author = "Shinonome",
    version = "v1.0.2",
    about = "Application short description."
)]
pub struct Config {

    #[arg(short='p', long="port", default_value_t = 2077)]
    pub port: u16,

    #[arg(short='t', long="time", default_value_t = 10)]
    pub time: u64,

    #[arg(short='c', long="client", default_value_t = false)]
    pub client: bool,

    #[arg(short='s', long="server", default_value_t = true)]
    pub server: bool,

    #[arg(short='a', long="address", default_value_t = String::from("127.0.0.1"))]
    pub address: String,

    #[arg(short='r', long="reverse", default_value_t = false)]
    pub reverse: bool,

    #[arg(short='u', long="udp", default_value_t = false)]
    pub udp: bool,

    #[arg(short='b', long="bandwidth", default_value_t = 1_u64<<20)]
    pub bandwidth: u64, //in bps
}

pub async fn send_arg(stream: &mut TcpStream, config: &Config) -> io::Result<()> {
    // serialize 会自动处理 String 的长度前缀和数据的二进制转换
    let data = bincode::serialize(config)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let len = data.len() as u64;
    stream.write_u64(len).await?;
    stream.write_all(&data).await?;
    stream.flush().await?;
    Ok(()) 
}

pub async fn receive_arg(stream: &mut TcpStream) -> io::Result<Config> {
    let len = stream.read_u64().await?;
    let mut data = vec![0u8; len as usize];
    stream.read_exact(&mut data).await?;
    // deserialize 会一直读取直到还原出一个完整的 Config 结构体
    let config: Config = bincode::deserialize(&data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(config)
}

pub async fn handle_tcp_test(socket:&mut TcpStream, time: u64) -> io::Result<u64> {
    let mut buffer = [0; 1024*64]; //64KiB 
    let mut total_bytes_received: u64 = 0;
    let mut interval_bytes: u64 = 0;
    
    // 用于计算速率的时间点
    let start_time = Instant::now();
    let mut last_report_time = Instant::now();
    let report_interval = Duration::from_secs(1); // 每秒显示一次速率
    
    loop {
        // 从接口读取数据
        let n = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => {
                // 显示最终统计
                let total_time = start_time.elapsed();
                if total_time.as_secs() > 0 {
                    let avg_bps = total_bytes_received as f64 * 8.0 / time as f64;
                    println!("总接收: {:.2} MB, 平均速率: {}", 
                    total_bytes_received as f64 / 1_048_576.0,
                    format_speed(avg_bps));
                }
                println!("接收结束，连接关闭");
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
            // 计算当前速率 (比特/秒)
            let speed_bps = interval_bytes as f64 * 8.0 / elapsed.as_secs_f64();
            
            // 格式化显示
            println!(
                "接收速率: {} | 总接收: {:.2} MB",
                format_speed(speed_bps),
                total_bytes_received as f64 / 1_048_576.0
            );
            
            // 重置间隔计数器
            interval_bytes = 0;
            last_report_time = Instant::now();
        }
    }
}

pub async fn make_tcp_test(stream:&mut TcpStream, time: u64) -> io::Result<u64> {
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

    //结束发送，关闭连接
    stream.flush().await?;
    stream.shutdown().await?;

    // 3. 计算并打印结果
    let elapsed = start_time.elapsed();
    let throughput_bps = (total_bytes_sent as f64 * 8.0) / (elapsed.as_secs_f64());

    println!("测试完成！");
    println!("持续时间: {:.2?}", elapsed);
    println!("总共发送: {:.2} MB", total_bytes_sent as f64 / 1_048_576.0);
    println!("平均速率: {} ", format_speed(throughput_bps));
    Ok(0)
}

pub async fn make_udp_test(mut udp_socket: UdpSocket, time: u64, speed: u64) -> io::Result<u64> {
    Ok(0)
}

pub async fn handle_udp_test(mut udp_socket: UdpSocket, time: u64, speed: u64) -> io::Result<u64> {
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