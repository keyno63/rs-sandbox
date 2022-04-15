extern crate time;

use std::io::Write;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read_exact(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let message    = "Hello, World!";
    let time       = time::now_utc();
    let format     = "%a, %d %b %Y %T GMT";
    let response   = format!("HTTP/1.1 200 OK\r\n\
                              Date: {}\r\n\
                              Content-Type: text/html; charset=utf-8\r\n\
                              Content-Length: {}\r\n\
                              \r\n\
                              {}",
                             time::strftime(format, &time).unwrap(),
                             message.len(),
                             message);
    let _          = stream.write(response.as_bytes());
}
