use std::{
    env, fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
                ()
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let _ = stream.read(&mut buffer);
    let request = String::from_utf8_lossy(&buffer[..]).to_string();

    // ex: (GET /index.html HTTP/1.1, other params like user-agent)
    let (header, parameters): (&str, &str) = request.split_once("\r\n").unwrap();
    let header: Vec<&str> = header.split(" ").collect();
    let parameters: Vec<&str> = parameters.split("\r\n").collect();

    let path: &str = header[1];

    match path {
        p if p.starts_with("/echo/") => {
            let echo_data = p.strip_prefix("/echo/").unwrap();
            let response_string = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                echo_data.len(),
                echo_data
            );

            respond(stream, &response_string)
        }
        p if p.starts_with("/user-agent") => {
            let mut user_agent = "";
            for param in parameters.iter() {
                if param.contains("User-Agent") {
                    user_agent = param.split_once(": ").unwrap().1;
                    break;
                }
            }
            let response_string = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                user_agent.len(),
                user_agent
            );

            respond(stream, &response_string)
        }
        p if p.starts_with("/files/") => {
            let args: Vec<String> = env::args().collect();
            let dir_arg_index = args.iter().position(|a| a == "--directory");
            let mut response = String::new();

            match dir_arg_index {
                Some(index) => {
                    let abs_dir = &args[index + 1];
                    let file_name = path.split_once("/files/").unwrap().1;
                    let abs_file_path = format!("{}{}", abs_dir, file_name);

                    match fs::read_to_string(abs_file_path) {
                        Ok(it) => {
                            let content_len = &it.len();
                            let len_header = format!("Content-Length: {}\r\n\r\n", content_len);
                            let contents = &format!("{}\r\n\r\n", it);

                            response.push_str("HTTP/1.1 200 OK\r\n");
                            response.push_str("Content-Type: application/octet-stream\r\n");
                            response.push_str(&len_header);
                            response.push_str(contents);
                        }
                        Err(_) => {
                            response.push_str("HTTP/1.1 404 Not Found\r\n\r\n");
                        }
                    };

                    respond(stream, &response)
                }
                None => respond(stream, "HTTP/1.1 404 Not Found\r\n\r\n"),
            }
        }
        "/" => respond(stream, "HTTP/1.1 200 OK\r\n\r\n"),
        _ => respond(stream, "HTTP/1.1 404 Not Found\r\n\r\n"),
    };
}

fn respond(mut stream: TcpStream, response: &str) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
