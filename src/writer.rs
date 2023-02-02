pub type Result<T> = std::result::Result<T, Box<Error>>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The written output is no valid UTF8-String: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("The xml write process failed because of an internal error: {0}")]
    XmlError(#[from] xml::writer::Error),
    #[error("The xml write process failed because of an io-error: {0}")]
    IoError(#[from] std::io::Error),
}
