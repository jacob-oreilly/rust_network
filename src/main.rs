use std::net::{TcpListener, TcpStream};

fn main() {
    match TcpListener::bind("127.0.0.1:80") {
        Ok(listener) => {
            println!("Server connected");
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                handle_client(stream);
            }
        }
        Err(e) => {
            println!("Error: {e}");
        }
    };
}

fn handle_client(stream: TcpStream) {
    println!("[Client connected]: {:?}", stream.peer_addr());
}