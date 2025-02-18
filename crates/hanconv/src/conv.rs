use crate::trie::Trie;
use crate::Dictionary::*;
use std::sync::LazyLock;

pub enum Converters {
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

impl Converters {
    pub fn new(&self) -> Converter {
        Converter::new(self.dictionaries())
    }

    pub fn dictionaries(&self) -> Vec<Trie<&'static str>> {
        match self {
            Converters::S2T => vec![trie![STPhrases, STCharacters]],
            Converters::S2TW => vec![trie![STPhrases, STCharacters], trie![TWVariants]],
            Converters::S2TWP => vec![
                trie![STPhrases, STCharacters],
                trie![TWPhrases],
                trie![TWVariants],
            ],
            Converters::T2S => vec![trie![TSPhrases, TSCharacters]],
            Converters::T2TW => vec![trie![TWVariants]],
            Converters::TW2S => vec![
                trie![TWVariantsRevPhrases, TWVariantsRev],
                trie![TSPhrases, TSCharacters],
            ],
            Converters::TW2SP => vec![
                trie![TWPhrasesRev, TWVariantsRevPhrases, TWVariantsRev],
                trie![TSPhrases, TSCharacters],
            ],
            Converters::TW2T => vec![trie![TWVariantsRevPhrases, TWVariantsRev]],
            Converters::S2HK => vec![trie![STPhrases, STCharacters], trie![HKVariants]],
            Converters::HK2S => vec![
                trie![HKVariantsRevPhrases, HKVariantsRev],
                trie![TSPhrases, TSCharacters],
            ],
            Converters::HK2T => vec![trie![HKVariantsRevPhrases, HKVariantsRev]],
            Converters::T2HK => vec![trie![HKVariants]],
            Converters::T2JP => vec![trie![JPVariants]],
            Converters::JP2T => vec![trie![
                JPShinjitaiPhrases,
                JPShinjitaiCharacters,
                JPVariantsRev
            ]],
        }
    }
}

pub struct Converter(Vec<Trie<&'static str>>);

impl Converter {
    pub fn new(dictionaries: Vec<Trie<&'static str>>) -> Self {
        Self(dictionaries)
    }

    pub fn convert(&self, s: impl AsRef<str>) -> String {
        match self.0.len() {
            0 => s.as_ref().to_string(),
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

/// Simplified Chinese to Traditional Chinese
///
/// 简体中文 → 繁体中文
pub fn s2t(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Converters::S2T.new());
    CONVERTER.convert(s)
}

/// Traditional Chinese to Simplified Chinese
///
/// 繁体中文 → 简体中文
pub fn t2s(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Converters::T2S.new());
    CONVERTER.convert(s)
}

/// Simplified Chinese to Traditional Chinese (Taiwan)
///
/// 简体中文 → 繁体中文（台湾）
pub fn s2tw(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Converters::S2TW.new());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Taiwan) to Simplified Chinese
///
/// 繁体中文（台湾）→ 简体中文
pub fn tw2s(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Converters::TW2S.new());
    CONVERTER.convert(s)
}

/// Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom
///
/// 简体中文 → 繁体中文（台湾），转换为台湾常用词
pub fn s2twp(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Converters::S2TWP.new());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom
///
/// 繁体中文（台湾）→ 简体中文，转化为中国大陆常用词
pub fn tw2sp(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Converters::TW2SP.new());
    CONVERTER.convert(s)
}

/// Traditional Chinese to Traditional Chinese (Taiwan)
///
/// 繁体中文 → 繁体中文（台湾）
pub fn t2tw(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Converters::T2TW.new());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Taiwan) to Traditional Chinese
///
/// 繁体中文（台湾）→ 繁体中文
pub fn tw2t(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Converters::TW2T.new());
    CONVERTER.convert(s)
}

/// Simplified Chinese to Traditional Chinese (Hong Kong)
///
/// 简体中文 → 繁体中文（香港）
pub fn s2hk(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Converters::S2HK.new());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Hong Kong) to Simplified Chinese
///
/// 繁体中文（香港）→ 简体中文
pub fn hk2s(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Converters::HK2S.new());
    CONVERTER.convert(s)
}

/// Traditional Chinese to Traditional Chinese (Hong Kong)
///
/// 繁体中文 → 繁体中文（香港）
pub fn t2hk(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Converters::T2HK.new());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Hong Kong) to Traditional Chinese
///
/// 繁体中文（香港）→ 繁体中文
pub fn hk2t(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Converters::HK2T.new());
    CONVERTER.convert(s)
}

/// Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)
///
/// 繁体字 → 日文新字体
pub fn t2jp(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Converters::T2JP.new());
    CONVERTER.convert(s)
}

/// New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)
///
/// 日文新字体 → 繁体字
pub fn jp2t(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Converters::JP2T.new());
    CONVERTER.convert(s)
}
