use std::fmt;

#[derive(Debug, Clone)]
pub struct NoFilePath;

impl fmt::Display for NoFilePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expected one command line argument. None were provided.")
    }
}

impl std::error::Error for NoFilePath {}