use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
};

fn main() {
    println!("Server starting.");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Started listening on port 7878.");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
