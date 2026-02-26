use std::io::ErrorKind;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt};

use crate::http::request::HttpRequestParser;

pub mod http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Listening on port: 8080");

    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                println!("New client: {:?}", addr);
                tokio::spawn(async move {
                    process_connection(&mut socket).await;
                });
            }
            Err(e) => {
                // We just log it and continue. The server stays ALIVE.
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}

async fn process_connection(socket: &mut TcpStream) -> std::io::Result<()> {
    let parser = HttpRequestParser::new();
    let buffer = [0u8; 4096];
    
    loop {
        let read_result = socket.read(&mut buffer).await;
        
        let n = match read_result {
            Ok(n) if n > 0 => n,
            Ok(0) => {
                println!("Client disconnected.");
                return Ok(());
            },
            Err(e) if e.kind() == ErrorKind::ConnectionReset => {
                eprintln!("Client reset by peer.");
                return Err(e);
            },
            Err(e) if e.kind() == ErrorKind::Interrupted => {
                continue;
            }
            Err(e) => {
                eprintln!("Unexpected socket error: {}", e);
                return Err(e);
            }
        };
        
        match parser.push_bytes(&buffer[0..n]) {
            //TODO
        }
    }
}