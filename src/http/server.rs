use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::response;

use super::{request::Request, response::Response, status_code::StatusCode};

#[derive(Debug)]
pub struct Server {
    pub addr: String,
    pub port: u32
}

impl Server {
    pub fn new(addr: String, port: Option<u32>) -> Self {
        Self {
            addr,
            port: match port {
                Some(p) => p,
                None => 8080
            }
        }
    }

    pub fn run(self) {
        println!("Listening on {}:{}", self.addr, self.port);

        let listener: TcpListener = TcpListener::bind(format!("{}:{}", self.addr, self.port)).unwrap();

        loop {

            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received the following request: {}", String::from_utf8_lossy(&buf));
                            let resp = match Request::try_from(&buf[..]) {
                                Ok(req) => {
                                    dbg!(req);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("HELLO".to_string())
                                    )
                                },
                                Err(e) => {
                                    println!("failed to parse request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            if let Err(e) = resp.send(&mut stream) {
                                println!("Failed to send response: {}", e)
                            }

                        },
                        _ => {println!("ERROR");}
                    }
                },
                _ => continue
            }



        }
    }
}