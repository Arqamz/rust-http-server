use anyhow::Result;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{env, thread};
const ADDRESS: &str = "127.0.0.1:4221";
const OK_HEADER: &str = "HTTP/1.1 200 OK\r\n\r\n";
const CREATED_HEADER: &str = "HTTP/1.1 201 OK\r\n\r\n";
const NOT_FOUND_HEADER: &str = "HTTP/1.1 404 NOTFOUND\r\n\r\n";
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let dir = if args.len() > 2 {
        args.windows(2)
            .find(|window| window[0] == "--directory")
            .unwrap_or_default()[1]
            .to_owned()
    } else {
        String::new()
    };
    let listener = TcpListener::bind(ADDRESS)?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let directory = dir.clone();
                let _handle = thread::spawn(move || {
                    println!("accepted new connection");
                    let mut request = [0_u8; 1024];
                    let bytes = stream.read(&mut request)?;
                    let request_string = String::from_utf8_lossy(&request[..bytes]).into_owned();
                    println!("got request:\n {}", request_string);
                    handle_client(stream, &request_string, &directory)
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
fn handle_client(mut stream: TcpStream, request_string: &str, directory: &str) -> Result<()> {
    let (fisrt_line, rest_lines) = request_string.split_once("\r\n").unwrap();
    let (method, rest) = fisrt_line.split_once(' ').unwrap();
    let response = match method {
        "GET" => match rest.split_once(' ') {
            Some((path, _)) => {
                if path == "/" {
                    OK_HEADER.to_string()
                } else if path.starts_with("/echo/") {
                    let rnd_str = path.strip_prefix("/echo/").unwrap();
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n", rnd_str.len(), rnd_str)
                } else if path == "/user-agent" {
                    let user_agents: Vec<String> = rest_lines
                        .split("\r\n")
                        .into_iter()
                        .filter(|l| l.starts_with("User-Agent: "))
                        .map(|l| l.strip_prefix("User-Agent: ").unwrap().to_string())
                        .collect();
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n", user_agents[0].len(), user_agents[0])
                } else if path.starts_with("/files/") {
                    let fname = path.strip_prefix("/files/").unwrap();
                    match File::open(directory.to_owned() + fname) {
                        Ok(mut file) => {
                            let mut data = vec![];
                            let _ = file.read_to_end(&mut data);
                            let file_data = String::from_utf8_lossy(&data);
                            format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n", file_data.len(), &file_data)
                        }
                        _ => NOT_FOUND_HEADER.to_string(),
                    }
                } else {
                    NOT_FOUND_HEADER.to_string()
                }
            }
            _ => NOT_FOUND_HEADER.to_string(),
        },
        "POST" => {
            let contents = rest_lines.split_once("\r\n\r\n").unwrap().1;
            let fname = rest.split_once(' ').unwrap().0;
            let fname = fname.strip_prefix("/files/").unwrap();
            let mut file = File::create(directory.to_owned() + fname)?;
            file.write_all(contents.as_bytes())?;
            CREATED_HEADER.to_string()
        }
        _ => NOT_FOUND_HEADER.to_string(),
    };
    println!("Sending Response: {}", response);
    stream.write_all(response.as_bytes())?;
    Ok(())
}
