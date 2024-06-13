use std::{io::Result, net::TcpStream, io::Write};

use super::status_code::StatusCode;

pub struct Response {
   status_code: StatusCode,
   body: Option<String> 
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self {status_code, body}
    }

    pub fn send(&self, stream: &mut TcpStream) ->Result<()> {
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_string(),
            match &self.body {
                Some(msg) => msg.to_owned(),
                _ => "".to_owned()
            })
        }
}
