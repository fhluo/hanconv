package dict

import (
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

func NewDictionary(rawDictionaries ...TextDictionary) Dictionary {
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

func NewInvDictionary(rawDictionaries ...TextDictionary) Dictionary {
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
	STCharacters          = NewDictionary(STCharactersText)
	STPhrases             = NewDictionary(STPhrasesText)
	TSCharacters          = NewDictionary(TSCharactersText)
	TSPhrases             = NewDictionary(TSPhrasesText)
	TWPhrases             = NewDictionary(TWPhrasesITText, TWPhrasesNameText, TWPhrasesOtherText)
	TWPhrasesRev          = NewInvDictionary(TWPhrasesITText, TWPhrasesNameText, TWPhrasesOtherText)
	TWVariants            = NewDictionary(TWVariantsText)
	TWVariantsRev         = NewInvDictionary(TWVariantsText)
	TWVariantsRevPhrases  = NewDictionary(TWVariantsRevPhrasesText)
	HKVariants            = NewDictionary(HKVariantsText)
	HKVariantsRev         = NewInvDictionary(HKVariantsText)
	HKVariantsRevPhrases  = NewDictionary(HKVariantsRevPhrasesText)
	JPShinjitaiCharacters = NewDictionary(JPShinjitaiCharactersText)
	JPShinjitaiPhrases    = NewDictionary(JPShinjitaiPhrasesText)
	JPVariants            = NewDictionary(JPVariantsText)
	JPVariantsRev         = NewInvDictionary(JPVariantsText)
)
