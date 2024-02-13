use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::thread::JoinHandle;

const PONG: &[u8; 7] = b"+PONG\r\n";
const BIND_ADDR: &str = "127.0.0.1:6379";

fn main() {
    println!("Logs from your program will appear here!");

    let tcp_listener: TcpListener = match TcpListener::bind(BIND_ADDR) {
        Ok(tcp_listener) => tcp_listener,
        Err(e) => panic!("BIND ERROR: {:#?}", e.kind()),
    };

    let mut open_handles: Vec<JoinHandle<()>> = Vec::new();

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted new connection");
                let handle: JoinHandle<()> = thread::spawn(move || handle_connection(stream));
                open_handles.push(handle);
            }

            Err(e) => {
                println!("Unsuccessful connection to TcpStream: {e}\r\nMoving on to next.");
            }
        }
    }

    for handle in open_handles {
        handle.join().expect("Couldn't join");
    }
}

fn handle_connection(mut stream: TcpStream) {
    // *1\r\n$4\r\nping\r\n
    let mut buffer: [u8; 512] = [0; 512];
    let mut count: usize = 0;
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            
            Ok(bytes_read) => {
                count += bytes_read;
                if count != count % 14 {
                    if let Err(e) = stream.write_all(PONG) {
                        eprintln!("Error sending response: {e}");
                    } else if let Err(e) = stream.flush() {
                        eprintln!("Error flushing tcpstream: {e}");
                    }
                }
                
                count %= 14;
            },

            Err(e) if e.kind() == ErrorKind::Interrupted => {},
    
            Err(e) => {
                eprintln!("TcpStream read error: {e}");
                break;
            },
        }
    }
}
