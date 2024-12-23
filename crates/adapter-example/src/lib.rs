use etymora_traits::markdown_builder::Markdown;
use etymora_traits::{Dictionary, Word};
use thiserror::Error;

#[derive(Debug)]
pub struct ExampleDictionary;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExampleError {
    #[error("Example error has occrued")]
    Error,
}

const CONFIG_URL: &'static str =
    "https://github.com/satler-git/etymora/blob/main/config-examples/etymora.lua";

impl Dictionary for ExampleDictionary {
    type Error = ExampleError;
    type InitInput = ();

    async fn init(_: &Self::InitInput) -> Result<Self, Self::Error> {
        Ok(ExampleDictionary)
    }

    #[tracing::instrument]
    async fn exits(&self, _word: &Word) -> Result<bool, Self::Error> {
        Ok(true)
    }

    #[tracing::instrument]
    async fn lookup_ditail(&self, word: &Word) -> Result<Option<Markdown>, Self::Error> {
        let mut doc = Markdown::new();

        doc.header1(format!("{word}"));

        doc.paragraph(
            "This message will may be seen when you didn't add the dictionary configuration.",
        );

        doc.paragraph(format!(
            "The sample configuration can be viewed [here]({CONFIG_URL})."
        ));

        Ok(Some(doc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_example_dictionary() {
        let dict = ExampleDictionary;
        assert!(dict.exits(&"lang".into()).await.unwrap());
    }

    #[tokio::test]
    async fn text_example_render() {
        let dict = ExampleDictionary;
        let doc = dict
            .lookup_ditail(&"lang".into())
            .await
            .unwrap()
            .unwrap()
            .render();

        assert_eq!(doc.as_str(), "# lang\n\nThis message will may be seen when you didn't add the dictionary configuration.\n\nThe sample configuration can be viewed\n[here](https://github.com/satler-git/etymora/blob/main/config-examples/etymora.lua).\n");
    }
}
