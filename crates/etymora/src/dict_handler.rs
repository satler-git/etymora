use crate::error::EtymoraError;

#[derive(Debug)]
pub(crate) enum Dicts {
    ExampleDict(example_adapter::ExampleDictionary),
}

impl etymora_traits::Dictionary for Dicts {
    type Error = EtymoraError;

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
    ) -> Result<etymora_traits::markdown_builder::Markdown, Self::Error> {
        match self {
            Dicts::ExampleDict(d) => d
                .lookup_ditail(word)
                .await
                .map_err(EtymoraError::ExampleAdapterError),
        }
    }
}
