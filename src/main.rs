use std::{
    env, fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

static RESPONSE_OK: &str = "200 OK";
static RESPONSE_NOT_FOUND: &str = "404 Not Found";

fn main() {
    let mut dir = String::new();

    // Parse command line arguments to find --directory flag
    let args: Vec<_> = env::args().collect();
    if let Some(index) = args.iter().position(|arg| arg == "--directory") {
        dir = args[index + 1].clone(); // Assume the directory path is provided next to --directory
    }

    // Bind to localhost:4221
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Server listening on port 4221...");

    // Listen for incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let new_dir = dir.clone(); // Clone dir for each new thread
                std::thread::spawn(|| handle_req(stream, new_dir));
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_req(mut stream: TcpStream, dir: String) {
    // Parse the HTTP request
    let lines = parse_request(&mut stream);
    let req_details: Vec<_> = lines[0].split_whitespace().collect();

    // Initialize response string
    let mut res = String::new();

    // Process request based on the requested path
    if req_details.len() < 2 {
        res = format!("HTTP/1.1 {}\r\n\r\n", RESPONSE_NOT_FOUND);
    } else {
        match req_details[1] {
            "/" => res = format!("HTTP/1.1 {}\r\n\r\n", RESPONSE_OK),
            "/files" => {
                let input: Vec<_> = req_details[1]
                    .split("/")
                    .filter(|s| !s.is_empty())
                    .collect();

                if input.len() > 1 {
                    let file_path = format!("{}{}", dir, input[1]);
                    let file_content = fs::read_to_string(file_path);

                    match file_content {
                        Ok(content) => {
                            res = echo_response(&content, "text/plain");
                        }
                        Err(_) => {
                            res = format!("HTTP/1.1 {}\r\n\r\n", RESPONSE_NOT_FOUND);
                        }
                    }
                } else {
                    res = format!("HTTP/1.1 {}\r\n\r\n", RESPONSE_NOT_FOUND);
                }
            }
            _ => res = format!("HTTP/1.1 {}\r\n\r\n", RESPONSE_NOT_FOUND),
        }
    }

    // Send the response back to the client
    if let Err(e) = stream.write_all(res.as_bytes()) {
        println!("Error writing to stream: {}", e);
    }
}

fn parse_request(stream: &mut TcpStream) -> Vec<String> {
    let mut buf_reader = BufReader::new(stream);
    let mut http_req = Vec::new();
    let mut buffer = Vec::new();  // Buffer to store read data

    loop {
        buffer.clear();
        match buf_reader.read_until(b'\n', &mut buffer) {
            Ok(0) => break,  // End of stream
            Ok(_) => {
                // Convert buffer to string and push to http_req
                let line = String::from_utf8_lossy(&buffer);
                http_req.push(line.trim().to_string());
            }
            Err(_) => break,  // Error or EOF
        }
    }

    http_req
}

fn echo_response(input: &str, content_type: &str) -> String {
    format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        RESPONSE_OK,
        content_type,
        input.len(),
        input
    )
}
