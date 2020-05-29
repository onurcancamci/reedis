use std::error::Error;
use std::fmt::{Display, Formatter};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    SocketTerminated,
    PacketIsNotFinished,
    SocketReadError,
    SocketWriteError,
    InvalidCommandType,
    InvalidDataType,
    InvalidString,
    InvalidCaommdnForServer,
    SizeCalculationIsInvalid,
    GetError,
    ChannelWriteError,
    SetError,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for AppError {}
