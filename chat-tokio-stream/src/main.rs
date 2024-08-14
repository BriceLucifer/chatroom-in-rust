use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    println!("a simple chat server in tokio");
    let envs :Vec<String>= std::env::args().collect();

    let listener = TcpListener::bind(&envs[1]).await.unwrap();
    // await wait the future get ready
    let (tx, _rx) = broadcast::channel(10);

    loop {
        // accept connect
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0{
                            break;
                        }

                        tx.send((line.clone(),addr)).unwrap();
                        line.clear()
                    }

                    result = rx.recv() => {
                        let (message,other_addr) = result.unwrap();

                        if addr != other_addr{
                            writer.write_all(message.as_bytes()).await.unwrap()
                        }
                    }
                }
            }
        });
    }
}

// step tcp echo server
// 1.tcplisten

// step turn echo server to chat server
