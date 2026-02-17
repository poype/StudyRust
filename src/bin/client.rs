use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:3030").unwrap();
    stream.write("2026, 马年冲冲冲！Fighting！！！".as_bytes()).unwrap();
    stream.flush().unwrap();
    let mut buffer = [0; 1024];
    let cnt = stream.read(&mut buffer).unwrap();
    let content = buffer[0..cnt].to_vec();
    println!("{:?}", str::from_utf8(&content).unwrap());
}