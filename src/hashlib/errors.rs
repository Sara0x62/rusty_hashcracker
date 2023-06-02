
#[derive(thiserror::Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]    // - For testing purposes
pub enum HashTypeError {
    #[error("Invalid hash size of {0} bits")]
    InvalidHashSize(usize),
}

#[derive(thiserror::Error, Debug)]
pub enum HashError {
    #[error("No match found for {0}")]
    NoMatchFound(String),

    #[error("File not found '{0}'")]
    FileNotFound(String),

}