package hanconv

import (
	"sync"

	"github.com/fhluo/hanconv/go/dict"
	"github.com/fhluo/hanconv/go/trie"
)

var (
	// S2TConverter converts Simplified Chinese to Traditional Chinese
	//
	// 简体中文 → 繁体中文
	S2TConverter = sync.OnceValue(func() *Converter {
		return NewConverter("s2t", trie.FromIters(dict.STPhrases(), dict.STCharacters()))
	})

	// T2SConverter converts Traditional Chinese to Simplified Chinese
	//
	// 繁体中文 → 简体中文
	T2SConverter = sync.OnceValue(func() *Converter {
		return NewConverter("t2s", trie.FromIters(dict.TSPhrases(), dict.TSCharacters()))
	})

	// S2TWConverter converts Simplified Chinese to Traditional Chinese (Taiwan)
	//
	// 简体中文 → 繁体中文（台湾）
	S2TWConverter = sync.OnceValue(func() *Converter {
		return NewConverter("s2tw", trie.FromIters(dict.STPhrases(), dict.STCharacters()),
			trie.FromIters(dict.TWVariants()))
	})

	// TW2SConverter converts Traditional Chinese (Taiwan) to Simplified Chinese
	//
	// 繁体中文（台湾）→ 简体中文
	TW2SConverter = sync.OnceValue(func() *Converter {
		return NewConverter("tw2s", trie.FromIters(dict.TWVariantsRevPhrases(), dict.TWVariantsRev()),
			trie.FromIters(dict.TSPhrases(), dict.TSCharacters()))
	})

	// S2TWPConverter converts Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom
	//
	// 简体中文 → 繁体中文（台湾），转换为台湾常用词
	S2TWPConverter = sync.OnceValue(func() *Converter {
		return NewConverter("s2twp", trie.FromIters(dict.STPhrases(), dict.STCharacters()), trie.FromIters(dict.TWPhrases()),
			trie.FromIters(dict.TWVariants()))
	})

	// TW2SPConverter converts Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom
	//
	// 繁体中文（台湾）→ 简体中文，转换为中国大陆常用词
	TW2SPConverter = sync.OnceValue(func() *Converter {
		return NewConverter("tw2sp", trie.FromIters(dict.TWPhrasesRev(), dict.TWVariantsRevPhrases(), dict.TWVariantsRev()),
			trie.FromIters(dict.TSPhrases(), dict.TSCharacters()))
	})

	// T2TWConverter converts Traditional Chinese to Traditional Chinese (Taiwan)
	//
	// 繁体中文 → 繁体中文（台湾）
	T2TWConverter = sync.OnceValue(func() *Converter {
		return NewConverter("t2tw", trie.FromIters(dict.TWVariants()))
	})

	// TW2TConverter converts Traditional Chinese (Taiwan) to Traditional Chinese
	//
	// 繁体中文（台湾）→ 繁体中文
	TW2TConverter = sync.OnceValue(func() *Converter {
		return NewConverter("tw2t", trie.FromIters(dict.TWVariantsRevPhrases(), dict.TWVariantsRev()))
	})

	// S2HKConverter converts Simplified Chinese to Traditional Chinese (Hong Kong)
	//
	// 简体中文 → 繁体中文（香港）
	S2HKConverter = sync.OnceValue(func() *Converter {
		return NewConverter("s2hk", trie.FromIters(dict.STPhrases(), dict.STCharacters()),
			trie.FromIters(dict.HKVariants()))
	})

	// HK2SConverter converts Traditional Chinese (Hong Kong) to Simplified Chinese
	//
	// 繁体中文（香港）→ 简体中文
	HK2SConverter = sync.OnceValue(func() *Converter {
		return NewConverter("hk2s", trie.FromIters(dict.HKVariantsRevPhrases(), dict.HKVariantsRev()),
			trie.FromIters(dict.TSPhrases(), dict.TSCharacters()))
	})

	// T2HKConverter converts Traditional Chinese to Traditional Chinese (Hong Kong)
	//
	// 繁体中文 → 繁体中文（香港）
	T2HKConverter = sync.OnceValue(func() *Converter {
		return NewConverter("t2hk", trie.FromIters(dict.HKVariants()))
	})

	// HK2TConverter converts Traditional Chinese (Hong Kong) to Traditional Chinese
	//
	// 繁体中文（香港）→ 繁体中文
	HK2TConverter = sync.OnceValue(func() *Converter {
		return NewConverter("hk2t", trie.FromIters(dict.HKVariantsRevPhrases(), dict.HKVariantsRev()))
	})

	// T2JPConverter converts Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)
	//
	// 繁体字 → 日文新字体
	T2JPConverter = sync.OnceValue(func() *Converter {
		return NewConverter("t2jp", trie.FromIters(dict.JPVariants()))
	})

	// JP2TConverter converts New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)
	//
	// 日文新字体 → 繁体字
	JP2TConverter = sync.OnceValue(func() *Converter {
		return NewConverter("jp2t", trie.FromIters(dict.JPShinjitaiPhrases(),
			dict.JPShinjitaiCharacters(),
			dict.JPVariantsRev()),
		)
	})
)

type Converter struct {
	Name         string       `json:"name"`
	Dictionaries []*trie.Trie `json:"dictionaries"`
}

func NewConverter(name string, dictionaries ...*trie.Trie) *Converter {
	return &Converter{
		Name:         name,
		Dictionaries: dictionaries,
	}
}

func (c *Converter) Convert(s string) string {
	if len(c.Dictionaries) == 0 {
		return s
	}

	for _, dictionary := range c.Dictionaries {
		s = dictionary.Convert(s)
	}

	return s
}

// S2T converts Simplified Chinese to Traditional Chinese
//
// 简体中文 → 繁体中文
func S2T(s string) string {
	return S2TConverter().Convert(s)
}

// T2S converts Traditional Chinese to Simplified Chinese
//
// 繁体中文 → 简体中文
func T2S(s string) string {
	return T2SConverter().Convert(s)
}

// S2TW converts Simplified Chinese to Traditional Chinese (Taiwan)
//
// 简体中文 → 繁体中文（台湾）
func S2TW(s string) string {
	return S2TWConverter().Convert(s)
}

// TW2S converts Traditional Chinese (Taiwan) to Simplified Chinese
//
// 繁体中文（台湾）→ 简体中文
func TW2S(s string) string {
	return TW2SConverter().Convert(s)
}

// S2TWP converts Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom
//
// 简体中文 → 繁体中文（台湾），转换为台湾常用词
func S2TWP(s string) string {
	return S2TWPConverter().Convert(s)
}

// TW2SP converts Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom
//
// 繁体中文（台湾）→ 简体中文，转换为中国大陆常用词
func TW2SP(s string) string {
	return TW2SPConverter().Convert(s)
}

// T2TW converts Traditional Chinese to Traditional Chinese (Taiwan)
//
// 繁体中文 → 繁体中文（台湾）
func T2TW(s string) string {
	return T2TWConverter().Convert(s)
}

// TW2T converts Traditional Chinese (Taiwan) to Traditional Chinese
//
// 繁体中文（台湾）→ 繁体中文
func TW2T(s string) string {
	return TW2TConverter().Convert(s)
}

// S2HK converts Simplified Chinese to Traditional Chinese (Hong Kong)
//
// 简体中文 → 繁体中文（香港）
func S2HK(s string) string {
	return S2HKConverter().Convert(s)
}

// HK2S converts Traditional Chinese (Hong Kong) to Simplified Chinese
//
// 繁体中文（香港）→ 简体中文
func HK2S(s string) string {
	return HK2SConverter().Convert(s)
}

// T2HK converts Traditional Chinese to Traditional Chinese (Hong Kong)
//
// 繁体中文 → 繁体中文（香港）
func T2HK(s string) string {
	return T2HKConverter().Convert(s)
}

// HK2T converts Traditional Chinese (Hong Kong) to Traditional Chinese
//
// 繁体中文（香港）→ 繁体中文
func HK2T(s string) string {
	return HK2TConverter().Convert(s)
}

// T2JP converts Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai)
//
// 繁体字 → 日文新字体
func T2JP(s string) string {
	return T2JPConverter().Convert(s)
}

// JP2T converts New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai)
//
// 日文新字体 → 繁体字
func JP2T(s string) string {
	return JP2TConverter().Convert(s)
}
