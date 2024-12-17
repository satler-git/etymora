//! Text Document Module
//! For extracting words from text documents.
//! Exstract a word from a line.
//! TODO: multiple word for some English idioms

use lsp_types::Position;
use rustc_hash::FxHashMap;

use std::{ops::Deref, path::PathBuf};
use tokio::fs;

#[derive(Debug, thiserror::Error)]
pub(crate) enum FsError {
    #[error("Given Uri has wrong scheme")]
    WrongScheme,
    #[error("{0}")]
    IoError(#[source] tokio::io::Error),
}

#[derive(Debug, Default)]
pub(crate) struct FileSystem {
    map: FxHashMap<PathBuf, fs::File>,
}

fn try_from_uri(value: &lsp_types::Uri) -> Result<PathBuf, FsError> {
    if value
        .scheme()
        .map(|s| s.to_lowercase().as_str().to_string())
        != Some("file".into())
    {
        Err(FsError::WrongScheme)
    } else {
        Ok(value.path().as_str().into())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use lsp_types::Uri;

    use tokio::fs;

    async fn create_tempfile(path_suffix: &str) -> Result<(fs::File, PathBuf), std::io::Error> {
        let path = tempfile::tempdir()?.path().join(path_suffix);

        Ok((fs::File::create_new(&path).await?, path))
    }

    #[tokio::test]
    async fn test_try_from_uri() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            PathBuf::from_str("/example")?,
            try_from_uri(&Uri::from_str("file:///example")?)?
        );

        // Wrong scheme
        assert!(try_from_uri(&Uri::from_str("https://example.com/")?).is_err());
        Ok(())
    }
}
