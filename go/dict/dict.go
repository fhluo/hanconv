package dict

import (
	_ "embed"
	"iter"
	"strings"
)

type RawDictionary string

var (
	//go:embed data/STCharacters.txt
	STCharacters RawDictionary

	//go:embed data/STPhrases.txt
	STPhrases RawDictionary

	//go:embed data/TSCharacters.txt
	TSCharacters RawDictionary

	//go:embed data/TSPhrases.txt
	TSPhrases RawDictionary

	//go:embed data/TWPhrasesIT.txt
	TWPhrasesIT RawDictionary

	//go:embed data/TWPhrasesName.txt
	TWPhrasesName RawDictionary

	//go:embed data/TWPhrasesOther.txt
	TWPhrasesOther RawDictionary

	//go:embed data/TWVariants.txt
	TWVariants RawDictionary

	//go:embed data/TWVariantsRevPhrases.txt
	TWVariantsRevPhrases RawDictionary

	//go:embed data/HKVariants.txt
	HKVariants RawDictionary

	//go:embed data/HKVariantsRevPhrases.txt
	HKVariantsRevPhrases RawDictionary

	//go:embed data/JPShinjitaiCharacters.txt
	JPShinjitaiCharacters RawDictionary

	//go:embed data/JPShinjitaiPhrases.txt
	JPShinjitaiPhrases RawDictionary

	//go:embed data/JPVariants.txt
	JPVariants RawDictionary
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

func (dict RawDictionary) Iter() iter.Seq2[string, string] {
	return func(yield func(string, string) bool) {
		for items := range Parse(string(dict)) {
			if len(items) >= 2 && !yield(items[0], items[1]) {
				return
			}
		}
	}
}

func (dict RawDictionary) InvIter() iter.Seq2[string, string] {
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

func (dict RawDictionary) VarIter() iter.Seq2[string, []string] {
	return func(yield func(string, []string) bool) {
		for items := range Parse(string(dict)) {
			if len(items) >= 2 && !yield(items[0], items[1:]) {
				return
			}
		}
	}
}
