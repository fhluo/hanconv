use crate::trie::Trie;
use crate::Dictionary::*;
use std::sync::LazyLock;

pub enum Convertors {
    /// Simplified Chinese to Traditional Chinese
    ///
    /// 简体中文 → 繁体中文
    S2T,
    /// Traditional Chinese to Simplified Chinese
    ///
    /// 繁体中文 → 简体中文
    T2S,
    /// Simplified Chinese to Traditional Chinese (Taiwan)
    ///
    /// 简体中文 → 繁体中文（台湾）
    S2TW,
    /// Traditional Chinese (Taiwan) to Simplified Chinese
    ///
    /// 繁体中文（台湾）→ 简体中文
    TW2S,
    /// Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom
    ///
    /// 简体中文 → 繁体中文（台湾），转换为台湾常用词
    S2TWP,
    /// Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom
    ///
    /// 繁体中文（台湾）→ 简体中文，转化为中国大陆常用词
    TW2SP,
    /// Traditional Chinese to Traditional Chinese (Taiwan)
    ///
    /// 繁体中文 → 繁体中文（台湾）
    T2TW,
    /// Traditional Chinese (Taiwan) to Traditional Chinese
    ///
    /// 繁体中文（台湾）→ 繁体中文
    TW2T,
    /// Simplified Chinese to Traditional Chinese (Hong Kong)
    ///
    /// 简体中文 → 繁体中文（香港）
    S2HK,
    /// Traditional Chinese (Hong Kong) to Simplified Chinese
    ///
    /// 繁体中文（香港）→ 简体中文
    HK2S,
    /// Traditional Chinese to Traditional Chinese (Hong Kong)
    ///
    /// 繁体中文 → 繁体中文（香港）
    T2HK,
    /// Traditional Chinese (Hong Kong) to Traditional Chinese
    ///
    /// 繁体中文（香港）→ 繁体中文
    HK2T,
    /// Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)
    ///
    /// 繁体字 → 日文新字体
    T2JP,
    /// New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)
    ///
    /// 日文新字体 → 繁体字
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
