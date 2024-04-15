use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

fn main() {
    match TcpListener::bind("127.0.0.1:80") {
        Ok(listener) => {
            println!("Server connected");
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let result = handle_client(stream);
                println!("[Client result]: {:?}", result);
            }
        }
        Err(e) => {
            println!("Error: {e}");
        }
    };
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    println!("[Client connected]: {:?}", stream.peer_addr());
    stream.write(b"hello")?;
    stream.read(&mut [0; 128])?;
    Ok(())
}