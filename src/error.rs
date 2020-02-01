#[derive(Debug)]
pub struct Error {
    pub cause: String
}

impl Error {

    pub fn new<C: Into<String>>(cause: C) -> Error {
        Error {
            cause: cause.into()
        }
    }

}

impl std::fmt::Display for Error {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cause)
    }

}
