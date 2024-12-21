//! Text Document Module
//! For extracting words from text documents.
//! Exstract a word from a line.
//! TODO: multiple word for some English idioms

use etymora_traits::Word;
use lsp_types::Position;
use rustc_hash::FxHashMap;

use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader, SeekFrom};

#[derive(Debug, thiserror::Error)]
pub(crate) enum FsError {
    #[error("Given Uri has wrong scheme")]
    WrongScheme,
    #[error("{0}")]
    IoError(#[source] tokio::io::Error),
    #[error("Wrong position")]
    WrongPosition,
}

#[derive(Debug, Default)]
pub(crate) struct FileSystem {
    map: Arc<RwLock<FxHashMap<PathBuf, fs::File>>>,
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
    async fn read_line(&self, path: &PathBuf, position: &Position) -> Result<String, FsError> {
        let mut map = self.map.write().unwrap();
        if !map.contains_key(path) {
            // Error型に
            map.insert(
                path.clone(),
                fs::File::open(&path).await.map_err(FsError::IoError)?,
            );
        }

        let file = map.get_mut(path).unwrap();

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

        Err(FsError::WrongPosition)
    }

    /// A wrapped function for `read_word`
    pub(crate) async fn read_word_uri(
        &self,
        uri: &lsp_types::Uri,
        position: &Position,
    ) -> Result<Option<Word>, FsError> {
        self.read_word(&try_from_uri(uri)?, position).await
    }

    /// read word
    pub(crate) async fn read_word(
        &self,
        path: &PathBuf,
        position: &Position,
    ) -> Result<Option<Word>, FsError> {
        Ok(extract_word_from_line(
            self.read_line(path, position).await?,
            position,
        ))
    }
}

/// Extract the word(lowercase, and ascii alphabet only) at the cursor position
fn extract_word_from_line(s: String, position: &Position) -> Option<Word> {
    let mut return_string: Option<String> = None;
    let mut is_return_word = false;
    for (i, ci) in s.chars().enumerate() {
        // dbg!(i, ci, is_return_word, &return_string, &s);
        if i as u32 == position.character {
            // カーソルの位置の単語を返すべきとしてマーク
            is_return_word = true;
        }
        if !ci.is_ascii_alphabetic() {
            if is_return_word {
                break;
            } else {
                return_string = None;
            }
        } else {
            if return_string.is_some() {
                return_string.as_mut().unwrap().push(ci);
            } else {
                return_string = Some(format!("{ci}"));
            }
        }
    }

    return_string.map(|s| s.to_lowercase().into())
}

#[cfg(test)]
mod tests {
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

        let fs = FileSystem::default();

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

        Ok(())
    }

    #[test]
    fn test_extract_word_from_line() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            extract_word_from_line(
                "testword\n".into(),
                &Position {
                    line: 0,
                    character: 0
                }
            ),
            Some(Word::from("testword".to_string()))
        );

        assert_eq!(
            extract_word_from_line(
                "testword(6)\n".into(),
                &Position {
                    line: 0,
                    character: 0
                }
            ),
            Some(Word::from("testword".to_string()))
        );

        assert_eq!(
            extract_word_from_line(
                "testword!()[]{}&%\n".into(),
                &Position {
                    line: 0,
                    character: 0
                }
            ),
            Some(Word::from("testword".to_string()))
        );

        assert_eq!(
            extract_word_from_line(
                "testworD6\n".into(),
                &Position {
                    line: 0,
                    character: 0
                }
            ),
            Some(Word::from("testword".to_string()))
        );

        assert_eq!(
            extract_word_from_line(
                "lorem ipsum\n".into(),
                &Position {
                    line: 0,
                    character: 0
                }
            ),
            Some(Word::from("lorem".to_string()))
        );

        assert_eq!(
            extract_word_from_line(
                "lorem ipsum\n".into(),
                &Position {
                    line: 0,
                    character: 7
                }
            ),
            Some(Word::from("ipsum".to_string()))
        );

        assert_eq!(
            extract_word_from_line(
                "".into(),
                &Position {
                    line: 0,
                    character: 0
                }
            ),
            None
        );

        Ok(())
    }
}
