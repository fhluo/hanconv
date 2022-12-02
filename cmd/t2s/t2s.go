package main

import (
	"github.com/fhluo/hanzi-conv/cmd"
	"github.com/fhluo/hanzi-conv/conv"
	"github.com/fhluo/hanzi-conv/dict/ts"
)

func main() {
	TSConv := conv.New()
	TSConv.UpdateDict(ts.Characters, ts.Phrases)
	cmd.Execute(TSConv, "t2s", "繁体中文 -> 简体中文")
}
