use super::method::{MethodError, Method};
use super::util::get_next_word;
use super::query_string::QueryString;
use std::convert::TryFrom;
use std::fmt::{Display, Debug};
use std::error::Error;
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let req = str::from_utf8(buf)?;

        let (method, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        dbg!(method);
        let (mut path, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        dbg!(path);
        // let (protocol, _) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        // dbg!(protocol);


        // if protocol != "HTTP/1.1" {
        //     return Err(ParseError::InvalidProtocol);
        // }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[i..]
        }

        Ok(
            Self {
                path,
                query_string,
                method
            }
        )

    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.message()) 
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.message()) 
    }
}

impl Error for ParseError {

}