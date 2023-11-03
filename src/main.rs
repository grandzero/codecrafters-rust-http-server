use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
fn handle_client(mut stream : TcpStream) {
    println!("Incoming request from: {}", stream.peer_addr().unwrap());
    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
}
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

  
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
