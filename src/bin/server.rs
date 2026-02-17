use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:3030").unwrap();
    println!("Listening on {}", tcp_listener.local_addr().unwrap());

    for stream in tcp_listener.incoming() {
        let mut stream: TcpStream = stream.unwrap();
        println!("Connection established!");

        let mut buffer = [0; 1024];
        let cnt: usize = stream.read(&mut buffer).unwrap();
        let content = buffer[0..cnt].to_vec();

        println!("{:?}", str::from_utf8(&content).unwrap());
        stream.write(&content).unwrap();
    }
}