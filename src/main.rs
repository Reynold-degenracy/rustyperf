use tokio::net::{TcpStream, TcpListener, UdpSocket};
use rustyperf::{handle_tcp_test, make_tcp_test, make_udp_test, handle_udp_test, send_arg, receive_arg, swap_udp_port,Config};
use clap::Parser;




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let arg = Config::parse();

    if arg.client {
        // 1. 连接到服务器 
        let mut stream = TcpStream::connect(format!("{}:{}", arg.address, arg.port)).await?;
        send_arg(&mut stream, &arg).await?;
        match (arg.reverse, arg.udp) {
            (true, true) => {
                println!("已连接到服务器 (udp反向模式)");
                let udp_socket = UdpSocket::bind("0.0.0.0:0").await?;
                let server_udp_port = swap_udp_port(&udp_socket, &mut stream).await?;
                udp_socket.connect(format!("{}:{}", arg.address, server_udp_port)).await?;
                handle_udp_test(&udp_socket, arg.time).await?;
            }
            (false, true) => {
                println!("已连接到服务器 (udp正向模式)");
                let udp_socket = UdpSocket::bind("0.0.0.0:0").await?;
                let server_udp_port = swap_udp_port(&udp_socket, &mut stream).await?;
                udp_socket.connect(format!("{}:{}", arg.address, server_udp_port)).await?;
                make_udp_test(&udp_socket, arg.time, arg.bandwidth).await?;
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
                match (client_arg.udp, client_arg.reverse) {
                    (true, true) => {
                        // 反向模式：服务器发送
                        println!("[{}] udp反向模式，服务器开始发送数据", addr);
                        let udp_socket = match UdpSocket::bind("0.0.0.0:0").await {
                            Ok(socket) => socket,
                            Err(e) => {
                                eprintln!("[{}] 绑定UDP端口错误: {}", addr, e);
                                return;
                            }
                        };
                        let client_udp_port = match swap_udp_port(&udp_socket, &mut socket).await {
                            Ok(port) => {
                                println!("udp端口为{}", port);
                                port
                            },
                            Err(e) =>{
                                eprintln!("[{}] 交换UDP端口错误: {}", addr, e);
                                return;
                            }
                        };
                        if let Err(e) = udp_socket.connect(format!("{}:{}", addr.ip(), client_udp_port)).await {
                            eprintln!("[{}] 连接UDP目标错误: {}", addr, e);
                            return;
                        }
                        if let Err(e) = make_udp_test(&udp_socket, client_arg.time, client_arg.bandwidth).await {
                            eprintln!("[{}] UDP测试失败: {}", addr, e);
                            return;
                        }
                    }
                    (true, false) => {
                        // 正向模式：服务器接收
                        println!("[{}] udp正向模式，服务器开始接收数据", addr);
                        let udp_socket = match UdpSocket::bind("0.0.0.0:0").await {
                            Ok(socket) => socket,
                            Err(e) => {
                                eprintln!("[{}] 绑定UDP端口错误: {}", addr, e);
                                return;
                            }
                        };
                        let client_udp_port = match swap_udp_port(&udp_socket, &mut socket).await {
                            Ok(port) => {
                                println!("udp端口为{}", port);
                                port
                            },
                            Err(e) =>{
                                eprintln!("[{}] 交换UDP端口错误: {}", addr, e);
                                return;
                            }
                        };
                        if let Err(e) = udp_socket.connect(format!("{}:{}", addr.ip(), client_udp_port)).await {
                            eprintln!("[{}] 连接UDP目标错误: {}", addr, e);
                            return;
                        }
                        if let Err(e) = handle_udp_test(&udp_socket, client_arg.time).await {
                            eprintln!("[{}] UDP测试失败: {}", addr, e);
                            return;
                        }
                        return;
                    }
                    (false, true) => {
                        // 反向模式：服务器发送
                        println!("[{}] tcp反向模式，服务器开始发送数据", addr);
                        if let Err(e) = make_tcp_test(&mut socket, client_arg.time).await {
                            eprintln!("[{}] 发送错误: {}", addr, e);
                            return;
                        }
                    }    
                    (false, false) => {
                        // 正常模式：服务器接收
                        println!("[{}] tcp正常模式，服务器开始接收数据", addr);
                        if let Err(e) = handle_tcp_test(&mut socket, client_arg.time).await {
                            eprintln!("[{}] 接收错误: {}", addr, e);
                            return;
                        }
                    }
                }
            });
        }
    }


    Ok(())
}
