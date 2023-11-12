use std::env::args;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
mod parse_file;
mod parse_headers;
use parse_file::read_file_and_return_content;
use parse_file::FileErrors;
use parse_headers::{parse_headers, Headers};
// use std::env::args;
// use std::process;
use std::thread;
trait Length {
    fn get_length(&self) -> usize;
}
impl Length for FileErrors {
    fn get_length(&self) -> usize {
        format!("{}", self).len()
    }
}

trait FindHeader {
    fn find_header(&self, header: &str) -> Option<String>;
}

impl FindHeader for Vec<Headers> {
    fn find_header(&self, header: &str) -> Option<String> {
        for h in self {
            match h {
                Headers::UserAgent(val) => {
                    if header == "User-Agent" {
                        return Some(val.to_string());
                    }
                }
                Headers::ContentType(val) => {
                    if header == "Content-Type" {
                        return Some(val.to_string());
                    }
                }
                Headers::ContentLength(val) => {
                    if header == "Content-Length" {
                        return Some(val.to_string());
                    }
                }
                Headers::Host(val) => {
                    if header == "Host" {
                        return Some(val.to_string());
                    }
                }
            }
        }
        None
    }
}

fn handle_client(mut stream: TcpStream, directory: String) {
    //println!("Incoming request from: {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request_details_as_str = String::from_utf8_lossy(&buffer[..]);
    let request_lines = request_details_as_str.split("\r\n");
    let query_param;
    let response: String;
    let headers = parse_headers(request_details_as_str.as_ref());
    // println!("Headers:{:?}", headers);
    match request_lines.clone().next() {
        Some(request_line) => {
            let request_original_details: Vec<&str> = request_line.split(" ").collect();
            //  println!("Request details:{:?}", request_original_details);

            response = if request_original_details[1].contains("/echo/") {
                query_param = request_original_details[1]
                    .split("/echo/")
                    .collect::<Vec<&str>>()[1];
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type:text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    query_param.len(),
                    query_param
                )
            } else if request_original_details[1].contains("/user-agent") {
                match headers.find_header("User-Agent") {
                    Some(str_result) => {
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Type:text/plain\r\nContent-Length: {}\r\n\r\n{}",
                            str_result.len(),
                            str_result
                        )
                    }
                    None => {
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Type:text/plain\r\nContent-Length: {}\r\n\r\n{}",
                            "Not Found".len(),
                            "Not Found"
                        )
                    }
                }
            } else if request_original_details[1] == "/" {
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type:text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    "Hello".len(),
                    "Hello"
                )
            } else if request_original_details[1].contains("/files/") {
                query_param = request_original_details[1]
                    .split("/files/")
                    .collect::<Vec<&str>>()[1];
                println!("Filename : {}", query_param);
                match read_file_and_return_content(query_param, &directory) {
                    Ok(content) => {
                        let head = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type:application/octet-stream\r\nContent-Disposition: attachment; filename=\"{}\"\r\nContent-Length: {}\r\n\r\n",
                            query_param,
                            content.len(),
                        );
                        stream.write_all(head.as_bytes()).unwrap();
                        stream.write_all(&content).unwrap();
                        return;
                    }
                    Err(e) => {
                        format!(
                            "HTTP/1.1 404 OK\r\nContent-Type:text/plain\r\nContent-Length: {}\r\n\r\n{}",
                            e.get_length(),
                            e
                        )
                    }
                }
            } else {
                "HTTP/1.1 404 NOT FOUND\r\nContent-Type:text/plain\r\n\r\nContent-Length: 9\r\n\r\nNot Found".to_string()
            };
            // println!("Response:{}", response);
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
    println!("Server listening at : http://127.0.0.1:4221");
    let mut directory: String = "./".to_string();
    let mut directory_flag = args();
    println!("{:?}", directory_flag);
    if directory_flag.len() >= 2 {
        // exit
        println!("{:?}", directory_flag);
        println!("Directory flag found");
        directory = directory_flag.nth(2).unwrap();
    }

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let borrowed_directory = directory.clone();
                thread::spawn(|| handle_client(_stream, borrowed_directory));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
