use std::path::PathBuf;

pub mod entry;
pub mod error;

use entry::Entry;
use error::{Result, WiktionaryError};
use rustc_hash::FxHashMap;

use tokio::io::{AsyncBufReadExt, AsyncSeekExt, SeekFrom};
use tracing::info_span;

#[derive(Debug)]
struct WiktinaryAdapter(FxHashMap<etymora_traits::Word, Entry>);

impl etymora_traits::Dictionary for WiktinaryAdapter {
    type Error = error::WiktionaryError;
    type InitInput = PathBuf;

    async fn init(path: &Self::InitInput) -> Result<Self> {
        let _init = info_span!("Init WiktinaryAdapter");

        let mut file = tokio::fs::File::open(&path)
            .await
            .map_err(WiktionaryError::FileNotFound)?;

        file.seek(SeekFrom::Start(0))
            .await
            .map_err(WiktionaryError::IO)?;

        let reader = tokio::io::BufReader::new(file);

        let mut lines = reader.lines();

        let mut map = FxHashMap::default();

        while let Some(line) = lines.next_line().await.map_err(WiktionaryError::IO)? {
            let (word, detail) = Entry::parse_from_tsv(line).unwrap();
            map.insert(word, detail);
        }
        Ok(WiktinaryAdapter(map))
    }

    async fn exits(&self, word: &etymora_traits::Word) -> Result<bool> {
        Ok(self.0.contains_key(word))
    }

    async fn lookup_ditail(
        &self,
        word: &etymora_traits::Word,
    ) -> Result<Option<etymora_traits::markdown_builder::Markdown>> {
        if let Some(_entry) = self.0.get(word) {
            todo!()
        } else {
            Ok(None)
        }
    }
}
