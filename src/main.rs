use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878");
    
    match stream {
        Ok(_stream) => println!("[SUCCESS] connected to the server"), 
        Err(_) => println!("[ERROR] could not connect to the server"),
    }
}
