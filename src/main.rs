use tokio::net::{TcpStream, TcpListener, UdpSocket};
use rustyperf::{handle_tcp_test, make_tcp_test, make_udp_test, handle_udp_test, send_arg, receive_arg, Config};
use clap::Parser;




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let arg = Config::parse();

    if arg.client {
        // 1. 连接到服务器
        let mut udp_socket = UdpSocket::bind(format!("0.0.0.0:{}", arg.port)).await?;
        udp_socket.connect(format!("{}:{}", arg.address, arg.port)).await?; 
        let mut stream = TcpStream::connect(format!("{}:{}", arg.address, arg.port)).await?;
        send_arg(&mut stream, &arg).await?;
        match (arg.reverse, arg.udp) {
            (true, true) => {
                println!("已连接到服务器 (udp反向模式)");
                handle_udp_test(udp_socket, arg.time, arg.bandwidth).await?;
            }
            (false, true) => {
                println!("已连接到服务器 (udp反向模式)");
                make_udp_test(udp_socket, arg.time, arg.bandwidth).await?;
            }
            (true, false) => {
                // 反向模式：客户端接收，服务器发送
                println!("已连接到服务器 (tcp反向模式)");
                handle_tcp_test(&mut stream, arg.time).await?;
            }
            (false, false) => {
                // 正常模式：客户端发送，服务器接收
                println!("已连接到服务器 (tcp正常模式)");
                make_tcp_test(&mut stream, arg.time).await?;
            }
        }
    }
    


    else if arg.server {
        // 1. 创建一个监听器并绑定到地址
        let listener = TcpListener::bind(format!("0.0.0.0:{}", arg.port)).await?;
        println!("服务器正在监听{}端口", arg.port);
        loop {
            // 2. 接受新的连接
            let (mut socket, addr) = listener.accept().await?; 
            println!("新连接来自: {}", addr);
            // 3. 为每个连接生成一个新任务，防止阻塞主循环
            tokio::spawn(async move {
                let client_arg = match receive_arg(&mut socket).await {
                    Ok(arg) => arg,
                    Err(e) => {
                        eprintln!("[{}] 接收参数错误: {}", addr, e);
                        return;
                    }
                };
                match (client_arg.udp, client_arg.reverse, client_arg.time) {
                    (true, true, time) => {
                        // 反向模式：服务器发送
                        
                    }
                    (true, false, time) => {
                        // 正向模式：服务器接收
                        
                    }
                    (false, true, time) => {
                        // 反向模式：服务器发送
                        println!("[{}] 反向模式，服务器开始发送数据", addr);
                        if let Err(e) = make_tcp_test(&mut socket, time).await {
                            eprintln!("[{}] 发送错误: {}", addr, e);
                        }
                    }    
                    (false, false, time) => {
                        // 正常模式：服务器接收
                        println!("[{}] 正常模式，服务器开始接收数据", addr);
                        if let Err(e) = handle_tcp_test(&mut socket, time).await {
                            eprintln!("[{}] 接收错误: {}", addr, e);
                        }
                    }
                }
            });
        }
    }


    Ok(())
}
