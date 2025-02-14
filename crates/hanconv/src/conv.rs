use crate::Dictionary::*;
use std::sync::LazyLock;
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
        Convertor::new(self.dictionaries())
    }

    pub fn dictionaries(&self) -> Vec<Trie<&'static str>> {
        match self {
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
        }
    }
}

pub struct Convertor(Vec<Trie<&'static str>>);

impl Convertor {
    pub fn new(dictionaries: Vec<Trie<&'static str>>) -> Self {
        Self(dictionaries)
    }

    pub fn convert(&self, s: &str) -> String {
        match self.0.len() {
            0 => s.to_string(),
            1 => self.0[0].convert(s),
            _ => {
                let mut s = self.0[0].convert(s);

                for trie in self.0.iter().skip(1) {
                    s = trie.convert(&s);
                }

                s
            }
        }
    }
}

macro_rules! f {
    {$($func_name:ident($convertor:ident))+} => {
        $(pub fn $func_name(s: &str) -> String {
            static CONVERTOR: LazyLock<Convertor> = LazyLock::new(|| Convertors::$convertor.new());
            CONVERTOR.convert(s)
        })+
    };
}

f! {
    s2t(S2T)
    s2tw(S2TW)
    s2twp(S2TWP)
    t2s(T2S)
    t2tw(T2TW)
    tw2s(TW2S)
    tw2sp(TW2SP)
    tw2t(TW2T)
    s2hk(S2HK)
    hk2s(HK2S)
    hk2t(HK2T)
    t2hk(T2HK)
    t2jp(T2JP)
    jp2t(JP2T)
}
