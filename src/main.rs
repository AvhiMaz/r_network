use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

async fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0u8; 4096];
    loop {
        match stream.read(&mut buf).await {
            Ok(0) => {
                println!("[CLOSE] connection closed");
                break;
            }
            Ok(n) => {
                println!("[READ] {} bytes", n);
            }
            Err(e) => {
                eprintln!("[error] read failed: {}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("[ACCEPT]: {:?}", addr);

        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}
