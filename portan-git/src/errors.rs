use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An Io error {}", _0)]
    IoError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
