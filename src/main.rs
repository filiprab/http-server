use std::{net::{TcpListener, TcpStream}, thread, io::{BufReader, BufRead}};

pub mod http;


fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&mut stream);
    let mut header_buffer = Vec::new();

    reader.read_until(b'\n', &mut header_buffer).unwrap();

    for &letter in header_buffer.iter() {
        print!("{:?} ", letter as char);
    }
    println!("");
}



fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:80")?;
    println!("Listening on port: 80");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().unwrap();
                println!("Handling request from: {}", addr);

                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => { eprintln!("Connection failed {:?}", e) }
        }
    }

    Ok(())
}
