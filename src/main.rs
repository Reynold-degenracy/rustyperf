use tokio::net::{TcpStream, TcpListener};
use rustyperf::{handle_connection, make_connection, send_mode, receive_mode};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(
    name = "RustyPerf",
    author = "Shinonome Rei",
    version = "v1.0.2",
    about = "Application short description."
)]
struct Config {

    #[arg(short='p', long="port", default_value_t = 2077)]
    port: u16,

    #[arg(short='t', long="time", default_value_t = 10)]
    time: u64,

    #[arg(short='c', long="client", default_value_t = false)]
    client: bool,

    #[arg(short='s', long="server", default_value_t = true)]
    server: bool,

    #[arg(short='a', long="address", default_value_t = String::from("127.0.0.1"))]
    address: String,

    #[arg(short='r', long="reverse", default_value_t = false)]
    reverse: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let arg = Config::parse();

    if arg.client {
        // 1. 连接到服务器
        let mut stream = TcpStream::connect(format!("{}:{}", arg.address, arg.port)).await?;
        if arg.reverse {
            // 反向模式：客户端接收，服务器发送
            println!("已连接到服务器 (反向模式)");
            send_mode(&mut stream, true, arg.time).await?;
            handle_connection(stream).await?;
        } else {
            // 正常模式：客户端发送，服务器接收
            println!("已连接到服务器 (正常模式)");
            send_mode(&mut stream, false, arg.time).await?;
            make_connection(stream, arg.time).await?;
        }
    }

    else if arg.server {
        // 1. 创建一个 TCP 监听器并绑定到地址
        let listener = TcpListener::bind(format!("0.0.0.0:{}", arg.port)).await?;
        println!("服务器正在监听{}端口", arg.port);
        loop {
            // 2. 接受新的连接
            let (mut socket, addr) = listener.accept().await?; 
            println!("新连接来自: {}", addr);
            // 3. 为每个连接生成一个新任务，防止阻塞主循环
            tokio::spawn(async move {
                match receive_mode(&mut socket).await {
                    Ok((is_reverse, time)) => {
                        if is_reverse {
                            // 反向模式：服务器发送
                            println!("[{}] 反向模式，服务器开始发送数据", addr);
                            if let Err(e) = make_connection(socket, time).await {
                                eprintln!("[{}] 发送错误: {}", addr, e);
                            }
                        } else {
                            // 正常模式：服务器接收
                            println!("[{}] 正常模式，服务器开始接收数据", addr);
                            if let Err(e) = handle_connection(socket).await {
                                eprintln!("[{}] 接收错误: {}", addr, e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[{}] 读取模式信息失败: {}", addr, e);
                    }
                }
            });
        }
    }


    Ok(())
}
