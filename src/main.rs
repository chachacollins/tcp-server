use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.write("ur mom\n".as_bytes()).unwrap();
    let read_data = stream.read(&mut buffer).unwrap();

    println!("read {} bytes", read_data);

    let received_message = std::str::from_utf8(&buffer[..read_data]).unwrap();
    println!("received message: {}", received_message);
    stream.write_all(received_message.as_bytes()).unwrap();
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let addr = "127.0.0.1:6969";
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        handle_stream(stream?);
    }

    Ok(())
}
