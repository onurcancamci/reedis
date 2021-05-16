#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MyError {
    TODO,
    SocketError,
    EventChannelClosed,
    SocketReadError,
    KeyNotFound,
    PathContainsPrimitiveValue,
    KeyAlreadyExists,
    MalformedCommand,
    NumberCantBeParsed,
    ArrayParseError,
}
