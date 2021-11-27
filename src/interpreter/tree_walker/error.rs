#[derive(Debug, Clone)]
pub struct Error {
    pub msg: String,
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(msg: &str, kind: ErrorKind) -> Self {
        Self {
            msg: msg.into(),
            kind,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    PrefixError,
    TypeError,
    ReturningNothing,
    MutationError,
    NotImplemented,
    Undefined,
    MisMatchedPramas,
}
