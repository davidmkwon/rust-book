use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        thread_pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    // read message into buffer
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // check if request is for /
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "src/success.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/error.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    // write response back
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}