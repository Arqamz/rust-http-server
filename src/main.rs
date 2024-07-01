use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    str,
};
static RESPONSE_OK: &str = "200 OK";
static RESPONSE_NOT_FOUND: &str = "404 Not Found";
fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_req(stream),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
fn handle_req(mut stream: TcpStream) {
    let lines = parse_request(&stream);
    let req_details: Vec<_> = lines[0].split(" ").collect();
    let headers = &lines[1..];
    let res: String;
    if req_details[1].starts_with("/echo") {
        let input: Vec<_> = req_details[1]
            .split("/")
            .filter(|s| !s.is_empty())
            .collect();
        println!("input: {:?}", input);
        res = echo_response(input[1]);
    } else if req_details[1].starts_with("/user-agent") {
        let found = headers
            .iter()
            .find(|header| header.starts_with("User-Agent:"));
        match found {
            Some(header) => {
                let user_agent: Vec<_> = header.split(" ").collect();
                res = echo_response(user_agent[1]);
            }
            None => res = format!("HTTP/1.1 {}\r\n\r\n", RESPONSE_NOT_FOUND),
        }
    } else {
        match req_details[1] {
            "/" => res = format!("HTTP/1.1 {}\r\n\r\n", RESPONSE_OK),
            _ => res = format!("HTTP/1.1 {}\r\n\r\n", RESPONSE_NOT_FOUND),
        }
    }
    stream.write_all(res.as_bytes()).unwrap()
}
fn parse_request(mut stream: &TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(&mut stream);
    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    http_req
}
fn echo_response(input: &str) -> String {
    format!(
        "HTTP/1.1 {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        RESPONSE_OK,
        input.len(),
        input
    )
}

