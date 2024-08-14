use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <IP> <PORT>", args[0]);
        return;
    }
    let ip = &args[1];
    let port = &args[2];

    let addr = format!("{}:{}", ip, port);

    // 连接到服务器
    let mut socket = TcpStream::connect(&addr).await.expect("Failed to connect");
    socket.set_nodelay(true).expect("Failed to set nodelay");

    // 创建一个通道用于从服务器读取数据
    let (mut reader, mut writer) = socket.split();

    // 创建一个异步任务，用于从服务器读取数据并将其输出到控制台
    let read_task = async move {
        let mut buf = [0; 1024];
        loop {
            match reader.read(&mut buf).await {
                Ok(n) if n == 0 => break, // EOF
                Ok(n) => {
                    print!("{}🐟->:{}",addr,String::from_utf8_lossy(&buf[..n]));
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