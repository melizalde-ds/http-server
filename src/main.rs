use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // Parsing the request
                let mut buffer = [0; 1024];
                let read_result = stream.read(&mut buffer);
                match read_result {
                    Ok(_) => {
                        let _ = read_result.unwrap();
                    }
                    Err(e) => {
                        println!("error: {}", e);
                    }
                }
                let buffer = String::from_utf8_lossy(&buffer);
                parse_request(&buffer);

                // TODO: Find a use for the following variables in the code below
                #[allow(unused_variables)]
                let (method, path, headers, body_response) = parse_request(&buffer);
                let paths = parse_path(path);
                let headers_request = parse_headers(headers);

                // Checking the path
                let (code, message, content_length, body) = match paths[0] {
                    "" => (200, "OK", 0, "".to_string()),
                    "echo" => echo(method, &paths),
                    "user-agent" => user_agent(method, &headers_request),
                    _ => (404, "Not Found", 9, "Not Found".to_string()),
                };

                // Response headers
                let headers = vec![("Content-Type", "text/plain")];

                // Serializing the headers
                let headers_response = serialize_headers(headers);

                // Creating the response
                let response = format!(
                    "HTTP/1.1 {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
                    code, message, headers_response, content_length, body
                );

                // Writing the response
                let write_result = stream.write(response.as_bytes());
                match write_result {
                    Ok(_) => {
                        let _ = write_result.unwrap();
                    }
                    Err(e) => {
                        println!("error: {}", e);
                    }
                }

                // Flushing the stream
                let flush_result = stream.flush();
                match flush_result {
                    Ok(_) => {
                        let _ = flush_result.unwrap();
                    }
                    Err(e) => {
                        println!("error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn parse_request(request: &str) -> (&str, &str, Vec<&str>, &str) {
    let (headers, body) = request.split_once("\r\n\r\n").unwrap();
    let (first_line, headers) = headers.split_once("\r\n").unwrap();
    let first_line = first_line.split_whitespace().collect::<Vec<&str>>();
    let method = first_line[0];
    let path = first_line[1];
    let headers = headers.split("\r\n").collect::<Vec<&str>>();
    (method, path, headers, body)
}

fn parse_path(path: &str) -> Vec<&str> {
    let mut split = path.split("/");
    let _ = split.next();
    let path = split.collect::<Vec<&str>>();
    path
}

fn parse_headers(headers: Vec<&str>) -> Vec<(&str, &str)> {
    let headers = headers
        .iter()
        .map(|header| {
            let (key, value) = header.split_once(": ").unwrap();
            (key, value)
        })
        .collect::<Vec<(&str, &str)>>();
    headers
}

fn serialize_headers(headers: Vec<(&str, &str)>) -> String {
    let headers = headers
        .iter()
        .map(|(key, value)| format!("{}: {}\r\n", key, value))
        .collect::<String>();
    headers
}

fn echo<'a>(method: &'a str, paths: &'a Vec<&'a str>) -> (u16, &'a str, usize, String) {
    match method {
        "GET" => {
            if paths.len() == 2 {
                (200, "OK", paths[1].len(), paths[1].to_string())
            } else {
                (400, "Bad Request", 11, "Bad Request".to_string())
            }
        }
        _ => {
            return (
                405,
                "Method Not Allowed",
                17,
                "Method Not Allowed".to_string(),
            );
        }
    }
}

fn user_agent<'a>(
    method: &'a str,
    headers: &'a Vec<(&str, &'a str)>,
) -> (u16, &'a str, usize, String) {
    let mut user = "";
    match method {
        "GET" => {
            for (key, value) in headers {
                if key.to_string() == "User-Agent" {
                    user = value;
                }
            }
            match user {
                "" => (400, "Bad Request", 11, "Bad Request".to_string()),
                _ => (200, "OK", user.len(), user.to_string()),
            }
        }
        _ => {
            return (
                405,
                "Method Not Allowed",
                17,
                "Method Not Allowed".to_string(),
            );
        }
    }
}
