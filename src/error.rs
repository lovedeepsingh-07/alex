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

impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::InvalidInputError(err_str) => format!("InvalidInputError: {}", err_str),
            Error::NotFoundError(err_str) => format!("NotFoundError: {}", err_str),
            Error::IOError(err_str) => format!("IOError: {}", err_str),
            Error::FSError(err_str) => format!("FSError: {}", err_str),
            Error::ProtocolError(err_str) => format!("ProtocolError: {}", err_str),
            Error::StreamError(err_str) => format!("StreamError: {}", err_str),
            Error::DecoderError(err_str) => format!("DecoderError: {}", err_str),
            Error::DeserializeError(err_str) => format!("DeserializeError: {}", err_str),
            Error::LoftyError(err_str) => format!("LoftyError: {}", err_str),
            Error::ChannelSendError(err_str) => format!("ChannelSendError: {}", err_str),
            Error::ChannelReceiveError(err_str) => format!("ChannelReceiveError: {}", err_str),
            Error::PlayerError(err_str) => format!("PlayerError: {}", err_str),
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
