use std::fs;
use std::io::prelude::*;
use std::io::{self, BufReader, Write};
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap_or_else(|e| {
        eprintln!("Failed to bind to address: {}", e);
        std::process::exit(1);
    });

    println!("Server is running on {}", listener.local_addr().unwrap());

    // Handle incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                // Handle the connection
                println!("New connection established.");
                handle_connection(_stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

//response type of the server
// HTTP-Version Status-code Reason-Phrase CRLF
// headers CRLF
// body

// example response:
// HTTP/1.1 200 OK\r\n
// Content-Type: text/plain\r\n// Content-Length: 13\r\n
// \r\n
// Hello, World!

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            println!(
                "Received request: {}",
                String::from_utf8_lossy(&buffer[..size])
            );

            let get_request = b"GET / HTTP/1.1\r\n";
            let (status_line, filename) = if buffer.starts_with(get_request) {
                println!("Received a GET request.");
                ("HTTP/1.1 200 OK\r\n", "index.html")
            } else {
                println!("Received a non-GET request.");
                ("HTTP/1.1 404 NOT FOUND\r\n", "404.html")
            };
            // Here we would typically parse the request and respond accordingly.
            // For simplicity, we'll just read the index.html file and send it as a response.
            let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read index.html");
                String::new()
            });

            let response = format!(
                "{}Content-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                contents.len(),
                contents
            );

            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("Failed to write to stream: {}", e);
            }
            if let Err(e) = stream.flush() {
                eprintln!("Failed to flush stream: {}", e);
            }
            // Here you would typically parse the request and send a response
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}
