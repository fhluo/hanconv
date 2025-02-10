package hanconv

import (
	"github.com/fhluo/hanconv/go/trie"
	"sync"
)

var (
	S2T = sync.OnceValue(func() *Converter {
		return NewConverter("s2t", trie.FromIters(STPhrases(), STCharacters()))
	})
	S2TW = sync.OnceValue(func() *Converter {
		return NewConverter("s2tw", trie.FromIters(STPhrases(), STCharacters()),
			trie.FromIters(TWVariants()))
	})
	S2TWP = sync.OnceValue(func() *Converter {
		return NewConverter("s2twp", trie.FromIters(STPhrases(), STCharacters()), trie.FromIters(TWPhrases()),
			trie.FromIters(TWVariants()))
	})
	T2S = sync.OnceValue(func() *Converter {
		return NewConverter("t2s", trie.FromIters(TSPhrases(), TSCharacters()))
	})
	T2TW = sync.OnceValue(func() *Converter {
		return NewConverter("t2tw", trie.FromIters(TWVariants()))
	})
	TW2S = sync.OnceValue(func() *Converter {
		return NewConverter("tw2s", trie.FromIters(TWVariantsRevPhrases(), TWVariantsRev()),
			trie.FromIters(TSPhrases(), TSCharacters()))
	})
	TW2SP = sync.OnceValue(func() *Converter {
		return NewConverter("tw2sp", trie.FromIters(TWPhrasesRev(), TWVariantsRevPhrases(), TWVariantsRev()),
			trie.FromIters(TSPhrases(), TSCharacters()))
	})
	TW2T = sync.OnceValue(func() *Converter {
		return NewConverter("tw2t", trie.FromIters(TWVariantsRevPhrases(), TWVariantsRev()))
	})
	S2HK = sync.OnceValue(func() *Converter {
		return NewConverter("s2hk", trie.FromIters(STPhrases(), STCharacters()),
			trie.FromIters(HKVariants()))
	})
	HK2S = sync.OnceValue(func() *Converter {
		return NewConverter("hk2s", trie.FromIters(HKVariantsRevPhrases(), HKVariantsRev()),
			trie.FromIters(TSPhrases(), TSCharacters()))
	})
	HK2T = sync.OnceValue(func() *Converter {
		return NewConverter("hk2t", trie.FromIters(HKVariantsRevPhrases(), HKVariantsRev()))
	})
	T2HK = sync.OnceValue(func() *Converter {
		return NewConverter("t2hk", trie.FromIters(HKVariants()))
	})
	T2JP = sync.OnceValue(func() *Converter {
		return NewConverter("t2jp", trie.FromIters(JPVariants()))
	})
	JP2T = sync.OnceValue(func() *Converter {
		return NewConverter("jp2t", trie.FromIters(JPShinjitaiPhrases(),
			JPShinjitaiCharacters(),
			JPVariantsRev()),
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

	for _, dict := range c.Dictionaries {
		s = dict.Convert(s)
	}

	return s
}
