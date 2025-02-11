use crate::{
    HKVariants, HKVariantsRev, HKVariantsRevPhrases, JPShinjitaiCharacters, JPShinjitaiPhrases,
    JPVariants, JPVariantsRev, STCharacters, STPhrases, TSCharacters, TSPhrases, TWPhrases,
    TWPhrasesRev, TWVariants, TWVariantsRev, TWVariantsRevPhrases,
};
use trie::Trie;

pub enum Convertors {
    S2T,
    S2TW,
    S2TWP,
    T2S,
    T2TW,
    TW2S,
    TW2SP,
    TW2T,
    S2HK,
    HK2S,
    HK2T,
    T2HK,
    T2JP,
    JP2T,
}

macro_rules! trie {
    [$a:expr $(,$b:expr)*] => {
        $a.iter()$(.chain($b.iter()))*.collect::<Trie<&'static str>>()
    };
}

impl Convertors {
    pub fn new(&self) -> Convertor {
        Convertor(match self {
            Convertors::S2T => vec![trie![STPhrases, STCharacters]],
            Convertors::S2TW => vec![trie![STPhrases, STCharacters], trie![TWVariants]],
            Convertors::S2TWP => vec![
                trie![STPhrases, STCharacters],
                trie![TWPhrases],
                trie![TWVariants],
            ],
            Convertors::T2S => vec![trie![TSPhrases, TSCharacters]],
            Convertors::T2TW => vec![trie![TWVariants]],
            Convertors::TW2S => vec![
                trie![TWVariantsRevPhrases, TWVariantsRev],
                trie![TSPhrases, TSCharacters],
            ],
            Convertors::TW2SP => vec![
                trie![TWPhrasesRev, TWVariantsRevPhrases, TWVariantsRev],
                trie![TSPhrases, TSCharacters],
            ],
            Convertors::TW2T => vec![trie![TWVariantsRevPhrases, TWVariantsRev]],
            Convertors::S2HK => vec![trie![STPhrases, STCharacters], trie![HKVariants]],
            Convertors::HK2S => vec![
                trie![HKVariantsRevPhrases, HKVariantsRev],
                trie![TSPhrases, TSCharacters],
            ],
            Convertors::HK2T => vec![trie![HKVariantsRevPhrases, HKVariantsRev]],
            Convertors::T2HK => vec![trie![HKVariants]],
            Convertors::T2JP => vec![trie![JPVariants]],
            Convertors::JP2T => vec![trie![
                JPShinjitaiPhrases,
                JPShinjitaiCharacters,
                JPVariantsRev
            ]],
        })
    }
}

pub struct Convertor(Vec<Trie<&'static str>>);

impl Convertor {
    pub fn convert(&self, s: &str) -> String {
        if let Some(trie) = self.0.first() {
            let mut next = trie.convert(s);

            for trie in self.0.iter().skip(1) {
                next = trie.convert(&next);
            }

            next
        } else {
            s.to_string()
        }
    }
}
