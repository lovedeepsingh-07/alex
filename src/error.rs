#[derive(Debug)]
pub enum Error {
    IOError(String),
    ParseError(String),
    NotFoundError(String)
}

impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::IOError(err_str) => format!("IOError {}", err_str),
            Error::ParseError(err_str) => format!("ParseError {}", err_str),
            Error::NotFoundError(err_str) => format!("NotFoundError {}", err_str),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value.to_string())
    }
}
