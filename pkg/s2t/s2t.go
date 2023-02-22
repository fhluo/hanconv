package s2t

import (
	_ "embed"
	"github.com/bytedance/sonic"
	"github.com/fhluo/hanzi-conv/pkg/hanzi"
	"github.com/fhluo/hanzi-conv/pkg/trie"
)

//go:generate go run ../../cmd/hanzi gen ../../dictionary/STCharacters.txt ../../dictionary/STPhrases.txt -o s2t.json

var (
	//go:embed s2t.json
	data []byte
	Dict = trie.New()
	conv *hanzi.Converter
)

func init() {
	if err := sonic.Unmarshal(data, Dict); err != nil {
		panic(err)
	}
	conv = hanzi.NewConverter(Dict)
}

func Convert(data []byte) []byte {
	return conv.Convert(data)
}

func ConvertString(s string) string {
	return conv.ConvertString(s)
}
