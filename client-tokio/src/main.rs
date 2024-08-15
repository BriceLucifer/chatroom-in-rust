use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    // 处理环境变量
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <IP> <PORT> <NICKNAME>", args[0]);
        return;
    }
    let ip = &args[1];
    let port = &args[2];
    let name = &args[3];

    let addr = format!("{}:{}", ip, port);

    // 连接到服务器
    let mut socket = TcpStream::connect(&addr).await.expect("Failed to connect");
    socket.set_nodelay(true).expect("Failed to set nodelay");

    // 本地服务地址
    let local_addr = socket.local_addr().unwrap();

    // 创建一个通道用于从服务器读取数据
    let (mut reader, mut writer) = socket.split();
    // 创建一个异步任务，用于从服务器读取数据并将其输出到控制台
    let read_task = async move {
        let mut buf = [0; 1024];
        loop {
            match reader.read(&mut buf).await {
                Ok(n) if n == 0 => break, // EOF
                Ok(n) => {
                    print!("{}🐟{}->:{}", local_addr, name , String::from_utf8_lossy(&buf[..n]));
                }
                Err(e) => {
                    eprintln!("Error reading from server: {:?}", e);
                    break;
                }
            }
        }
    };

    // 创建一个异步任务，用于从控制台读取输入并将其发送到服务器
    let write_task = async move {
        let mut buf = String::new();
        let mut stdin = BufReader::new(tokio::io::stdin());
        loop {
            buf.clear();
            match stdin.read_line(&mut buf).await {
                Ok(n) if n == 0 => break, // EOF
                Ok(_) => {
                    if let Err(e) = writer.write_all(buf.as_bytes()).await {
                        eprintln!("Error writing to server: {:?}", e);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from stdin: {:?}", e);
                    break;
                }
            }
        }
    };

    // 等待两个异步任务完成
    tokio::join!(read_task, write_task);
}
