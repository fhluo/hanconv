package trie

import (
	"bytes"
	"fmt"
	"iter"
	"unicode/utf8"
	"unsafe"
)

// Node 表示字典树的一个结点
type Node struct {
	Children map[rune]*Node `json:"c,omitempty"` // 子结点
	Value    string         `json:"v,omitempty"` // 值
	Exist    bool           `json:"e"`           // 以该结点结尾的字符串是否存在
}

func NewNode() *Node {
	return &Node{Children: make(map[rune]*Node)}
}

// Trie 字典树，键由结点的位置决定
type Trie struct {
	Root  *Node `json:"root"`  // 根结点
	Depth int   `json:"depth"` // 树的深度
}

func New() *Trie {
	return &Trie{
		Root: NewNode(),
	}
}

func FromMap(dictionaries ...map[string]string) *Trie {
	trie := New()
	for _, dict := range dictionaries {
		for k, v := range dict {
			trie.Set(k, v)
		}
	}
	return trie
}

func FromIter(dictionaries iter.Seq[iter.Seq2[string, string]]) *Trie {
	trie := New()
	for dict := range dictionaries {
		for k, v := range dict {
			trie.Set(k, v)
		}
	}
	return trie
}

func FromIters(dictionaries ...iter.Seq2[string, string]) *Trie {
	trie := New()
	for _, dict := range dictionaries {
		for k, v := range dict {
			trie.Set(k, v)
		}
	}
	return trie
}

func (t *Trie) String() string {
	return fmt.Sprint(t.ToMap())
}

func (t *Trie) StartsWith(s string) bool {
	node := t.Root

	for _, r := range s {
		if _, ok := node.Children[r]; !ok {
			return false
		}
		node = node.Children[r]
	}

	return true
}

func (t *Trie) Set(key, value string) {
	count := utf8.RuneCountInString(key)
	if count > t.Depth {
		t.Depth = count
	}

	node := t.Root

	// 迭代字符串 key 直到字符串末尾，子结点不存在则建立子结点
	for _, r := range key {
		if _, ok := node.Children[r]; !ok {
			node.Children[r] = NewNode()
		}
		node = node.Children[r]
	}

	node.Value = value
	node.Exist = true
}

func (t *Trie) Get(key string) string {
	node := t.Root

	// 迭代字符串 key 直到字符串末尾，子结点不存在则返回空字符串
	for _, r := range key {
		if _, ok := node.Children[r]; !ok {
			return ""
		}
		node = node.Children[r]
	}

	return node.Value
}

// Match 返回最大正向匹配键对应的值和键的长度(rune)，s 的最大长度不应超过树的深度
func (t *Trie) Match(runes []rune) (value string, count int) {
	node := t.Root

	// 迭代字符串 s 直到字符串末尾
	for i, r := range runes {
		if _, ok := node.Children[r]; !ok {
			break
		}

		// 若以当前字符结尾的字符串存在，则更新值为当前结点的值并更新键的长度
		node = node.Children[r]
		if node.Exist {
			value = node.Value
			count = i + 1
		}
	}

	return
}

func (t *Trie) Convert(s string) string {
	runes := []rune(s)

	buffer := bytes.NewBuffer(make([]byte, 0, len(s)))

	for len(runes) != 0 {
		value, count := t.Match(runes[:t.Depth])
		if count == 0 {
			buffer.WriteRune(runes[0])
			runes = runes[1:]
		} else {
			buffer.WriteString(value)
			runes = runes[count:]
		}
	}

	r := buffer.Bytes()
	return unsafe.String(unsafe.SliceData(r), len(r))
}

func build(node *Node, left string, dict map[string]string) {
	if node.Exist {
		dict[left] = node.Value
	}

	for k, v := range node.Children {
		build(v, left+string(k), dict)
	}
}

// ToMap 将 trie 转换为 map
func (t *Trie) ToMap() map[string]string {
	dict := make(map[string]string)
	build(t.Root, "", dict)
	return dict
}
