package dict

import (
	_ "embed"
	"iter"
	"strings"
	"unicode"
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

type Variants string

func (v Variants) String() string { return string(v) }

func (v Variants) Iter() iter.Seq[string] {
	return strings.FieldsSeq(string(v))
}

type TextDictionaryIterator interface {
	Iter() iter.Seq2[string, string]
	InvIter() iter.Seq2[string, string]
	VarIter() iter.Seq2[string, Variants]
}

func (dict TextDictionary) iter(yield func(string, string) bool) {
	var (
		header = true
		fields [2]string
		i      int
	)

	for line := range strings.Lines(string(dict)) {
		if header {
			if strings.HasPrefix(line, "#") || line == "" {
				continue
			}
			header = false
		}

		i = 0
		for field := range strings.FieldsSeq(line) {
			if i < len(fields) {
				fields[i] = field
			} else {
				break
			}
			i++
		}

		if i < 2 {
			continue
		}

		if !yield(fields[0], fields[1]) {
			return
		}
	}
}

func (dict TextDictionary) invIter(yield func(string, string) bool) {
	var (
		header = true
		key    string
		hasKey bool
	)

	for line := range strings.Lines(string(dict)) {
		if header {
			if strings.HasPrefix(line, "#") || line == "" {
				continue
			}
			header = false
		}

		hasKey = false
		for field := range strings.FieldsSeq(line) {
			if !hasKey {
				key, hasKey = field, true
				continue
			}

			if !yield(field, key) {
				return
			}
		}
	}
}

func (dict TextDictionary) varIter(yield func(string, Variants) bool) {
	header := true

	for line := range strings.Lines(string(dict)) {
		if header {
			if strings.HasPrefix(line, "#") || line == "" {
				continue
			}
			header = false
		}

		s := strings.TrimLeftFunc(line, unicode.IsSpace)
		j := strings.IndexFunc(s, unicode.IsSpace)
		if j == -1 || !strings.ContainsFunc(s[j:], func(r rune) bool {
			return !unicode.IsSpace(r)
		}) {
			continue
		}

		if !yield(s[:j], Variants(s[j:])) {
			return
		}
	}
}

func (dict TextDictionary) Iter() iter.Seq2[string, string] {
	return dict.iter
}

func (dict TextDictionary) InvIter() iter.Seq2[string, string] {
	return dict.invIter
}

func (dict TextDictionary) VarIter() iter.Seq2[string, Variants] {
	return dict.varIter
}

type TextDictionaries []TextDictionary

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

func (dictionaries TextDictionaries) VarIter() iter.Seq2[string, Variants] {
	return func(yield func(string, Variants) bool) {
		for _, dictionary := range dictionaries {
			dictionary.VarIter()(yield)
		}
	}
}
