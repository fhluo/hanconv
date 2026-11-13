use crate::Dictionary::*;
use crate::trie::Trie;
use Conversion::*;
use std::fmt::{Display, Formatter};
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[non_exhaustive]
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
    /// Returns the conversion name.
    pub const fn name(self) -> &'static str {
        match self {
            S2T => "s2t",
            T2S => "t2s",
            S2TW => "s2tw",
            TW2S => "tw2s",
            S2TWP => "s2twp",
            TW2SP => "tw2sp",
            T2TW => "t2tw",
            TW2T => "tw2t",
            S2HK => "s2hk",
            HK2S => "hk2s",
            T2HK => "t2hk",
            HK2T => "hk2t",
            T2JP => "t2jp",
            JP2T => "jp2t",
        }
    }

    pub fn converter(&self) -> Converter {
        Converter::new(self.dictionaries())
    }

    pub fn dictionaries(&self) -> Vec<&'static Trie<&'static str>> {
        match self {
            S2T => vec![&*S2T_DICT],
            S2TW => vec![&*S2T_DICT, &*T2TW_VARIANTS_DICT],
            S2TWP => vec![&*S2T_DICT, &*T2TW_PHRASES_DICT, &*T2TW_VARIANTS_DICT],
            T2S => vec![&*T2S_DICT],
            T2TW => vec![&*T2TW_VARIANTS_DICT],
            TW2S => vec![&*TW2T_VARIANTS_DICT, &*T2S_DICT],
            TW2SP => vec![&*TW2T_PHRASES_VARIANTS_DICT, &*T2S_DICT],
            TW2T => vec![&*TW2T_VARIANTS_DICT],
            S2HK => vec![&*S2T_DICT, &*T2HK_VARIANTS_DICT],
            HK2S => vec![&*HK2T_VARIANTS_DICT, &*T2S_DICT],
            HK2T => vec![&*HK2T_VARIANTS_DICT],
            T2HK => vec![&*T2HK_VARIANTS_DICT],
            T2JP => vec![&*T2JP_VARIANTS_DICT],
            JP2T => vec![&*JP2T_DICT],
        }
    }
}

impl Display for Conversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
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
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| S2T.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese to Simplified Chinese
///
/// 繁体中文 → 简体中文
pub fn t2s(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| T2S.converter());
    CONVERTER.convert(s)
}

/// Simplified Chinese to Traditional Chinese (Taiwan)
///
/// 简体中文 → 繁体中文（台湾）
pub fn s2tw(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| S2TW.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Taiwan) to Simplified Chinese
///
/// 繁体中文（台湾）→ 简体中文
pub fn tw2s(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| TW2S.converter());
    CONVERTER.convert(s)
}

/// Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom
///
/// 简体中文 → 繁体中文（台湾），转换为台湾常用词
pub fn s2twp(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| S2TWP.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom
///
/// 繁体中文（台湾）→ 简体中文，转换为中国大陆常用词
pub fn tw2sp(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| TW2SP.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese to Traditional Chinese (Taiwan)
///
/// 繁体中文 → 繁体中文（台湾）
pub fn t2tw(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| T2TW.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Taiwan) to Traditional Chinese
///
/// 繁体中文（台湾）→ 繁体中文
pub fn tw2t(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| TW2T.converter());
    CONVERTER.convert(s)
}

/// Simplified Chinese to Traditional Chinese (Hong Kong)
///
/// 简体中文 → 繁体中文（香港）
pub fn s2hk(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| S2HK.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Hong Kong) to Simplified Chinese
///
/// 繁体中文（香港）→ 简体中文
pub fn hk2s(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| HK2S.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese to Traditional Chinese (Hong Kong)
///
/// 繁体中文 → 繁体中文（香港）
pub fn t2hk(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| T2HK.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese (Hong Kong) to Traditional Chinese
///
/// 繁体中文（香港）→ 繁体中文
pub fn hk2t(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| HK2T.converter());
    CONVERTER.convert(s)
}

/// Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)
///
/// 繁体字 → 日文新字体
pub fn t2jp(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| T2JP.converter());
    CONVERTER.convert(s)
}

/// New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)
///
/// 日文新字体 → 繁体字
pub fn jp2t(s: impl AsRef<str>) -> String {
    static CONVERTER: LazyLock<Converter> = LazyLock::new(|| JP2T.converter());
    CONVERTER.convert(s)
}
