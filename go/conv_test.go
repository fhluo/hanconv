package hanconv

import (
	"github.com/fhluo/hanconv/go/trie"
	"testing"
)

var c = NewConverter("", trie.FromMap(map[string]string{
	"乾乾淨淨": "干干净净",
	"無序":   "无序",
	"書":    "书",
}))

func TestConverter_Convert(t *testing.T) {
	tests := []struct {
		s, expected string
	}{
		{"", ""},
		{"乾乾淨淨", "干干净净"},
		{"無序", "无序"},
		{"書", "书"},
		{"書，乾乾淨淨。", "书，干干净净。"},
		{"無序書乾乾淨淨", "无序书干干净净"},
		{"干干净净！", "干干净净！"},
	}

	for _, test := range tests {
		if r := c.Convert(test.s); r != test.expected {
			t.Errorf("c.Convert(%v), got %v, want %v", test.s, r, test.expected)
		}
	}
}
