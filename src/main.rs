use std::{
    fs::File,
    thread,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use webserver_rust::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("ここはもう使われている。。");
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.expect("読み取り失敗");

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "src/sample.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/404.html")
    };
    
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    
    let response = format!("{}{}", status_line, contents);
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
