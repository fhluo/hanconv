package main

import (
	"github.com/fhluo/hanzi-conv/cmd"
	"github.com/fhluo/hanzi-conv/pkg/hanzi/conv"
	ts2 "github.com/fhluo/hanzi-conv/pkg/hanzi/dict/ts"
)

func main() {
	TSConv := conv.New()
	TSConv.UpdateDict(ts2.Characters, ts2.Phrases)
	cmd.Execute(TSConv, "t2s", "繁体中文 -> 简体中文")
}
