use etymora_traits::markdown_builder::Markdown;
use etymora_traits::{Dictionary, Word};
use thiserror::Error;

pub struct ExampleDictionary;

#[derive(Debug, Error)]
pub enum ExampleError {
    #[error("Example error has occrued")]
    Error,
}

impl Dictionary for ExampleDictionary {
    type Error = ExampleError;

    async fn exits(&self, _word: &Word) -> Result<bool, Self::Error> {
        Ok(true)
    }

    async fn lookup_ditail(&self, word: &Word) -> Result<Markdown, Self::Error> {
        let mut doc = Markdown::new();

        doc.header1(format!("{word}"));

        doc.paragraph(format!("{word} meaning is idk..."));

        Ok(doc)
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
        let doc = dict.lookup_ditail(&"lang".into()).await.unwrap().render();
        assert_eq!(doc.as_str(), "# lang\n\nlang meaning is idk...\n");
    }
}
