use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
};

fn main() {
    println!("Server starting.");
    let port = 7878;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

    println!("Started listening on {port}.");
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

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
