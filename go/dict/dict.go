package dict

import (
	"iter"
)

type Dictionary func() iter.Seq2[string, string]

func NewDictionary(dictionaries ...TextDictionary) Dictionary {
	return TextDictionaries(dictionaries).Iter
}

func NewInvDictionary(dictionaries ...TextDictionary) Dictionary {
	return TextDictionaries(dictionaries).InvIter
}

var (
	STCharacters          = NewDictionary(STCharactersText)
	STPhrases             = NewDictionary(STPhrasesText)
	TSCharacters          = NewDictionary(TSCharactersText)
	TSPhrases             = NewDictionary(TSPhrasesText)
	TWPhrases             = NewDictionary(TWPhrasesText)
	TWPhrasesRev          = NewInvDictionary(TWPhrasesRevText)
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
