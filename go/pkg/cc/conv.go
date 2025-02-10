package cc

import (
	"github.com/fhluo/hanconv/go/trie"
)

type Converter struct {
	Name         string       `json:"name"`
	Dictionaries []*trie.Trie `json:"dictionaries"`
}

func New(name string, dictionaries ...*trie.Trie) *Converter {
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
