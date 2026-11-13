use crate::trie::Trie;
use crate::Dictionary::*;
use std::sync::LazyLock;

pub enum Conversion {
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
    /// 繁体中文（台湾）→ 简体中文，转换为中国大陆常用词
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

static S2T_DICT: LazyLock<Trie<&'static str>> = LazyLock::new(|| trie![STPhrases, STCharacters]);
static T2S_DICT: LazyLock<Trie<&'static str>> = LazyLock::new(|| trie![TSPhrases, TSCharacters]);
static T2TW_VARIANTS_DICT: LazyLock<Trie<&'static str>> = LazyLock::new(|| trie![TWVariants]);
static T2TW_PHRASES_DICT: LazyLock<Trie<&'static str>> = LazyLock::new(|| trie![TWPhrases]);
static TW2T_VARIANTS_DICT: LazyLock<Trie<&'static str>> =
    LazyLock::new(|| trie![TWVariantsRevPhrases, TWVariantsRev]);
static TW2T_PHRASES_VARIANTS_DICT: LazyLock<Trie<&'static str>> =
    LazyLock::new(|| trie![TWPhrasesRev, TWVariantsRevPhrases, TWVariantsRev]);
static T2HK_VARIANTS_DICT: LazyLock<Trie<&'static str>> = LazyLock::new(|| trie![HKVariants]);
static HK2T_VARIANTS_DICT: LazyLock<Trie<&'static str>> =
    LazyLock::new(|| trie![HKVariantsRevPhrases, HKVariantsRev]);
static T2JP_VARIANTS_DICT: LazyLock<Trie<&'static str>> = LazyLock::new(|| trie![JPVariants]);
static JP2T_DICT: LazyLock<Trie<&'static str>> =
    LazyLock::new(|| trie![JPShinjitaiPhrases, JPShinjitaiCharacters, JPVariantsRev]);

impl Conversion {
    pub fn converter(&self) -> Converter {
        Converter::new(self.dictionaries())
    }

    pub fn dictionaries(&self) -> Vec<&'static Trie<&'static str>> {
        match self {
            Conversion::S2T => vec![&*S2T_DICT],
            Conversion::S2TW => vec![&*S2T_DICT, &*T2TW_VARIANTS_DICT],
            Conversion::S2TWP => vec![&*S2T_DICT, &*T2TW_PHRASES_DICT, &*T2TW_VARIANTS_DICT],
            Conversion::T2S => vec![&*T2S_DICT],
            Conversion::T2TW => vec![&*T2TW_VARIANTS_DICT],
            Conversion::TW2S => vec![&*TW2T_VARIANTS_DICT, &*T2S_DICT],
            Conversion::TW2SP => vec![&*TW2T_PHRASES_VARIANTS_DICT, &*T2S_DICT],
            Conversion::TW2T => vec![&*TW2T_VARIANTS_DICT],
            Conversion::S2HK => vec![&*S2T_DICT, &*T2HK_VARIANTS_DICT],
            Conversion::HK2S => vec![&*HK2T_VARIANTS_DICT, &*T2S_DICT],
            Conversion::HK2T => vec![&*HK2T_VARIANTS_DICT],
            Conversion::T2HK => vec![&*T2HK_VARIANTS_DICT],
            Conversion::T2JP => vec![&*T2JP_VARIANTS_DICT],
            Conversion::JP2T => vec![&*JP2T_DICT],
        }
    }
}

pub struct Converter(Vec<&'static Trie<&'static str>>);

impl Converter {
    pub fn new(dictionaries: Vec<&'static Trie<&'static str>>) -> Self {
        Self(dictionaries)
    }

    pub fn convert(&self, s: impl AsRef<str>) -> String {
        let Some((first, rest)) = self.0.split_first() else {
            return s.as_ref().to_string();
        };

        let mut s = first.convert(s);

        for trie in rest {
            s = trie.convert(&s);
        }

        s
    }
}

/// Simplified Chinese to Traditional Chinese
///
/// 简体中文 → 繁体中文
pub fn s2t(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Conversion::S2T.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese to Simplified Chinese
///
/// 繁体中文 → 简体中文
pub fn t2s(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Conversion::T2S.converter());
    CONVERTER.convert(s)
}

/// Simplified Chinese to Traditional Chinese (Taiwan)
///
/// 简体中文 → 繁体中文（台湾）
pub fn s2tw(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Conversion::S2TW.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Taiwan) to Simplified Chinese
///
/// 繁体中文（台湾）→ 简体中文
pub fn tw2s(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Conversion::TW2S.converter());
    CONVERTER.convert(s)
}

/// Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom
///
/// 简体中文 → 繁体中文（台湾），转换为台湾常用词
pub fn s2twp(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Conversion::S2TWP.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom
///
/// 繁体中文（台湾）→ 简体中文，转换为中国大陆常用词
pub fn tw2sp(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Conversion::TW2SP.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese to Traditional Chinese (Taiwan)
///
/// 繁体中文 → 繁体中文（台湾）
pub fn t2tw(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Conversion::T2TW.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Taiwan) to Traditional Chinese
///
/// 繁体中文（台湾）→ 繁体中文
pub fn tw2t(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Conversion::TW2T.converter());
    CONVERTER.convert(s)
}

/// Simplified Chinese to Traditional Chinese (Hong Kong)
///
/// 简体中文 → 繁体中文（香港）
pub fn s2hk(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Conversion::S2HK.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Hong Kong) to Simplified Chinese
///
/// 繁体中文（香港）→ 简体中文
pub fn hk2s(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Conversion::HK2S.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese to Traditional Chinese (Hong Kong)
///
/// 繁体中文 → 繁体中文（香港）
pub fn t2hk(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Conversion::T2HK.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Hong Kong) to Traditional Chinese
///
/// 繁体中文（香港）→ 繁体中文
pub fn hk2t(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Conversion::HK2T.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)
///
/// 繁体字 → 日文新字体
pub fn t2jp(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Conversion::T2JP.converter());
    CONVERTER.convert(s)
}

/// New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)
///
/// 日文新字体 → 繁体字
pub fn jp2t(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| Conversion::JP2T.converter());
    CONVERTER.convert(s)
}
