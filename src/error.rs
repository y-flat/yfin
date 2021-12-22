use std::fmt;

pub enum Error {
    ParseError,
    RequestFailed,
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseError => write!(
                f,
                "Failed to parse package.yml, package may not be implemented properly"
            ),
            Error::RequestFailed => write!(
                f,
                "Failed to grab package.yml, package may not exist or github may be down"
            ),
            Error::Unknown => write!(f, "Failed for an unknown reason"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseError => write!(f, "ParseError"),
            Error::RequestFailed => write!(f, "RequestFailed"),
            Error::Unknown => write!(f, "Unknown"),
        }
    }
}
