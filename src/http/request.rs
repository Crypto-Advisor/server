use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::fmt::{Result as FmtResult, Formatter, Debug};


pub struct Request{
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {

    }
}

impl TryFrom<&[u8]> for Request{
    type Error = ParseError;

    

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

pub enum ParseError{
    InvalidMethod,
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
}

impl ParseError {
    fn message(&self) -> &str {
        match self{
            Self::InvalidMethod => "InvalidMethod",
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
        }
    }
}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message());
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message());
    }
}

impl Error for ParseError {
    
}