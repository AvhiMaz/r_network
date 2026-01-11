use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0u8; 4096];

    tokio::spawn(async move {
        loop {
            match stream.read(&mut buf) {
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
    });
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("[error] read failed: {}", e);
            }
        }
    }

    println!("listener: {:?}", listener);

    Ok(())
}
