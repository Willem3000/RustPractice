use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{SocketAddr, TcpListener, TcpStream},
};

use http::ThreadPool;

const POOL_SIZE: usize = 100;

fn main() {
    let public_ip = "0.0.0.0:80";

    let addr: SocketAddr = public_ip.parse().expect("Invalid IP address");

    println!("Hosting at {addr}");

    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(POOL_SIZE);
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let connecting_ip = stream.local_addr().unwrap();
    let buf_reader = BufReader::new(&stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    let request_line: &str = if http_request.len() > 0 {
        let request_line = http_request.get(0).unwrap();
        println!("{} requested {}", connecting_ip, request_line);
        request_line
    } else {
        println!("{} sent empty request.", connecting_ip);
        "return GET / HTTP/1.1"
    };

    let (status_line, filename) = match request_line {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "main.html"),
        "GET /sleep HTTP/1.1" => {
            ("HTTP/1.1 200 OK", "main.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    
    let contents = get_page(filename);
    let length: usize = contents.len();

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",status_line, length, contents);

    stream.write_all(response.as_bytes()).unwrap();
}


fn get_page(filename: &str) -> String {
    return match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Failed to read file '{}': {}", filename, error);
            match fs::read_to_string("error.html") {
                Ok(error_html) => error_html,
                Err(error) => panic!("No error html found. {error}"),
            }
        }
    };
}

