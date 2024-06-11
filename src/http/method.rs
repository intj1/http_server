use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    POST,
    PUT,
    GET,
    DELETE,
    OPTION,
    HEAD
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "OPTION" => Ok(Self::OPTION),
            "HEAD" => Ok(Self::HEAD),
            _ => Err(MethodError)
        }
}}

pub struct MethodError;