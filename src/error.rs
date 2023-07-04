use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub cause: String,
}

impl Error {
    pub fn new<C: Into<String>>(cause: C) -> Error {
        Error {
            cause: cause.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cause)
    }
}
