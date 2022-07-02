#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    BracketMismatch,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}
