// Uncomment this block to pass the first stage
use std::{io::Write, net::{TcpListener, TcpStream}};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let tcp_listener: TcpListener = match TcpListener::bind("127.0.0.1:6379") {
        Ok(tcp_listener) => tcp_listener,
        Err(e) => panic!("BIND ERROR: {:#?}", e.kind()),
    };

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let pong: &str = "+PONG\r\n";

    if let Err(e) = stream.write_all(pong.as_bytes()) {
        println!("{}",e);
    } else if let Err(e) = stream.flush() {
        println!("{}",e);
    }
}
