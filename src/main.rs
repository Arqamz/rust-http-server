use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};
fn handle_path(first_line: &String) -> String {
    let path_list = first_line.split(" ").collect::<Vec<&str>>();
    println!("path_list {:?}", path_list);
    if path_list[1] == "/" {
        return "HTTP/1.1 200 OK\r\n\r\n".to_owned();
    }
    if !path_list[1].starts_with("/echo") {
        return "HTTP/1.1 404 Not Found\r\n\r\n".to_owned();
    }
    let len = path_list.len();
    let echo_sub_path = path_list[1].split("/").collect::<Vec<&str>>()[len - 1];
    println!("sub path {:?}", echo_sub_path);
    let path_string_len = echo_sub_path.len();
    return format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        path_string_len, echo_sub_path
    );
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let first_line = &_http_request[0];
    let path = first_line.split(" ").collect::<Vec<&str>>()[1];
    let response = if path == "/" {
        "HTTP/1.1 200 OK\r\n\r\n"
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n"
    };
    let response = handle_path(&_http_request[0]);
    if let Ok(_) = stream.write_all(response.as_bytes()) {
        println!("accepted new connection");
    } else {
        eprintln!("Error occured!");
    }
}
fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}