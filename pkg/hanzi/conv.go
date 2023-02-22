package hanzi

import (
	"bytes"
	"github.com/fhluo/hanzi-conv/pkg/trie"
)

type Converter struct {
	Dictionaries []*trie.Trie
}

func New(dictionaries ...*trie.Trie) *Converter {
	return &Converter{
		Dictionaries: dictionaries,
	}
}

func (c *Converter) Convert(s string) string {
	if len(c.Dictionaries) == 0 {
		return s
	}

	depth := c.Dictionaries[0].Depth
	for _, dict := range c.Dictionaries[1:] {
		if dict.Depth > depth {
			depth = dict.Depth
		}
	}

	buffer := new(bytes.Buffer)
	runes := []rune(s)
	for len(runes) != 0 {
		var (
			value string
			count int
		)

		for _, dict := range c.Dictionaries {
			value, count = dict.Match(string(runes[:Min(depth, len(runes))]))
			if count != 0 {
				break
			}
		}

		if count == 0 {
			value = string(runes[:1])
			count = 1
		}

		buffer.WriteString(value)
		runes = runes[count:]
	}

	return buffer.String()
}
