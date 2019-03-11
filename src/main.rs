mod ftp;
use std::net::TcpListener;

const LISTEN_ADDRESS: &str = "127.0.0.1";
const LISTEN_PORT: i32 = 9000;

fn main() {
    let listen_addr = format!("{}:{}", LISTEN_ADDRESS, LISTEN_PORT);
    let listener = TcpListener::bind(listen_addr).unwrap();

    println!("Listening on 127.0.0.1:9000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => ftp::Ftp::new(stream).begin(),
            Err(e) => println!("{}", e),
        };
    }
}
