use thiserror::Error;

#[derive(Debug, Error)]
pub enum WiktionaryError {}

pub type Result<T> = std::result::Result<T, WiktionaryError>;
