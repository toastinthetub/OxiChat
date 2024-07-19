use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidServerAddress;

impl fmt::Display for InvalidServerAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid server address format")
    }
}

impl Error for InvalidServerAddress {}

#[derive(Debug)]
pub struct InvalidArguments;

impl fmt::Display for InvalidArguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid number of arguments")
    }
}

impl Error for InvalidArguments {}
