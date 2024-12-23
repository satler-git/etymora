use etymora_traits::Word;

use crate::error::{Result, WiktionaryError};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Entry {
    /// 名詞
    noun: Vec<String>,
    /// 動詞の意味
    verb: Vec<String>,
    pronoun: Vec<String>,
    /// 前置詞
    preposition: Vec<String>,
    /// 略
    abbreviation: Vec<String>,
    /// 形容詞
    adjective: Vec<String>,
    adverb: Vec<String>,
    /// 接続詞
    conjunction: Vec<String>,
    prefix: Vec<String>,
    determiner: Vec<String>,
    /// 複数形
    inflection_noun_plural: Vec<String>,
    /// 三人称単数
    inflection_verb_singular: Option<String>,
    /// 現在分詞
    inflection_verb_present_participle: Option<String>,
    /// 過去形
    inflection_verb_past: Option<String>,
    /// 過去分詞
    inflection_verb_past_participle: Option<String>,
    /// IPA発音記号
    pronunciation_ipa: Vec<String>,
    pronunciation_sampa: Vec<String>,
    pronunciation_ahd: Vec<String>,
    alternative: Option<String>,
    derivative: Option<String>,
    antonym: Option<String>,
    synonym: Option<String>,
    relation: Option<String>,
    inflection_adjective_comparative: Option<String>,
    inflection_adjective_superlative: Option<String>,
    interjection: Vec<String>,
    /// 上位語
    hypernym: Option<String>,
    /// 下位語
    hyponym: Option<String>,
    auxverb: Vec<String>,
    article: Vec<String>,
    inflection_adverb_comparative: Option<String>,
    inflection_adverb_superlative: Option<String>,
    suffix: Vec<String>,
}

macro_rules! detail {
    ($key:ident, $struct:ident, $value:ident) => {{
        if $struct.$key != None {
            panic!("{}", stringify!($struct.$key))
        }
        $struct.$key = Some($value.into());
    }};
}

macro_rules! meaning {
    ($key:ident, $struct:ident, $value:ident) => {{
        $struct.$key.push($value.into());
    }};
}

impl Entry {
    pub fn parse_from_tsv(value: String) -> Result<(Word, Self)> {
        let splited = value.split('\t');
        let mut r = Entry::default();
        let mut word = None;

        for entry_detail in splited {
            let (title, detail) = entry_detail
                .split_once('=')
                .ok_or(WiktionaryError::IsNotDetail)?;

            match title {
                // TODO:
                "word" => word = Some(Word::from(detail)),
                stringify!(abbreviation) => meaning!(abbreviation, r, detail),
                stringify!(adjective) => meaning!(adjective, r, detail),
                stringify!(adverb) => meaning!(adverb, r, detail),
                stringify!(conjunction) => meaning!(conjunction, r, detail),
                stringify!(interjection) => meaning!(interjection, r, detail),
                stringify!(noun) => meaning!(noun, r, detail),
                stringify!(prefix) => meaning!(prefix, r, detail),
                stringify!(preposition) => meaning!(preposition, r, detail),
                stringify!(pronoun) => meaning!(pronoun, r, detail),
                stringify!(verb) => meaning!(verb, r, detail),
                stringify!(determiner) => meaning!(determiner, r, detail),
                stringify!(inflection_verb_singular) => {
                    detail!(inflection_verb_singular, r, detail)
                }
                stringify!(inflection_verb_present_participle) => {
                    detail!(inflection_verb_present_participle, r, detail)
                }
                stringify!(inflection_verb_past) => detail!(inflection_verb_past, r, detail),
                stringify!(inflection_verb_past_participle) => {
                    detail!(inflection_verb_past_participle, r, detail)
                }
                stringify!(pronunciation_ipa) => meaning!(pronunciation_ipa, r, detail),
                stringify!(alternative) => detail!(alternative, r, detail),
                stringify!(derivative) => detail!(derivative, r, detail),
                stringify!(antonym) => detail!(antonym, r, detail),
                stringify!(synonym) => detail!(synonym, r, detail),
                stringify!(relation) => detail!(relation, r, detail),
                stringify!(inflection_adjective_comparative) => {
                    detail!(inflection_adjective_comparative, r, detail)
                }
                stringify!(inflection_adjective_superlative) => {
                    detail!(inflection_adjective_superlative, r, detail)
                }
                stringify!(inflection_noun_plural) => {
                    meaning!(inflection_noun_plural, r, detail)
                }
                stringify!(pronunciation_sampa) => meaning!(pronunciation_sampa, r, detail),
                stringify!(hypernym) => detail!(hypernym, r, detail),
                stringify!(hyponym) => detail!(hyponym, r, detail),
                stringify!(pronunciation_ahd) => meaning!(pronunciation_ahd, r, detail),
                stringify!(auxverb) => meaning!(auxverb, r, detail),
                stringify!(article) => meaning!(article, r, detail),
                stringify!(inflection_adverb_comparative) => {
                    detail!(inflection_adverb_comparative, r, detail)
                }
                stringify!(inflection_adverb_superlative) => {
                    detail!(inflection_adverb_superlative, r, detail)
                }
                stringify!(suffix) => {
                    meaning!(suffix, r, detail)
                }

                _ => todo!("{title} does not implemented yet"),
            }
        }

        if word.is_none() {
            return Err(WiktionaryError::TitleNotInclued);
        }

        Ok((word.unwrap(), r))
    }
}

#[cfg(test)]
mod tests {
    use etymora_traits::Word;

    #[test]
    #[ignore]
    fn test_parse_from_tsv_blue() -> Result<(), Box<dyn std::error::Error>> {
        // please put data on dict/wiktionary-blue on the repository root.
        // https://dbmx.net/dict/wiktionary-ja.tsv
        let data = include_str!("../../../dict/wiktionary-blue");
        let parsed = super::Entry::parse_from_tsv(data.into())?;

        assert_eq!(parsed.0, Word::from("blue"));
        assert_eq!(parsed.1.derivative, Some("bluish".into()));

        Ok(())
    }
}
