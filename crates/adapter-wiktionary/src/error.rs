use thiserror::Error;

#[derive(Debug, Error)]
pub enum WiktionaryError {
    #[error("= does not inclued in this entry detail")]
    IsNotDetail,
    #[error("This entry does not inclued the title")]
    TitleNotInclued,
    #[error("File Not found: {0}")]
    FileNotFound(#[source] tokio::io::Error),
    #[error("Io Error has occured: {0}")]
    IO(#[source] tokio::io::Error),
}

pub type Result<T> = std::result::Result<T, WiktionaryError>;
