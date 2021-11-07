#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error<I> {
    input: I,
    code: ErrorKind,
}

impl<I> Error<I> {
    pub fn new(input: I, code: ErrorKind) -> Self {
        Self { input, code }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Tag(String),
    Ident,
    Float,
    Int,
    AnyChar,
    Comparison,
}
