use tokio::net::TcpListener;
use rustyperf::handle_connection;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(
    name = "My Application",
    author = "Author's name",
    version = "v1.0.0",
    about = "Application short description."
)]
struct Config {

    #[arg(short='p', long="port", default_value_t = 8080)]
    port: u16,

    #[arg(short='t', long="time", default_value_t = 10)]
    time: u64,

    #[arg(short='c', long="client", default_value_t = false)]
    client: bool,

    #[arg(short='s', long="server", default_value_t = true)]
    server: bool,

    #[arg(short='a', long="address", default_value_t = String::from(""))]
    address: String,

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let arg = Config::parse();

    if arg.client {

        use tokio::net::TcpStream;
        use tokio::io::AsyncWriteExt;
        use tokio::time::{self, Duration};

        // 1. 连接到服务器
        let mut stream = TcpStream::connect(format!("{}:{}", arg.address, arg.port)).await?;
        println!("已连接到服务器");

        let data = [0u8; 1024 * 64]; // 64KB 的数据块
        let test_duration: Duration = Duration::from_secs(arg.time as u64);
        let start_time: time::Instant = time::Instant::now();
        let mut total_bytes_sent: usize = 0;

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

    }

    else if arg.server {

        // 1. 创建一个 TCP 监听器并绑定到地址
        let listener = TcpListener::bind(format!("127.0.0.1:{}", arg.port)).await?;
        println!("服务器正在监听 127.0.0.1:{}", arg.port);
        
        loop {
            // 2. 接受新的连接
            let (socket, _) = listener.accept().await?;
            
            // 3. 为每个连接生成一个新任务，防止阻塞主循环
            tokio::spawn(async move {
                handle_connection(socket).await;
            });
        }
    }


    Ok(())
}
