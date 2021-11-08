use std::io::prelude::*;    // for writting and reading in theard
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;


fn main() {
    
    // can listen tcp
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();

    // returns an iterator over the connections being received on this listener.
    for stream in listner.incoming() {  

        let stream = stream.unwrap();

        // fn for handle new connection
        handle_connection(stream);

    }
    
}

// because TcpStream can change
fn handle_connection(mut stream: TcpStream) {

    // in steck for reading data from reqest
    let mut buffer = [0; 512];
    // reading bytes from TcpStream and adding in buffer
    stream.read(&mut buffer).unwrap();

    // convent HTML file in str
    let contents = fs::read_to_string("auth.html").unwrap();

    //server response 
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    // convent str in bytes for response
    stream.write(response.as_bytes()).unwrap();
    // attempts to write an entire buffer into this writer
    stream.flush().unwrap();

}