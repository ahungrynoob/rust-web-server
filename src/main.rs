use std::net::{TcpListener, TcpStream};

fn handle_client(_stream: TcpStream) {
    println!("收到了一些内容")
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
