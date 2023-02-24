// This implementation is based on the "The Rust Programming Language Book" chapter 20: "Final
// Project: Building a Multithreaded Web Server".

use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
    thread,
    time::Duration,
};
use hello::ThreadPool;

// TODO:
// - Add README
// - Add more documentation to ThreadPool and its public methods.
// - Add tests of the library’s functionality.
// - Change calls to unwrap to more robust error handling.
// - Use ThreadPool to perform some task other than serving web requests.
// - Find a thread pool crate on crates.io and implement a similar web server using the crate instead. Then compare its API and robustness to the thread pool we implemented.

fn main() {
    println!("Server starting.");
    let port = 7878;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();
    let thread_pool = ThreadPool::new(4);

    println!("Started listening on {port}.");
    // Note: the server will process only three requests and shutdown gracefully afterwards
    // However, it won't shutdown until it can join all the threads. Therefore, we first need
    // to send another request and since the sender has already been dropped, the threads
    // will receiver an error and finish their execution.
    for stream in listener.incoming().take(3) {
        let stream = stream.unwrap();

        println!("Connection established!");
        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
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
