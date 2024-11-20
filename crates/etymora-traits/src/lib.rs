pub struct Word(String);

impl Word {
    pub fn inner(self) -> String {
        self.0
    }
}

impl<T: Into<String>> From<T> for Word {
    fn from(value: T) -> Self {
        let w: String = value.into().trim().into();
        Word(w)
    }
}

pub trait Dictionary {
    type Error;

    fn exits(
        &self,
        word: &Word,
    ) -> impl std::future::Future<Output = Result<bool, Self::Error>> + Send;
    fn lookup_ditail(
        &self,
        word: &Word,
    ) -> impl std::future::Future<Output = Result<markdown_builder::Markdown, Self::Error>> + Send;
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
