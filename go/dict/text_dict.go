package dict

import (
	_ "embed"
	"iter"
	"strings"
)

type TextDictionary string

var (
	//go:embed data/STCharacters.txt
	STCharactersText TextDictionary

	//go:embed data/STPhrases.txt
	STPhrasesText TextDictionary

	//go:embed data/TSCharacters.txt
	TSCharactersText TextDictionary

	//go:embed data/TSPhrases.txt
	TSPhrasesText TextDictionary

	//go:embed data/TWPhrasesIT.txt
	TWPhrasesITText TextDictionary

	//go:embed data/TWPhrasesName.txt
	TWPhrasesNameText TextDictionary

	//go:embed data/TWPhrasesOther.txt
	TWPhrasesOtherText TextDictionary

	//go:embed data/TWVariants.txt
	TWVariantsText TextDictionary

	//go:embed data/TWVariantsRevPhrases.txt
	TWVariantsRevPhrasesText TextDictionary

	//go:embed data/HKVariants.txt
	HKVariantsText TextDictionary

	//go:embed data/HKVariantsRevPhrases.txt
	HKVariantsRevPhrasesText TextDictionary

	//go:embed data/JPShinjitaiCharacters.txt
	JPShinjitaiCharactersText TextDictionary

	//go:embed data/JPShinjitaiPhrases.txt
	JPShinjitaiPhrasesText TextDictionary

	//go:embed data/JPVariants.txt
	JPVariantsText TextDictionary
)

func Parse(s string) iter.Seq[[]string] {
	return func(yield func([]string) bool) {
		for line := range strings.Lines(s) {
			if !yield(strings.Fields(line)) {
				return
			}
		}
	}
}

func (dict TextDictionary) Iter() iter.Seq2[string, string] {
	return func(yield func(string, string) bool) {
		for items := range Parse(string(dict)) {
			if len(items) >= 2 && !yield(items[0], items[1]) {
				return
			}
		}
	}
}

func (dict TextDictionary) InvIter() iter.Seq2[string, string] {
	return func(yield func(string, string) bool) {
		for items := range Parse(string(dict)) {
			if len(items) < 2 {
				continue
			}

			for _, item := range items[1:] {
				if !yield(item, items[0]) {
					return
				}
			}
		}
	}
}

func (dict TextDictionary) VarIter() iter.Seq2[string, []string] {
	return func(yield func(string, []string) bool) {
		for items := range Parse(string(dict)) {
			if len(items) >= 2 && !yield(items[0], items[1:]) {
				return
			}
		}
	}
}
