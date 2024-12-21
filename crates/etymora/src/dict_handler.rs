use crate::error::EtymoraError;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(crate) enum Dicts {
    ExampleDict(adapter_example::ExampleDictionary),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DictConfigs {
    Example,
}

impl Default for DictConfigs {
    fn default() -> Self {
        DictConfigs::Example
    }
}

impl etymora_traits::Dictionary for Dicts {
    type Error = EtymoraError;
    type InitInput = DictConfigs;

    async fn init(input: &Self::InitInput) -> Result<Self, Self::Error> {
        match input {
            DictConfigs::Example => Ok(Dicts::ExampleDict(
                adapter_example::ExampleDictionary::init(&())
                    .await
                    .map_err(EtymoraError::ExampleAdapter)?,
            )),
        }
    }

    async fn exits(&self, word: &etymora_traits::Word) -> Result<bool, Self::Error> {
        match self {
            Dicts::ExampleDict(d) => d.exits(word).await.map_err(EtymoraError::ExampleAdapter),
        }
    }

    async fn lookup_ditail(
        &self,
        word: &etymora_traits::Word,
    ) -> Result<Option<etymora_traits::markdown_builder::Markdown>, Self::Error> {
        match self {
            Dicts::ExampleDict(d) => d
                .lookup_ditail(word)
                .await
                .map_err(EtymoraError::ExampleAdapter),
        }
    }
}
