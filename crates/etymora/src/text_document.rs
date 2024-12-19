//! Text Document Module
//! For extracting words from text documents.
//! Exstract a word from a line.
//! TODO: multiple word for some English idioms

use etymora_traits::Word;
use lsp_types::Position;
use rustc_hash::FxHashMap;

use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader, SeekFrom};

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

impl FileSystem {
    async fn read_line_uri(
        &mut self,
        uri: &lsp_types::Uri,
        position: &Position,
    ) -> Result<String, FsError> {
        self.read_line(&try_from_uri(uri)?, position).await
    }

    async fn read_line(&mut self, path: &PathBuf, position: &Position) -> Result<String, FsError> {
        if !self.map.contains_key(path) {
            self.map.insert(
                path.clone(),
                fs::File::open(&path).await.map_err(FsError::IoError)?,
            );
        }

        let file = self.map.get_mut(path).unwrap(); // TODO: 多分IO重い

        file.seek(SeekFrom::Start(0))
            .await
            .map_err(FsError::IoError)?;

        let reader = BufReader::new(file);

        let mut lines = reader.lines();
        let mut current_line = 0;

        while let Some(line) = lines.next_line().await.map_err(FsError::IoError)? {
            if current_line == position.line {
                return Ok(line);
            }
            current_line += 1;
        }

        Ok("".into())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::Permissions;
    use std::str::FromStr;

    use super::*;
    use lsp_types::Uri;

    use tempfile::TempDir;
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    async fn create_tempfile(
        path_suffix: &str,
    ) -> Result<(fs::File, PathBuf, TempDir), std::io::Error> {
        let dir = tempfile::tempdir()?;

        let path = dir.path().join(path_suffix);

        Ok((fs::File::create(&path).await?, path, dir)) // dirを返さないと消されちゃう
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

    #[tokio::test]
    async fn test_read_line() -> Result<(), Box<dyn std::error::Error>> {
        let (mut file, path, _tempdir) = create_tempfile("test1").await?;

        file.write_all(
            b"0
1
2
3
4
55
        ",
        )
        .await?;

        let mut fs = FileSystem::default();

        assert_eq!(
            fs.read_line(
                &path,
                &Position {
                    line: 5,
                    character: 0,
                },
            )
            .await?,
            "55"
        );

        assert_eq!(
            fs.read_line(
                &path,
                &Position {
                    line: 0,
                    character: 0,
                },
            )
            .await?,
            "0"
        );

        assert_eq!(
            fs.read_line_uri(
                &Uri::from_str(&format!("file://{}", path.to_str().unwrap()))?,
                &Position {
                    line: 3,
                    character: 0
                }
            )
            .await?,
            "3"
        );

        Ok(())
    }
}
