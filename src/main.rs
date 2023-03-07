use std::error;
use std::fs::File;
use std::io;
use std::net::{self, Ipv6Addr};
use std::fmt;



#[derive(Debug)]
enum UpsteamError {
    IO(io::Error),
    Parsing(net::AddrParseError)
}

impl fmt::Display for UpsteamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for UpsteamError {}

impl From<io::Error> for UpsteamError {
    fn from(error: io::Error) -> Self {
        UpsteamError::IO(error)
    }
}

impl From<net::AddrParseError> for UpsteamError {
    fn from(error: net::AddrParseError) -> Self {
        UpsteamError::Parsing(error)
    }
}


fn main() -> Result<(), UpsteamError> {
    let _f = File::open("invisible.txt")?;
    let _localhost = "::1".parse::<Ipv6Addr>()?;
    Ok(())
}