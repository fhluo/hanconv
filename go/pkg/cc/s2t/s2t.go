package s2t

import (
	_ "embed"
	"github.com/bytedance/sonic"
	"github.com/fhluo/hanconv/go/pkg/cc"
)

var (
	//go:embed s2t.json
	data []byte
	conv cc.Converter
)

func init() {
	if err := sonic.Unmarshal(data, &conv); err != nil {
		panic(err)
	}
}

func Convert(data []byte) []byte {
	return conv.Convert(data)
}

func ConvertString(s string) string {
	return conv.ConvertString(s)
}
