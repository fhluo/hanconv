package t2s

import (
	_ "embed"
	"github.com/bytedance/sonic"
	"github.com/fhluo/gocc/pkg/cc"
)

var (
	//go:embed t2s.json
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
