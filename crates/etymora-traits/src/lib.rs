#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Word(String);

pub use markdown_builder;

impl Word {
    pub fn inner(self) -> String {
        self.0
    }
}

pub fn from_markdown(value: markdown_builder::Markdown) -> lsp_types::MarkupContent {
    lsp_types::MarkupContent {
        kind: lsp_types::MarkupKind::Markdown,
        value: value.render(),
    }
}

impl<T: Into<String>> From<T> for Word {
    fn from(value: T) -> Self {
        let w: String = value.into().trim().into();
        Word(w)
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub trait Dictionary: Sized {
    type Error;
    type InitInput: serde::Serialize;

    fn init(
        input: &Self::InitInput,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send;

    fn exits(
        &self,
        word: &Word,
    ) -> impl std::future::Future<Output = Result<bool, Self::Error>> + Send;

    fn lookup_ditail(
        &self,
        word: &Word,
    ) -> impl std::future::Future<Output = Result<Option<markdown_builder::Markdown>, Self::Error>> + Send;
}

#[cfg(test)]
mod tests {
    use super::Word;

    #[test]
    fn test_word() {
        let word: Word = "  lang\n\r".into();
        assert_eq!(word.inner(), "lang".to_string());
    }
}
