package hanconv

import (
	"github.com/fhluo/hanconv/go/dict"
	"iter"
)

func chain(iters iter.Seq[iter.Seq2[string, string]]) iter.Seq2[string, string] {
	return func(yield func(string, string) bool) {
		for seq := range iters {
			seq(yield)
		}
	}
}

type Dictionary func() iter.Seq2[string, string]

func NewDictionary(rawDictionaries ...dict.RawDictionary) Dictionary {
	return func() iter.Seq2[string, string] {
		return chain(func(yield func(iter.Seq2[string, string]) bool) {
			for _, rawDictionary := range rawDictionaries {
				if !yield(rawDictionary.Iter()) {
					return
				}
			}
		})
	}
}

func NewInvDictionary(rawDictionaries ...dict.RawDictionary) Dictionary {
	return func() iter.Seq2[string, string] {
		return chain(func(yield func(iter.Seq2[string, string]) bool) {
			for _, rawDictionary := range rawDictionaries {
				if !yield(rawDictionary.InvIter()) {
					return
				}
			}
		})
	}
}

var (
	STCharacters          = NewDictionary(dict.STCharacters)
	STPhrases             = NewDictionary(dict.STPhrases)
	TSCharacters          = NewDictionary(dict.TSCharacters)
	TSPhrases             = NewDictionary(dict.TSPhrases)
	TWPhrases             = NewDictionary(dict.TWPhrasesIT, dict.TWPhrasesName, dict.TWPhrasesOther)
	TWPhrasesRev          = NewInvDictionary(dict.TWPhrasesIT, dict.TWPhrasesName, dict.TWPhrasesOther)
	TWVariants            = NewDictionary(dict.TWVariants)
	TWVariantsRev         = NewInvDictionary(dict.TWVariants)
	TWVariantsRevPhrases  = NewDictionary(dict.TWVariantsRevPhrases)
	HKVariants            = NewDictionary(dict.HKVariants)
	HKVariantsRev         = NewInvDictionary(dict.HKVariants)
	HKVariantsRevPhrases  = NewDictionary(dict.HKVariantsRevPhrases)
	JPShinjitaiCharacters = NewDictionary(dict.JPShinjitaiCharacters)
	JPShinjitaiPhrases    = NewDictionary(dict.JPShinjitaiPhrases)
	JPVariants            = NewDictionary(dict.JPVariants)
	JPVariantsRev         = NewInvDictionary(dict.JPVariants)
)
