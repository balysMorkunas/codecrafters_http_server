use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 512];
                let _ = stream.read(&mut buffer);
                let request = String::from_utf8_lossy(&buffer[..]).to_string();
                println!("{}", request);

                // ex: GET /index.html HTTP/1.1
                let header: Vec<&str> = request.split_once("\r\n").unwrap().0.split(" ").collect();

                let path: &str = header[1];

                match path {
                    "/" => respond(stream, "HTTP/1.1 200 OK\r\n\r\n"),
                    p if p.starts_with("/echo/") => {
                        let echo_data = p.strip_prefix("/echo/").unwrap();
                        let response_string = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", echo_data.len(), echo_data);

                        respond(stream, &response_string)
                    }
                    _ => respond(stream, "HTTP/1.1 404 Not Found\r\n\r\n"),
                };
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn respond(mut stream: TcpStream, response: &str) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
