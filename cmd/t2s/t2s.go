package main

import (
	"github.com/fhluo/zhconv/cmd"
	"github.com/fhluo/zhconv/conv"
	"github.com/fhluo/zhconv/dict/ts"
)

func main() {
	TSConv := conv.New()
	TSConv.UpdateDict(ts.Characters, ts.Phrases)
	cmd.Execute(TSConv, "t2s", "繁体中文 -> 简体中文")
}
