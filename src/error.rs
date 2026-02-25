#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputError(String),
    NotFoundError(String),
    IOError(String),
    FSError(String),
    ProtocolError(String),
    StreamError(String),
    DecoderError(String),
    DeserializeError(String),
    LoftyError(String),
    ChannelSendError(String),
    ChannelReceiveError(String),
    PlayerError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidInputError(err_str) => write!(f, "InvalidInputError: {}", err_str),
            Error::NotFoundError(err_str) => write!(f, "NotFoundError: {}", err_str),
            Error::IOError(err_str) => write!(f, "IOError: {}", err_str),
            Error::FSError(err_str) => write!(f, "FSError: {}", err_str),
            Error::ProtocolError(err_str) => write!(f, "ProtocolError: {}", err_str),
            Error::StreamError(err_str) => write!(f, "StreamError: {}", err_str),
            Error::DecoderError(err_str) => write!(f, "DecoderError: {}", err_str),
            Error::DeserializeError(err_str) => write!(f, "DeserializeError: {}", err_str),
            Error::LoftyError(err_str) => write!(f, "LoftyError: {}", err_str),
            Error::ChannelSendError(err_str) => write!(f, "ChannelSendError: {}", err_str),
            Error::ChannelReceiveError(err_str) => write!(f, "ChannelReceiveError: {}", err_str),
            Error::PlayerError(err_str) => write!(f, "PlayerError: {}", err_str),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value.to_string())
    }
}
impl From<rodio::stream::StreamError> for Error {
    fn from(value: rodio::stream::StreamError) -> Self {
        Error::StreamError(value.to_string())
    }
}
impl From<rodio::decoder::DecoderError> for Error {
    fn from(value: rodio::decoder::DecoderError) -> Self {
        Error::DecoderError(value.to_string())
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::DeserializeError(value.to_string())
    }
}
impl From<bitcode::Error> for Error {
    fn from(value: bitcode::Error) -> Self {
        Error::DeserializeError(value.to_string())
    }
}
impl From<lofty::error::LoftyError> for Error {
    fn from(value: lofty::error::LoftyError) -> Self {
        Error::LoftyError(value.to_string())
    }
}
impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(value: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Error::ChannelSendError(value.to_string())
    }
}
