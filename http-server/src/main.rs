use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;


fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in  listener.incoming() {
        let stream = stream.unwrap();
        hanle_connection(stream);
    }
}

fn hanle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request:\n{}", String::from_utf8_lossy(&buffer[..]));

    let contents = fs::read_to_string("src/index.html").unwrap();
    let response = format!(
        "HTTP1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
