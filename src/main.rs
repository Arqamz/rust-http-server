use std::{
    env, fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    str,
};
use itertools::Itertools;
static RESPONSE_OK: &str = "200 OK";
static RESPONSE_NOT_FOUND: &str = "404 Not Found";
fn main() {
    let mut dir: &String = &Default::default();
    let args: Vec<_> = env::args().collect();
    let found = args.iter().find_position(|arg| *arg == "--directory");
    match found {
        Some((index, _)) => dir = &args[index + 1],
        None => (),
    }
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_req(stream),
            Ok(stream) => {
                let new_dir = dir.clone();
                std::thread::spawn(|| handle_req(stream, new_dir));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
fn handle_req(mut stream: TcpStream) {
fn handle_req(mut stream: TcpStream, dir: String) {
    let lines = parse_request(&stream);
    let req_details: Vec<_> = lines[0].split(" ").collect();
    let headers = &lines[1..];

            .filter(|s| !s.is_empty())
            .collect();
        res = echo_response(input[1]);
        res = echo_response(input[1], "text/plain");
    } else if req_details[1].starts_with("/user-agent") {
        let found = headers
            .iter()
            .find(|header| header.starts_with("User-Agent:"));
        match found {
            Some(header) => {
                let user_agent: Vec<_> = header.split(" ").collect();
                res = echo_response(user_agent[1]);
                res = echo_response(user_agent[1], "text/plain");
            }
            None => res = format!("HTTP/1.1 {}\r\n\r\n", RESPONSE_NOT_FOUND),
        }
    } else if req_details[1].starts_with("/files") {
        let input: Vec<_> = req_details[1]
            .split("/")
            .filter(|s| !s.is_empty())
            .collect();
        let file = fs::read_to_string(format!("{}{}", dir, input[1]));
        match file {
            Ok(contents) => res = echo_response(contents.as_str(), "application/octet-stream"),
            Err(e) => {
                res = format!("HTTP/1.1 {}\r\n\r\n", RESPONSE_NOT_FOUND);
                println!("error: {}", e)
            }
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
fn echo_response(input: &str, content_type: &str) -> String {
    format!(
        "HTTP/1.1 {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        RESPONSE_OK,
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        content_type,
        input.len(),
        input
    )
}