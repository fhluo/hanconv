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

func NewDictionary(rawDictionaries ...dict.TextDictionary) Dictionary {
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

func NewInvDictionary(rawDictionaries ...dict.TextDictionary) Dictionary {
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
	STCharacters          = NewDictionary(dict.STCharactersText)
	STPhrases             = NewDictionary(dict.STPhrasesText)
	TSCharacters          = NewDictionary(dict.TSCharactersText)
	TSPhrases             = NewDictionary(dict.TSPhrasesText)
	TWPhrases             = NewDictionary(dict.TWPhrasesITText, dict.TWPhrasesNameText, dict.TWPhrasesOtherText)
	TWPhrasesRev          = NewInvDictionary(dict.TWPhrasesITText, dict.TWPhrasesNameText, dict.TWPhrasesOtherText)
	TWVariants            = NewDictionary(dict.TWVariantsText)
	TWVariantsRev         = NewInvDictionary(dict.TWVariantsText)
	TWVariantsRevPhrases  = NewDictionary(dict.TWVariantsRevPhrasesText)
	HKVariants            = NewDictionary(dict.HKVariantsText)
	HKVariantsRev         = NewInvDictionary(dict.HKVariantsText)
	HKVariantsRevPhrases  = NewDictionary(dict.HKVariantsRevPhrasesText)
	JPShinjitaiCharacters = NewDictionary(dict.JPShinjitaiCharactersText)
	JPShinjitaiPhrases    = NewDictionary(dict.JPShinjitaiPhrasesText)
	JPVariants            = NewDictionary(dict.JPVariantsText)
	JPVariantsRev         = NewInvDictionary(dict.JPVariantsText)
)
