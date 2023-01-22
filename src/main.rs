use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
    thread,
    time::Duration,
};

fn main() {
    println!("Server starting.");
    let port = 7878;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

    println!("Started listening on {port}.");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if http_request.len() > 0 {
        // The first line contains request type
        let request_line = http_request.first().unwrap();

        let (status_line, filename) = match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        println!("HTTP request is empty.");
    }
}
