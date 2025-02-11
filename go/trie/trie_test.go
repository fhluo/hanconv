package trie

import (
	_ "embed"
	"testing"
)

var (
	trie *Trie
	data = map[string]string{
		"乾乾淨淨": "干干净净",
		"無序":   "无序",
		"書":    "书",
	}
)

func init() {
	trie = New()

	for k, v := range data {
		trie.Set(k, v)
	}
}

func TestTrie(t *testing.T) {
	tests := []struct {
		key, expected string
	}{
		{"", ""},
		{"乾乾淨淨", "干干净净"},
		{"乾乾", ""},
		{"乾乾淨淨?", ""},
		{"無序", "无序"},
		{"書", "书"},
	}

	for _, test := range tests {
		if r := trie.Get(test.key); r != test.expected {
			t.Errorf("trie.Get(%v), got %v, want %v", test.key, r, test.expected)
		}
	}
}

func TestTrie_Match(t *testing.T) {
	tests := []struct {
		s, value string
		count    int
	}{
		{"", "", 0},
		{"乾乾淨淨", "干干净净", 4},
		{"乾乾", "", 0},
		{"乾乾淨淨?", "干干净净", 4},
		{"無序", "无序", 2},
		{"書", "书", 1},
	}

	for _, test := range tests {
		if v, c := trie.Match([]rune(test.s)); v != test.value || c != test.count {
			t.Errorf("trie.Match(%v), got (%v, %v), want (%v, %v)", test.s, v, c, test.value, test.count)
		}
	}
}
