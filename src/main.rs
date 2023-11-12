use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
fn handle_client(mut stream: TcpStream) {
    //println!("Incoming request from: {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request_details_as_str = String::from_utf8_lossy(&buffer[..]);
    let request_lines = request_details_as_str.split("\r\n");
    let query_param;
    let response: String;
    match request_lines.clone().next() {
        Some(request_line) => {
            let request_original_details: Vec<&str> = request_line.split(" ").collect();
            println!("Request details:{:?}", request_original_details);

            response = if request_original_details[1].contains("/echo/") {
                query_param = request_original_details[1]
                    .split("/echo/")
                    .collect::<Vec<&str>>()[1];
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type:text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    query_param.len(),
                    query_param
                )

                // if request_original_details[1].starts_with("/echo/") {
                //     let query_param = request_original_details[1]
                //         .split("/echo/")
                //         .collect::<Vec<&str>>()[1];
                //     println!("Query param:{}", query_param);
                //     //response = format!("HTTP/1.1 200 OK\r\nContent-Type:text/plain\r\nContent-Length: {}\r\n\r\n{}", query_param.len(), query_param).as_str();
                // } else if request_original_details[1] == "/" {
                //     response =
                //         "HTTP/1.1 200 OK\r\nContent-Type:text/plain\r\nContent-Length: 0\r\n";
                // } else {
                //     response = "HTTP/1.1 404 NOT FOUND\r\nContent-Type:text/plain\r\n\r\nContent-Length: 9\r\n\r\nNot Found";
                // }
            } else if request_original_details[1] == "/" {
                "HTTP/1.1 200 OK\r\nContent-Type:text/plain\r\nContent-Length: 0\r\n".to_string()
            } else {
                "HTTP/1.1 404 NOT FOUND\r\nContent-Type:text/plain\r\n\r\nContent-Length: 9\r\n\r\nNot Found".to_string()
            };
            println!("Response:{}", response);
            stream.write(response.as_bytes()).unwrap();
        }
        None => {
            println!("No request line found");
        }
    }
    // println!("Request headers:{:?}", request_lines);
    // request_lines.for_each(|line| println!("{}", line));
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
