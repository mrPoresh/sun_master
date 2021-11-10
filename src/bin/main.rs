use std::io::prelude::*;    // for writting and reading in theard
use std::fs;                // for reading root directotry
use std::net::TcpListener;
use std::net::TcpStream;

use sun_master::ThreadPool;


fn main() {
    
    // can listen tcp
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    //*---------------------------------------------------------------------------------------------------*//
    //                                                                                                     //
    // Create pool of threards :                                                                           //
    //                                                                                                     //
    // - pool is a group of threards, who are expecting a task and are ready to complite it                //
    // - when server receives a task, it assigns it to one of its execution threards in the pool           //
    // - the rest of the threads in the pool can process other tasks while the first threard is procrssing //
    // - when the first threard finishes processing its task, it returns to the pool of idle theards       //
    //                                                                                                     //
    // With the help of a threard pool, reqests are processed i parallel, which allows you to              //
    //   increase the sevrer bandwith                                                                      //
    //                                                                                                     //
    //*---------------------------------------------------------------------------------------------------*//
    let pool = ThreadPool::new(4);

    // returns an iterator over the connections being received on this listener.
    for stream in listner.incoming() {  

        let stream = stream.unwrap();

        // take a closure and pass it to a threard from the pool for execution
        pool.execute(|| {
            // fn for handle new connection
            handle_connection(stream);
        });

    }
    
}

// mut -> because TcpStream can change
fn handle_connection(mut stream: TcpStream) {

    // in steck for reading data from reqest
    let mut buffer = [0; 512];
    // reading bytes from TcpStream and adding in buffer
    stream.read(&mut buffer).unwrap();

    println!("---* Reqest ->\n{}------------------------------\n", 
        String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    // reqest "handler"
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "auth.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // convent HTML file in str than create a server responce
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    // convent str in bytes for response and attempts to write an entire buffer into this writer
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}