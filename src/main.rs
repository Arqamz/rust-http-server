use anyhow::Error;
use std::{
    env, fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str, thread,
};
#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    version: String,
    host: Option<String>,
    user_agent: Option<String>,
}
impl Default for HttpRequest {
    fn default() -> Self {
        HttpRequest {
            user_agent: Default::default(),
            host: Default::default(),
            method: Default::default(),
            version: Default::default(),
            path: Default::default(),
        }
    }
}
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                thread::spawn(|| handle_connect(_stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
fn handle_connect(mut stream: TcpStream) {
    println!("accepted new connection");
    let mut buffer = [0; 1024];
    let ok_resp = "HTTP/1.1 200 OK\r\n\r\n".to_string();
    let not_found_resp = "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string();
    match stream.read(&mut buffer) {
        Ok(_n) => {
            println!("Req receided");
            let req = str::from_utf8(&buffer).unwrap();
            let http_request = pars_req(&req).unwrap();
            let mut resp = not_found_resp.clone();
            println!("path: {:?}", http_request.user_agent);
            if http_request.path == "/" {
                resp = ok_resp.clone();
            } else if http_request.path.starts_with("/echo") {
                let body = http_request.path.replace("/echo/", "");
                resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
            } else if http_request.path.starts_with("/user-agent") {
                let body = http_request.user_agent.unwrap();
                if http_request.method == "GET" {
                    resp = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n",
                                    body.len(),
                                    body
                                )
                }
            } else if http_request.path.starts_with("/files") {
                let file_name = http_request.path.replace("/files/", "");
                let env_args: Vec<String> = env::args().collect();
                let mut dir = env_args[2].clone();
                dir.push_str(&file_name);
                let file = fs::read(dir);
                match file {
                    Ok(fc) => {
                        resp = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n", fc.len(), String::from_utf8(fc).expect("file content"));
                    }
                    Err(..) => resp = not_found_resp.clone(),
                }
            }
            match stream.write(resp.as_bytes()) {
                Ok(_) => println!("Ok"),
                Err(e) => println!("err: {}", e),
            }
        }
        Err(e) => println!("Fail connect: {}", e),
    }
}
fn pars_req(req: &str) -> Result<HttpRequest, Error> {
    let content: Vec<&str> = req.lines().collect();
    let mut method_header = content[0].split_whitespace();
    let host = content[1].replace("Host: ", "");
    let user_agent = content[2].replace("User-Agent: ", "");
    let http_request = HttpRequest {
        method: String::from(method_header.next().unwrap()),
        path: String::from(method_header.next().unwrap()),
        version: String::from(method_header.next().unwrap()),
        host: Some(host),
        user_agent: Some(user_agent),
        ..Default::default()
    };
    Ok(http_request)
}
