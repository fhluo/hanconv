package s2t

import (
	_ "embed"
	"github.com/bytedance/sonic"
	"github.com/fhluo/hanconv/pkg/hanconv"
	"github.com/fhluo/hanconv/pkg/trie"
)

//go:generate go run ../../cmd/hanconv gen ../../dictionary/STCharacters.txt ../../dictionary/STPhrases.txt -o s2t.json

var (
	//go:embed s2t.json
	data []byte
	Dict = trie.New()
	conv *hanconv.Converter
)

func init() {
	if err := sonic.Unmarshal(data, Dict); err != nil {
		panic(err)
	}
	conv = hanconv.New(Dict)
}

func Convert(data []byte) []byte {
	return conv.Convert(data)
}

func ConvertString(s string) string {
	return conv.ConvertString(s)
}
