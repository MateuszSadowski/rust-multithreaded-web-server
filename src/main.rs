use std::net::TcpListener;

fn main() {
    println!("Server starting.");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Started listening on port 7878.");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
