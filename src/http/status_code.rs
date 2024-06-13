use std::{fmt::Formatter, fmt::Display, fmt::Result as FmtResult};

#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404
}

impl StatusCode {
    pub fn reason_string(&self) -> &str {
        match self {
            StatusCode::Ok => "Ok",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found" 
        }
    }
}


impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}