use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use http::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for streamed_request in listener.incoming() {
        let _streamed_request = streamed_request.unwrap();
        println!("Connection established");
        pool.execute(|| {
            connection_handler(_streamed_request);
        })
    }
    println!("Shutting down.");
}

fn connection_handler(mut stream: TcpStream){
    let _buff_reader = BufReader::new(&stream);
    let request_line = _buff_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 Ok","response.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND","404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response_data = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{contents}");
    stream.write_all(response_data.as_bytes()).unwrap();

}