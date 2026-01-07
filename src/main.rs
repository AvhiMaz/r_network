use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0u8; 4096];

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
                eprintln!("[ERROR] read failed: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("listener: {:?}", listener);

    Ok(())
}
