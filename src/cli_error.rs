use std::{fmt, io, str};

pub enum CliError {
    Io(io::Error),
    String(str::Utf8Error),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<std::str::Utf8Error> for CliError {
    fn from(err: std::str::Utf8Error) -> CliError {
        CliError::String(err)
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Io(ref err) => write!(f, "{}", err),
            CliError::String(ref err) => write!(f, "{}", err),
        }
    }
}
