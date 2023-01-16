package conv

import (
	"bytes"
	"github.com/fhluo/hanzi-conv/pkg/trie"
)

type Converter struct {
	*trie.Trie
}

func New() *Converter {
	return &Converter{
		Trie: trie.New(),
	}
}

// UpdateDict 更新 Converter 的字典
func (c *Converter) UpdateDict(dictionaries ...map[string]string) {
	for _, dict := range dictionaries {
		for k, v := range dict {
			c.Set(k, v)
		}
	}
}

// Convert 对字符串 s 进行转换
func (c *Converter) Convert(s string) string {
	runes := []rune(s)
	depth := c.Depth()

	buffer := new(bytes.Buffer)
	for len(runes) != 0 {
		value, count := c.Match(string(runes[:Min(depth, len(runes))]))

		if count == 0 {
			value = string(runes[:1])
			count = 1
		}

		buffer.WriteString(value)
		runes = runes[count:]
	}

	return buffer.String()
}
