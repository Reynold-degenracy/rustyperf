use tokio::net::{TcpStream, TcpListener};
use rustyperf::{handle_connection, make_connection};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(
    name = "RustyPerf",
    author = "Shinonome Rei",
    version = "v1.0.2",
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
        let stream = TcpStream::connect(format!("{}:{}", arg.address, arg.port)).await?;
        if arg.reverse {
            // let reverse_stream = TcpListener::bind("").await?;
            
        }
        println!("已连接到服务器");
        make_connection(stream, arg.time).await.unwrap();
    }

    else if arg.server {
        // 1. 创建一个 TCP 监听器并绑定到地址
        let listener = TcpListener::bind(format!("{}:{}", arg.address, arg.port)).await?;
        println!("服务器正在监听 {}:{}", arg.address, arg.port);
        loop {
            // 2. 接受新的连接
            let (socket, _) = listener.accept().await?; 
            // 3. 为每个连接生成一个新任务，防止阻塞主循环
            tokio::spawn(async move {
                handle_connection(socket).await.unwrap();
            });
        }
    }


    Ok(())
}
