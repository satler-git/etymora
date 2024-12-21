use crate::error::EtymoraError;
use etymora_traits::Dictionary;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(crate) enum Dicts {
    ExampleDict(adapter_example::ExampleDictionary),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum DictConfigs {
    Example(<adapter_example::ExampleDictionary as Dictionary>::InitInput),
}

impl etymora_traits::Dictionary for Dicts {
    type Error = EtymoraError;
    type InitInput = DictConfigs;

    async fn init(input: &Self::InitInput) -> Result<Self, Self::Error> {
        match input {
            DictConfigs::Example(p) => Ok(Dicts::ExampleDict(
                adapter_example::ExampleDictionary::init(&p)
                    .await
                    .map_err(EtymoraError::ExampleAdapterError)?,
            )),
        }
    }

    async fn exits(&self, word: &etymora_traits::Word) -> Result<bool, Self::Error> {
        match self {
            Dicts::ExampleDict(d) => d
                .exits(word)
                .await
                .map_err(EtymoraError::ExampleAdapterError),
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
                .map_err(EtymoraError::ExampleAdapterError),
        }
    }
}
