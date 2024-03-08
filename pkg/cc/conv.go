package cc

import (
	"bytes"
	"github.com/fhluo/gocc/pkg/trie"
	"unsafe"
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

func (c *Converter) Convert(data []byte) []byte {
	if len(c.Dictionaries) == 0 {
		return nil
	}

	depth := c.Dictionaries[0].Depth
	for _, dict := range c.Dictionaries[1:] {
		if dict.Depth > depth {
			depth = dict.Depth
		}
	}

	buffer := new(bytes.Buffer)

	runes := []rune(unsafe.String(unsafe.SliceData(data), len(data)))
	for len(runes) != 0 {
		var (
			value string
			count int
		)

		if len(runes) < depth {
			depth = len(runes)
		}

		for _, dict := range c.Dictionaries {
			value, count = dict.Match(string(runes[:depth]))
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

	return buffer.Bytes()
}

func (c *Converter) ConvertString(s string) string {
	r := c.Convert(unsafe.Slice(unsafe.StringData(s), len(s)))
	return unsafe.String(unsafe.SliceData(r), len(r))
}
