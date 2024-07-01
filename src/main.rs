use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let get = b"GET / HTTP/1.1\r\n";
    let response = if buffer.starts_with(get) {
        "HTTP/1.1 200 OK\r\n\r\n"
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n"
    };
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {

    println!("Logs from your program will appear here!");

    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                _stream.write(b"HTTP/1.1 200 OK\r\n\r\n").expect("200 \n");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
