<details open>
<summary>🇨🇳 中文</summary>

# 关于本项目
Rustyperf 是一个rust项目，旨在复现部分iperf3的功能。
# 环境依赖
使用tokio作为异步运行时，
使用clap解析命令行参数，
使用cross+docker进行交叉编译。
# 使用
在不传入任何参数时，默认以服务器模式运行。
```bash
#等同于 cargo run -- -s
cargo run -- 
```
在传入 -c 参数时，以客户端模式运行
```bash
# cargo run -- -c [-a --address] [-t --time] [-r --reverse]
cargo run -- -c -a 127.0.0.1 -t 10 #向本机服务器进行10秒钟正向测试
cargo run -- -c -a 192.168.1.1 -t 60 -r #向192.168.1.1进行60秒反向测试
```
</details>


<details>
<summary>🇺🇸 English</summary>

# About
Rustyperf is a Rust project designed to reproduce some of the functionality of iperf3.
# Environment
Use tokio as an asynchronous runtime.
Use clap to parse command-line arguments.
Cross-compilation is performed using cross+docker.
# Usage
If no parameters are passed, the program will run in server mode by default.
```bash
#Equivalent to cargo run -- -s
cargo run -- 
```
When the -c parameter is passed, the program runs in client mode.
```bash
# cargo run -- -c [-a --address] [-t --time] [-r --reverse]
cargo run -- -c -a 127.0.0.1 -t 10 #Perform a 10-second positive test on the local server.
cargo run -- -c -a 192.168.1.1 -t 60 -r #Perform a 60-second reverse test on 192.168.1.1.
```

</details>
