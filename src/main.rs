use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for streamed_request in listener.incoming() {
        let _streamed_request = streamed_request.unwrap();
        println!("Connection established");
        connection_handler(_streamed_request);
    }
}

fn connection_handler(mut stream: TcpStream){
    let _buff_reader = BufReader::new(&stream);
    let _http_request: Vec<_> = _buff_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty()).collect();
    // println!("Request: {http_request:#?}");

    let status_line  = "HTTP/1.1 200 Ok";
    let contents = fs::read_to_string("response.html").unwrap();
    let length = contents.len();

    let response_data = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{contents}");
    stream.write_all(response_data.as_bytes()).unwrap()
}