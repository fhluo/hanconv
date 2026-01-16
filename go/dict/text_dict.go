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

	//go:embed data/TWPhrases.txt
	TWPhrasesText TextDictionary

	//go:embed data/TWPhrasesRev.txt
	TWPhrasesRevText TextDictionary

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

type TextDictionaryIterator interface {
	Parse() iter.Seq[[]string]
	Iter() iter.Seq2[string, string]
	InvIter() iter.Seq2[string, string]
	VarIter() iter.Seq2[string, []string]
}

func (dict TextDictionary) Parse() iter.Seq[[]string] {
	return func(yield func([]string) bool) {
		lines := strings.Lines(string(dict))

		for line := range lines {
			if strings.HasPrefix(line, "#") || line == "" {
				continue
			}

			if !yield(strings.Fields(line)) {
				return
			}
			break
		}

		for line := range lines {
			if !yield(strings.Fields(line)) {
				return
			}
		}
	}
}

func (dict TextDictionary) Iter() iter.Seq2[string, string] {
	return func(yield func(string, string) bool) {
		for items := range dict.Parse() {
			if len(items) >= 2 && !yield(items[0], items[1]) {
				return
			}
		}
	}
}

func (dict TextDictionary) InvIter() iter.Seq2[string, string] {
	return func(yield func(string, string) bool) {
		for items := range dict.Parse() {
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
		for items := range dict.Parse() {
			if len(items) >= 2 && !yield(items[0], items[1:]) {
				return
			}
		}
	}
}

type TextDictionaries []TextDictionary

func (dictionaries TextDictionaries) Parse() iter.Seq[[]string] {
	return func(yield func([]string) bool) {
		for _, dictionary := range dictionaries {
			dictionary.Parse()(yield)
		}
	}
}

func (dictionaries TextDictionaries) Iter() iter.Seq2[string, string] {
	return func(yield func(string, string) bool) {
		for _, dictionary := range dictionaries {
			dictionary.Iter()(yield)
		}
	}
}

func (dictionaries TextDictionaries) InvIter() iter.Seq2[string, string] {
	return func(yield func(string, string) bool) {
		for _, dictionary := range dictionaries {
			dictionary.InvIter()(yield)
		}
	}
}

func (dictionaries TextDictionaries) VarIter() iter.Seq2[string, []string] {
	return func(yield func(string, []string) bool) {
		for _, dictionary := range dictionaries {
			dictionary.VarIter()(yield)
		}
	}
}
