#[derive(Debug, Eq, PartialEq)]
pub enum MyError {
    TODO,
    SocketError,
    EventChannelClosed,
    SocketReadError,
    KeyNotFound,
    PathContainsPrimitiveValue,
    KeyAlreadyExists,
    MalformedCommand,
}
