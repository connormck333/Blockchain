use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct Network {
    listener: TcpListener
}

impl Network {
    pub fn new() -> Network {
        Network {
            listener: TcpListener::bind("127.0.0.1:7878").unwrap()
        }
    }

    pub fn listen(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();

            Self::handle_connection(stream);
        }
    }

    pub fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}