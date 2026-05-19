use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn setup_server() {
    let listener = TcpListener::bind("127.0.0.1:4978").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&stream);

    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).unwrap();
    let request_line = request_line.trim_end();

    if request_line == "POST / HTTP/1.1" {
        let mut content_length = 0usize;

        loop {
            let mut line = String::new();
            buf_reader.read_line(&mut line).unwrap();

            if line == "\r\n" {
                break;
            }

            if let Some(value) = line.strip_prefix("Content-Length:") {
                content_length = value.trim().parse::<usize>().unwrap_or(0);
            }
        }

        let mut body = Vec::with_capacity(content_length);
        let mut remaining = content_length;
        let mut chunk = [0u8; 8192];

        while remaining > 0 {
            let to_read = remaining.min(chunk.len());
            buf_reader.read_exact(&mut chunk[..to_read]).unwrap();
            body.extend_from_slice(&chunk[..to_read]);
            remaining -= to_read;
        }

        let body_text = String::from_utf8_lossy(&body);

        println!("POST body: {}", body_text);

        let status_line = "HTTP/1.1 200 OK";

        let response = format!("{status_line}\r\nContent-Length: 0\r\n\r\n");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

        stream.write_all(response.as_bytes()).unwrap();
    }
}
